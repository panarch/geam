use super::EntryTarget;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{ExecutionFunctionRef, FunctionBodyOwner, FunctionExit};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::execution::{Evaluation, ServiceContext, Yield};
use crate::runtime::graph::RuntimeGraphState;
use crate::runtime::graph::{
    GraphExecution, GraphProgress, GraphStorage, GraphValue, RetainedValues,
};
use crate::runtime::state::RuntimeState;
use std::num::NonZeroUsize;

pub(in crate::runtime) struct Execution<'plan, Plan: ExecutableRuntimePlan, Id: EntryTarget<Plan>> {
    function: Id,
    position: Position<'plan, Plan, Id>,
    storage: Box<GraphStorage<'plan, Plan>>,
}

pub(in crate::runtime) enum Progress<
    'plan,
    Plan: ExecutableRuntimePlan + 'plan,
    Id: EntryTarget<Plan> + 'plan,
> {
    Continue(Execution<'plan, Plan, Id>),
    Host(Plan::HostInvocation<'plan, Progress<'plan, Plan, Id>>),
    Complete(EntryValue<Plan, Id>),
}

pub(in crate::runtime) type EntryValue<Plan, Id> =
    <<<Id as EntryTarget<Plan>>::Body as FunctionBodyOwner>::Return as GraphValue>::Evaluated;

enum Position<'plan, Plan: ExecutableRuntimePlan, Id: EntryTarget<Plan>> {
    Entry {
        origin: HostCallOrigin,
        inputs: RetainedValues,
    },
    Graph {
        body: &'plan Id::Body,
        execution: GraphExecution<'plan, Plan>,
    },
}

pub(super) fn run<'plan, Id>(
    plan: &'plan ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: Id,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EntryValue<ExecutionPlan, Id>>
where
    Id: EntryTarget<ExecutionPlan> + 'plan,
    Id::Body: 'plan,
    Id::HostTarget: 'plan,
{
    let mut progress = Progress::Continue(Execution::new(function, origin, inputs));
    loop {
        progress = match progress {
            Progress::Continue(execution) => execution.advance(plan, state, NonZeroUsize::MAX)?,
            Progress::Host(invoke) => match invoke {},
            Progress::Complete(value) => return Ok(value),
        };
    }
}

impl<'plan, Plan, Id> Execution<'plan, Plan, Id>
where
    Plan: ExecutableRuntimePlan,
    Id: EntryTarget<Plan> + 'plan,
    Id::Body: 'plan,
    Id::HostTarget: 'plan,
{
    pub(in crate::runtime) fn new(
        function: Id,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> Self {
        Self {
            function,
            position: Position::Entry { origin, inputs },
            storage: Box::new(GraphStorage::new()),
        }
    }

    pub(in crate::runtime) async fn drive(
        self,
        plan: &'plan Plan,
        context: &ServiceContext<Plan>,
        budget: NonZeroUsize,
    ) -> Result<ExecutionResult<EntryValue<Plan, Id>>, crate::runtime::work::Cancelled> {
        let mut evaluation = Evaluation::new(context.captures().clone());
        let mut progress = Ok(Progress::Continue(self));
        loop {
            progress = match progress {
                Ok(Progress::Continue(execution)) => {
                    let result = evaluation.access(|state| execution.advance(plan, state, budget));
                    let echo = evaluation.take_echo();
                    if !echo.is_empty() {
                        context
                            .submit(move |_, state| {
                                for output in echo {
                                    state.emit_echo(output);
                                }
                            })
                            .await?;
                    }
                    if matches!(result, Ok(Progress::Continue(_))) {
                        Yield::new().await;
                    }
                    result
                }
                Ok(Progress::Host(invoke)) => Plan::submit_host(invoke, context, budget).await?,
                Ok(Progress::Complete(value)) => return Ok(Ok(value)),
                Err(error) => return Ok(Err(error)),
            };
        }
    }

    pub(in crate::runtime) fn advance(
        mut self,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
        budget: NonZeroUsize,
    ) -> ExecutionResult<Progress<'plan, Plan, Id>> {
        let mut remaining = budget.get();
        while remaining > 0 {
            remaining -= 1;
            match self.advance_position(plan, state, &mut remaining)? {
                Progress::Continue(next) => self = next,
                host @ Progress::Host(_) => return Ok(host),
                complete @ Progress::Complete(_) => return Ok(complete),
            }
        }
        Ok(Progress::Continue(self))
    }

    fn advance_position(
        self,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
        remaining: &mut usize,
    ) -> ExecutionResult<Progress<'plan, Plan, Id>> {
        let Self {
            function,
            position,
            mut storage,
        } = self;
        match position {
            Position::Entry { origin, inputs } => {
                if let Some(cancelled) =
                    plan.reject_foreign_callable(&inputs, Some(state.captures().domain()))
                {
                    return Ok(Progress::Host(cancelled));
                }
                match function.entry(plan) {
                    ExecutionFunctionRef::Graph(entry) => {
                        let compiled = function.compiled(plan);
                        Ok(Progress::Continue(Self {
                            function,
                            storage,
                            position: Position::Graph {
                                body: entry.body(),
                                execution: GraphExecution::new(
                                    entry.body().function_body().block_graph().as_view(),
                                    inputs,
                                    compiled,
                                ),
                            },
                        }))
                    }
                    ExecutionFunctionRef::Host(target) => Ok(Progress::Host(Plan::map_host(
                        Id::prepare_host(plan, origin, target, inputs),
                        |value| Ok(Progress::Complete(value)),
                    ))),
                }
            }
            Position::Graph { body, execution } => {
                let progress = execution.advance(plan, state, &mut storage, remaining)?;
                Self::finish_graph(function, body, storage, plan, progress)
            }
        }
    }

    fn finish_graph(
        function: Id,
        body: &'plan Id::Body,
        mut storage: Box<GraphStorage<'plan, Plan>>,
        plan: &'plan Plan,
        progress: GraphProgress<'plan, Plan>,
    ) -> ExecutionResult<Progress<'plan, Plan, Id>> {
        match progress {
            GraphProgress::GeneratedHost { frame, invoke } => {
                Ok(Progress::Host(Plan::map_host(invoke, move |state| {
                    let progress =
                        GraphExecution::generated_ready(frame, state, plan, &mut storage)?;
                    Self::finish_graph(function, body, storage, plan, progress)
                })))
            }

            GraphProgress::Host(invoke) => {
                Ok(Progress::Host(Plan::map_host(invoke, move |execution| {
                    Ok(Progress::Continue(Self {
                        function,
                        position: Position::Graph { body, execution },
                        storage,
                    }))
                })))
            }
            GraphProgress::Continue(next) => Ok(Progress::Continue(Self {
                function,
                storage,
                position: Position::Graph {
                    body,
                    execution: next,
                },
            })),
            GraphProgress::CallComplete(output) => Ok(Progress::Complete(
                <Id::Body as FunctionBodyOwner>::Return::from_call_output(output)?,
            )),
            GraphProgress::CallInterpreted {
                target,
                point,
                values,
            } => {
                let function = Id::from_call_target(target)?;
                Ok(match function.entry(plan) {
                    ExecutionFunctionRef::Graph(entry) => Progress::Continue(Self {
                        function,
                        storage,
                        position: Position::Graph {
                            body: entry.body(),
                            execution: GraphExecution::interpreted(
                                entry.body().function_body().block_graph().as_view(),
                                point,
                                values,
                            ),
                        },
                    }),
                    ExecutionFunctionRef::Host(target) => Progress::Host(Plan::map_host(
                        Id::prepare_host(
                            plan,
                            HostCallOrigin::Entry,
                            target,
                            values.into_retained(),
                        ),
                        |value| Ok(Progress::Complete(value)),
                    )),
                })
            }
            GraphProgress::Complete(completed) => {
                Ok(match body.function_body().exit(completed.exit()) {
                    FunctionExit::Return(value) => Progress::Complete(completed.into_value(value)),
                    FunctionExit::TailCall {
                        function: target,
                        transfer,
                        ..
                    } => {
                        let (function, origin) = function.next(target);
                        Progress::Continue(Self {
                            function,
                            position: Position::Entry {
                                origin,
                                inputs: completed.into_retained(transfer),
                            },
                            storage,
                        })
                    }
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Execution, Progress};
    use crate::ExecutionPlan;
    use crate::plan::execution::function::IntFunctionId;
    use crate::runtime::error::HostCallOrigin;
    use crate::runtime::graph::RetainedValues;
    use crate::runtime::state::RuntimeState;
    use std::num::NonZeroUsize;

    #[test]
    fn generated_handoffs_reject_other_return_families_and_resume_the_real_source_graph() {
        use crate::plan::execution::compiled::{CallTarget, CompiledCheckpoint};
        use crate::plan::execution::function::{BoolFunctionId, FunctionReturnFamily};
        use crate::plan::execution::graph::BlockId;
        use crate::runtime::compiled::calls::{CallOutput, CallValues};
        use crate::runtime::graph::{GraphProgress, GraphStorage};
        use crate::runtime::{ExecutionError, InvariantError};

        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        let body = plan.int_function(IntFunctionId(0)).body();
        let point = CompiledCheckpoint {
            block: BlockId(0),
            instruction: 0,
            ints: 0,
            bools: 0,
            bit_arrays: 0,
            int_lists: 0,
            strings: 0,
            customs: 0,
            custom_lists: 0,
            int_functions: 0,
            bool_functions: 0,
        };
        for progress in [
            GraphProgress::CallComplete(CallOutput::Bool(true)),
            GraphProgress::CallInterpreted {
                target: CallTarget::Bool(BoolFunctionId(0)),
                point,
                values: Box::new(CallValues::default()),
            },
        ] {
            let failure = Execution::finish_graph(
                IntFunctionId(0),
                body,
                Box::new(GraphStorage::new()),
                &plan,
                progress,
            )
            .err()
            .unwrap();
            assert_eq!(
                failure,
                ExecutionError::Invariant(InvariantError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::Int,
                    actual: FunctionReturnFamily::Bool,
                })
            );
        }
        let progress = Execution::finish_graph(
            IntFunctionId(0),
            body,
            Box::new(GraphStorage::new()),
            &plan,
            GraphProgress::CallInterpreted {
                target: CallTarget::Int(IntFunctionId(0)),
                point,
                values: Box::new(CallValues::default()),
            },
        )
        .unwrap();
        let mut execution = continuing(progress);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        loop {
            match execution
                .advance(&plan, &mut state, NonZeroUsize::MIN)
                .unwrap()
            {
                Progress::Continue(next) => execution = next,
                Progress::Complete(value) => {
                    assert_eq!(value.into_bigint(), num_bigint::BigInt::from(42));
                    break;
                }
                Progress::Host(never) => match never {},
            }
        }
        assert!(echo.is_empty());
    }

    #[test]
    fn hosted_string_handoffs_keep_the_target_family_and_native_service_boundary() {
        use crate::plan::execution::compiled::{CallTarget, CompiledCheckpoint};
        use crate::plan::execution::function::{
            ExecutionFunctionEntry, ExecutionFunctionRef, FunctionReturnFamily, StringFunctionId,
        };
        use crate::plan::execution::graph::BlockId;
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        use crate::runtime::compiled::calls::{CallOutput, CallValues};
        use crate::runtime::graph::{GraphProgress, GraphStorage};
        use crate::runtime::{ExecutionError, InvariantError};
        use crate::{HostProviderModule, HostProviderSet, StatelessHostProfile, StringValue};
        let source = "@external(erlang, \"example\", \"answer\") fn answer() -> String pub fn main() { let _ = answer() \"kept\" }";
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::from_providers([HostProviderModule::<StatelessHostProfile>::new(
                "example", "example",
            )
            .unwrap()
            .with_function::<(), StringValue, _>("answer", || StringValue::from("native"))
            .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let plan = &**plan;
        let mut graphs = (0..2)
            .filter_map(
                |index| match plan.string_function(StringFunctionId(index)).as_ref() {
                    ExecutionFunctionRef::Graph(entry) => Some(entry.body()),
                    ExecutionFunctionRef::Host(_) => None,
                },
            )
            .collect::<Vec<_>>();
        assert_eq!(graphs.len(), 1);
        let body = graphs.pop().unwrap();
        let point = CompiledCheckpoint {
            block: BlockId(0),
            instruction: 0,
            ints: 0,
            bools: 0,
            bit_arrays: 0,
            int_lists: 0,
            strings: 0,
            customs: 0,
            custom_lists: 0,
            int_functions: 0,
            bool_functions: 0,
        };
        for progress in [
            GraphProgress::CallComplete(CallOutput::Int(42_i128.into())),
            GraphProgress::CallInterpreted {
                target: CallTarget::Int(IntFunctionId(0)),
                point,
                values: Box::default(),
            },
        ] {
            assert_eq!(
                Execution::finish_graph(
                    StringFunctionId(0),
                    body,
                    Box::new(GraphStorage::new()),
                    plan,
                    progress
                )
                .err()
                .unwrap(),
                ExecutionError::Invariant(InvariantError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::String,
                    actual: FunctionReturnFamily::Int
                })
            );
        }
        for (target, native) in [(StringFunctionId(0), false), (StringFunctionId(1), true)] {
            let progress = Execution::finish_graph(
                StringFunctionId(0),
                body,
                Box::new(GraphStorage::new()),
                plan,
                GraphProgress::CallInterpreted {
                    target: CallTarget::String(target),
                    point,
                    values: Box::<CallValues>::default(),
                },
            )
            .unwrap();
            assert_eq!(matches!(progress, Progress::Host(_)), native);
            assert_eq!(matches!(progress, Progress::Continue(_)), !native);
        }
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::String("kept".into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn generated_string_native_results_resume_the_original_function_driver() {
        use super::Position;
        use crate::plan::execution::function::{
            ExecutionFunctionEntry, ExecutionFunctionRef, StringFunctionId,
        };
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        use crate::plan::execution::{
            HostedProgram,
            compiled::{
                CallTarget, CompiledCheckpoint, CompiledImplementation, FunctionCallsImplementation,
            },
        };
        use crate::plan::{HostCallSite, SourceSpan};
        use crate::runtime::compiled::calls::{
            CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage,
            StringNativeExecution, StringNativeRequest,
        };
        use crate::runtime::graph::{
            BlockEnvironment, GraphExecution, GraphStorage, RetainedValues,
        };
        use crate::{
            HostCall, HostCallContinuation, HostCallError, HostConstructions, HostOwnedCompletion,
            HostProvider, HostProviderModule, HostProviderSet, HostTypeListEnd,
            StatelessHostProfile, StringValue,
        };
        use std::sync::{
            Arc, Mutex, Weak,
            atomic::{AtomicUsize, Ordering},
        };
        const SOURCE: &str = r#"
@external(erlang, "example", "answer")
fn answer() -> String
@external(erlang, "example", "exercise")
fn exercise() -> Nil
fn source() { let _ = answer() "kept" }
pub fn main() { exercise() source() }
"#;
        type PlanSlot = Arc<Mutex<Option<Weak<HostedProgram<StatelessHostProfile>>>>>;
        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        struct Completion {
            requested: bool,
        }
        impl CallExecution for Completion {
            fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
                false
            }
            fn retained_bytes(&self) -> usize {
                0
            }
            fn advance(
                mut self: Box<Self>,
                _: &mut CallOps<'_>,
                budget: &mut usize,
            ) -> CallProgress {
                if *budget == 0 {
                    return CallProgress::Yield(self);
                }
                *budget -= 1;
                if !self.requested {
                    self.requested = true;
                    CallProgress::StringNative(StringNativeRequest {
                        function: StringFunctionId(2),
                        site: HostCallSite::from_static(
                            "example",
                            "source",
                            SourceSpan::new(0, SOURCE.len()),
                        ),
                        root_tail: false,
                        arguments: Box::default(),
                        execution: self,
                    })
                } else {
                    CallProgress::Complete {
                        output: CallOutput::String("kept".into()),
                        execution: self,
                    }
                }
            }
        }
        impl StringNativeExecution for Completion {
            fn resume_native(self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> {
                assert_eq!(value.as_str(), Ok("native"));
                self
            }
        }
        fn start(
            _: usize,
            _: CallInputs<'_>,
            _: &mut CallStorage,
        ) -> Option<Box<dyn CallExecution>> {
            Some(Box::new(Completion { requested: false }))
        }
        fn exercise<'call>(
            slot: &PlanSlot,
            mut call: HostCall<'call, StatelessHostProfile, Provider, ()>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
            let _ = call.state();
            let plan = slot.lock().unwrap().as_ref().unwrap().upgrade().unwrap();
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let mut graphs = (0..3)
                        .filter_map(|index| {
                            match plan.string_function(StringFunctionId(index)).as_ref() {
                                ExecutionFunctionRef::Graph(entry) if index == 1 => {
                                    Some(entry.body())
                                }
                                _ => None,
                            }
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(graphs.len(), 1);
                    let body = graphs.pop().unwrap();
                    let graph = body.block_graph().as_view();
                    let point = CompiledCheckpoint {
                        block: graph.entry(),
                        instruction: 0,
                        ints: 0,
                        bools: 0,
                        bit_arrays: 0,
                        int_lists: 0,
                        strings: 0,
                        customs: 0,
                        custom_lists: 0,
                        int_functions: 0,
                        bool_functions: 0,
                    };
                    // This owner supplies published protocol boundaries. Public
                    // prepared fixtures prove generator selection and exact charges.
                    let implementation = CompiledImplementation::FunctionCalls(
                        Box::new(FunctionCallsImplementation {
                            root: true,
                            entry: 0,
                            checkpoints: vec![point].into(),
                            locals: Vec::new().into(),
                            calls: Vec::new().into(),
                            creations: Vec::new().into(),
                            returns: Vec::new().into(),
                            tails: Vec::new().into(),
                            start,
                        })
                        .into(),
                    );
                    for budget in [NonZeroUsize::MIN, NonZeroUsize::new(16).unwrap()] {
                        let execution = Execution {
                            function: StringFunctionId(1),
                            storage: Box::new(GraphStorage::new()),
                            position: Position::Graph {
                                body,
                                execution: GraphExecution::new(
                                    graph,
                                    RetainedValues::empty(),
                                    Some(&implementation),
                                ),
                            },
                        };
                        let value = execution
                            .drive(&*plan, context.execution().services(), budget)
                            .await
                            .unwrap()
                            .unwrap();
                        assert_eq!(value.as_str(), Ok("kept"));
                    }
                    Ok(HostOwnedCompletion::new(
                        |call, _| Ok(call.return_value(())),
                    ))
                })
            }))
        }
        let empty = BlockEnvironment::from_retained(RetainedValues::empty());
        let mut engine = Completion { requested: false };
        assert!(!engine.restart(
            CallTarget::String(StringFunctionId(1)),
            0,
            CallInputs::new(&empty)
        ));
        assert_eq!(engine.retained_bytes(), 0);
        let mut numeric = Default::default();
        let mut strings = None;
        let mut bits = None;
        let mut probe_echo = Vec::new();
        let runtime = RuntimeState::new(&mut probe_echo);
        for offered in [0, 1] {
            let mut remaining = offered;
            let progress = Box::new(Completion { requested: false }).advance(
                &mut CallOps::new(
                    runtime.captures(),
                    &mut numeric,
                    runtime.lists(),
                    &mut strings,
                    &mut bits,
                ),
                &mut remaining,
            );
            assert_eq!(matches!(progress, CallProgress::Yield(_)), offered == 0);
            assert_eq!(remaining, 0);
        }
        let slot: PlanSlot = Arc::new(Mutex::new(None));
        let observed_slot = slot.clone();
        type ExerciseNative = dyn for<'call> Fn(
                HostCall<'call, StatelessHostProfile, Provider, ()>,
                HostConstructions<'call, HostTypeListEnd>,
            ) -> Result<HostCallContinuation<'call, ()>, HostCallError>
            + Send
            + Sync;
        let exercise_native: Box<ExerciseNative> =
            Box::new(move |call, constructions| exercise(&observed_slot, call, constructions));
        let calls = Arc::new(AtomicUsize::new(0));
        let observed_calls = calls.clone();
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    SOURCE,
                )],
            )],
            HostProviderSet::from_providers([HostProviderModule::<StatelessHostProfile>::new(
                "example", "example",
            )
            .unwrap()
            .with_function::<(), StringValue, _>("answer", move || {
                observed_calls.fetch_add(1, Ordering::SeqCst);
                StringValue::from("native")
            })
            .unwrap()
            .with_resumable_function::<Provider, (), (), HostTypeListEnd, _>(
                "exercise",
                exercise_native,
            )
            .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        {
            let (plan, _, _) = hosted.parts_mut();
            *slot.lock().unwrap() = Some(Arc::downgrade(plan));
        }
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::String("kept".into())
        );
        assert_eq!(calls.load(Ordering::SeqCst), 3);
        assert!(echo.is_empty());
    }

    #[test]
    fn finite_entry_completes_with_unused_budget() {
        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        let execution = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let result = execution.advance(&plan, &mut state, NonZeroUsize::new(100).unwrap());
        assert!(
            matches!(result, Ok(Progress::Complete(value)) if value == num_bigint::BigInt::from(42))
        );
    }

    #[test]
    fn pure_self_loops_yield_at_root_and_nested_entries_without_effects() {
        for source in [
            "pub fn main() -> Int { main() }",
            "fn spin() -> Int { spin() } pub fn main() { spin() + 1 }",
            "pub fn main() -> Int { case 1 < 2 { True -> main() False -> 0 } }",
            "fn spin(value: Int) { case value >= 0 { True -> spin(value) False -> value } } pub fn main() { spin(1) + 1 }",
        ] {
            let plan = crate::runtime::plan_src(source);
            for budget in [1, 2, 7, 1024] {
                let mut execution = Execution::new(
                    IntFunctionId(0),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                );
                let mut echo = Vec::new();
                for _ in 0..8 {
                    execution = continuing(
                        execution
                            .advance(
                                &plan,
                                &mut RuntimeState::new(&mut echo),
                                NonZeroUsize::new(budget).unwrap(),
                            )
                            .unwrap(),
                    );
                }
                drop(execution);
                assert!(echo.is_empty());
            }
        }
    }

    #[test]
    fn bounded_calls_preserve_exact_echo_and_completion_turns() {
        let plan = crate::runtime::plan_src(
            r#"
const adjustment = 2

fn apply(value, callback) { callback(value) + adjustment }

fn walk(remaining, total, callback) {
  case remaining {
    0 -> total
    n -> {
      let value = apply(n, callback)
      walk(n - 1, total + value, callback)
    }
  }
}

pub fn main() {
  let offset = 5
  walk(12, 0, fn(value) {
    echo value
    value + offset
  })
}
"#,
        );
        // Twelve two-operation tail argument regions each replace two scalar
        // instructions with one sealed instruction: 167 charged steps. The
        // first echo is step12 and subsequent echoes are thirteen steps apart.
        let echo_steps = [12, 25, 38, 51, 64, 77, 90, 103, 116, 129, 142, 155];
        for budget in [1, 2, 3, 7, 29, 128, 167, 1024, usize::MAX] {
            let turns = 167_usize.div_ceil(budget);
            let echo_turns = echo_steps.map(|step: usize| step.div_ceil(budget));
            let trace = trace_int_turns(&plan, NonZeroUsize::new(budget).unwrap());
            assert_eq!(trace.turns, turns, "budget {budget}");
            assert_eq!(trace.result, Ok(162.into()), "budget {budget}");
            assert_eq!(
                trace.echo,
                echo_turns
                    .into_iter()
                    .zip([
                        "12", "11", "10", "9", "8", "7", "6", "5", "4", "3", "2", "1"
                    ])
                    .map(|(turn, value)| (turn, value.to_owned()))
                    .collect::<Vec<_>>(),
                "budget {budget}",
            );
        }
    }

    #[test]
    fn a_source_panic_preserves_its_exact_turn_echo_and_origin() {
        let plan = crate::runtime::plan_src(
            r#"
fn walk(remaining) {
  echo remaining
  case remaining {
    0 -> panic as "probe stop"
    n -> walk(n - 1)
  }
}

pub fn main() { walk(5) + 1 }
"#,
        );
        // Five literal loads disappear before the panic. Echo steps are
        // 4, 8, 12, 16, 20, 24 and the source stop is reached at step 27.
        for (budget, turns, echo_turns) in [
            (1, 27, [4, 8, 12, 16, 20, 24]),
            (2, 14, [2, 4, 6, 8, 10, 12]),
            (3, 9, [2, 3, 4, 6, 7, 8]),
            (7, 4, [1, 2, 2, 3, 3, 4]),
            (27, 1, [1, 1, 1, 1, 1, 1]),
            (29, 1, [1, 1, 1, 1, 1, 1]),
            (128, 1, [1, 1, 1, 1, 1, 1]),
            (1024, 1, [1, 1, 1, 1, 1, 1]),
            (usize::MAX, 1, [1; 6]),
        ] {
            let trace = trace_int_turns(&plan, NonZeroUsize::new(budget).unwrap());
            assert_eq!(trace.turns, turns, "budget {budget}");
            assert_eq!(
                trace.echo,
                echo_turns
                    .into_iter()
                    .zip(["5", "4", "3", "2", "1", "0"])
                    .map(|(turn, value)| (turn, value.to_owned()))
                    .collect::<Vec<_>>(),
                "budget {budget}",
            );
            let error = trace.result.unwrap_err().into_materialized();
            assert_eq!(
                format!("{error:?}"),
                r#"Panic(Panic { kind: Panic, message: Explicit("probe stop"), site: PanicSite { module: "main", function: "walk", span: SourceSpan { start: 67, end: 88 } }, source: None, details: None })"#,
            );
        }
    }

    #[test]
    fn tail_recursive_entries_share_bounded_turns_and_release_independently() {
        let plan = crate::runtime::plan_src(
            r#"
fn count(value: Int) -> Int {
  echo value
  count(value + 1)
}
pub fn main() { count(0) }
"#,
        );
        let mut left = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let mut right = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let left_storage = std::ptr::from_ref(left.storage.as_ref());
        let right_storage = std::ptr::from_ref(right.storage.as_ref());
        let budget = NonZeroUsize::new(29).unwrap();
        let mut left_echo = Vec::new();
        let mut right_echo = Vec::new();
        for _ in 0..100 {
            let mut state = RuntimeState::new(&mut left_echo);
            left = continuing(left.advance(&plan, &mut state, budget).expect("left turn"));
            let mut state = RuntimeState::new(&mut right_echo);
            right = continuing(
                right
                    .advance(&plan, &mut state, budget)
                    .expect("right turn"),
            );
        }
        assert_eq!(std::ptr::from_ref(left.storage.as_ref()), left_storage);
        assert_eq!(std::ptr::from_ref(right.storage.as_ref()), right_storage);
        assert!(left_echo.len() > 100);
        assert_eq!(left_echo.len(), right_echo.len());
        for (index, output) in left_echo.iter().enumerate() {
            assert_eq!(output.value().inspect().to_string(), index.to_string());
        }
        drop(left);
        let previous = right_echo.len();
        let mut state = RuntimeState::new(&mut right_echo);
        let progress = right
            .advance(&plan, &mut state, budget)
            .expect("surviving turn");
        drop(continuing(progress));
        assert!(right_echo.len() > previous);
    }

    #[test]
    fn graph_storage_survives_root_tail_calls_nested_calls_and_independent_yields() {
        let plan = crate::runtime::plan_src(
            r#"
fn ordinary(value, depth) {
  let assert <<first, rest:bits>> = value
  case depth {
    0 -> first
    _ -> ordinary(value, depth - 1) + first
  }
}
fn left(values: List(Int), count: Int) -> Int {
  let assert [head, ..tail] = values
  echo ordinary(<<head, 2>>, 8)
  right([head, ..tail], count + 1)
}
fn right(values: List(Int), count: Int) -> Int {
  let assert [head, ..tail] = values
  echo count
  left([head, ..tail], count)
}
pub fn main() { left([1, 2, 3], 0) }
"#,
        );
        let mut first = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let mut second = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let mut first_echo = Vec::new();
        let mut second_echo = Vec::new();
        // Enter both source executions with a fixed number of one-step turns.
        for _ in 0..32 {
            first = continuing(
                first
                    .advance(
                        &plan,
                        &mut RuntimeState::new(&mut first_echo),
                        NonZeroUsize::MIN,
                    )
                    .unwrap(),
            );
            second = continuing(
                second
                    .advance(
                        &plan,
                        &mut RuntimeState::new(&mut second_echo),
                        NonZeroUsize::MIN,
                    )
                    .unwrap(),
            );
        }
        let first_buffer = &*first.storage as *const _;
        let second_buffer = &*second.storage as *const _;
        assert_ne!(first_buffer, second_buffer);
        for _ in 0..4_000 {
            first = continuing(
                first
                    .advance(
                        &plan,
                        &mut RuntimeState::new(&mut first_echo),
                        NonZeroUsize::MIN,
                    )
                    .unwrap(),
            );
            second = continuing(
                second
                    .advance(
                        &plan,
                        &mut RuntimeState::new(&mut second_echo),
                        NonZeroUsize::MIN,
                    )
                    .unwrap(),
            );

            assert_eq!(&*first.storage as *const _, first_buffer);
            assert_eq!(&*second.storage as *const _, second_buffer);
        }
        assert!(first_echo.len() > 10);
        assert_eq!(first_echo.len(), second_echo.len());
        for (index, pair) in first_echo.chunks_exact(2).enumerate() {
            assert_eq!(pair[0].value().inspect().to_string(), "9");
            assert_eq!(
                pair[1].value().inspect().to_string(),
                (index + 1).to_string()
            );
        }
        drop(first);
        second = continuing(
            second
                .advance(
                    &plan,
                    &mut RuntimeState::new(&mut second_echo),
                    NonZeroUsize::MIN,
                )
                .unwrap(),
        );
        assert_eq!(&*second.storage as *const _, second_buffer);
    }

    #[test]
    #[should_panic(expected = "fixture entry must still be running")]
    fn continuing_guard_rejects_a_completed_source_entry() {
        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        let execution = Execution::new(
            IntFunctionId(0),
            HostCallOrigin::Entry,
            RetainedValues::empty(),
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        continuing(
            execution
                .advance(&plan, &mut state, NonZeroUsize::MAX)
                .unwrap(),
        );
    }

    #[test]
    fn connected_custom_loops_preserve_exact_turns_echoes_and_stops() {
        use crate::__prepared_support as data;
        use crate::embedding::{BigInt, FunctionDeclaration, ModuleBuilder};
        use std::convert::Infallible;

        // Both execution owners admit the actual generated artifact. Only its
        // compiled sidecar is disabled for the canonical control; no graph or
        // checkpoint is fabricated or changed.
        const ARTIFACT: data::ModuleArtifact<Infallible> =
            include!("../../../tests/fixtures/prepared/custom_loop_boundaries.rs");
        static COMPILED: data::ModuleArtifact<Infallible> = ARTIFACT;
        let source = r#"type Item {
  Item(Int)
}

fn walk(
  items: List(Item),
  total: Int,
  fail: Bool,
  apply: fn(Int, Item) -> Int,
) {
  case items {
    [] ->
      case fail {
        True -> panic
        False -> {
          echo total
          let assert [result] = [total]
          result
        }
      }
    [head, ..tail] -> walk(tail, apply(total + 1, head), fail, apply)
  }
}

fn add(total: Int, item: Item) {
  let Item(value) = item
  total + value
}

fn relay(total: Int, item: Item) {
  add(total, item)
}

pub fn integer(count: Int, fail: Bool, connected: Bool, initial: Int) {
  let apply = case connected {
    True -> add
    False -> relay
  }
  walk(items(count, []), initial, fail, apply)
}

fn any(items: List(Item), seen: Bool, apply: fn(Bool, Item) -> Bool) {
  case items {
    [] -> seen
    [head, ..tail] -> any(tail, apply(seen, head), apply)
  }
}

pub fn boolean(bias: Int) {
  any([Item(1)], False, fn(_seen, item) {
    let Item(value) = item
    value + bias > 0
  })
}

pub fn main() {
  integer(1, False, True, 3)
}

fn items(count: Int, result: List(Item)) {
  case count {
    0 -> result
    _ -> items(count - 1, [Item(2), ..result])
  }
}
"#;
        let typed =
            crate::compile_typed_module("example", "src/custom_loop_boundaries.gleam", source)
                .unwrap();
        let (mut bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt, bool, bool, BigInt), BigInt>::new("integer"))
            .unwrap();
        bindings
            .function(FunctionDeclaration::<(BigInt,), bool>::new("boolean"))
            .unwrap();
        assert_eq!(
            bindings.prepare().emit_rust(),
            include_str!("../../../tests/fixtures/prepared/custom_loop_boundaries.rs").trim()
        );
        assert!(!COMPILED.program.compiled.callbacks.ints.is_empty());
        let mut canonical = ARTIFACT;
        canonical.program.compiled = data::compiled::CompiledFunctions::interpreted();
        let (canonical, canonical_entries) = Box::leak(Box::new(canonical))
            .admit()
            .unwrap()
            .into_execution();
        let (compiled, compiled_entries) = COMPILED.admit().unwrap().into_execution();
        let function = *compiled_entries.ints[0].function();
        assert_eq!(function, *canonical_entries.ints[0].function());
        for count in [0, 1, 2, 129] {
            for connected in [false, true] {
                for fail in [false, true] {
                    for initial in [BigInt::from(3), BigInt::from(i64::MAX)] {
                        for budget in [1, 2, 3, 7, 31, 35, 36, 1024, usize::MAX] {
                            let inputs = || {
                                let mut inputs = RetainedValues::empty();
                                inputs.push_int(count.into());
                                inputs.push_bool(fail);
                                inputs.push_bool(connected);
                                inputs.push_int(initial.clone().into());
                                inputs
                            };
                            let budget = NonZeroUsize::new(budget).unwrap();
                            let expected =
                                trace_entry_turns(&canonical, function, inputs(), budget);
                            let actual = trace_entry_turns(&compiled, function, inputs(), budget);
                            assert_eq!(
                                actual.turns, expected.turns,
                                "count {count}, connected {connected}, fail {fail}, initial {initial}, budget {budget}"
                            );
                            assert_eq!(actual.echo, expected.echo);
                            assert_eq!(actual.result, expected.result);
                            if fail {
                                assert!(actual.echo.is_empty());
                                let error = actual.result.unwrap_err().into_materialized();
                                assert!(matches!(error, crate::ExecutionError::Panic(panic)
                                    if panic.kind() == crate::PanicKind::Panic
                                        && panic.site().function() == "walk"));
                            } else {
                                let total = &initial + 3 * count;
                                assert_eq!(actual.result, Ok(total));
                                assert_eq!(actual.echo.len(), 1);
                            }
                        }
                    }
                }
            }
        }

        // The hosted driver must resume the same admitted connection across
        // its real scheduling boundary, including the canonical list suffix.
        use crate::embedding::HostedModuleBuilder;
        use crate::execution_fixture::TestHost;
        use crate::runtime::execution::Domain;
        use crate::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};
        use std::sync::Arc;
        const HOSTED: data::HostedModuleArtifact =
            include!("../../../tests/fixtures/prepared/custom_loop_boundaries_hosted.rs");
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "example",
                    "src/custom_loop_boundaries.gleam",
                    source,
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let (mut bindings, _) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt, bool, bool, BigInt), BigInt>::new("integer"))
            .unwrap();
        bindings
            .function(FunctionDeclaration::<(BigInt,), bool>::new("boolean"))
            .unwrap();
        assert_eq!(
            bindings.prepare().unwrap().emit_rust(),
            include_str!("../../../tests/fixtures/prepared/custom_loop_boundaries_hosted.rs")
                .trim()
        );
        for compiled in [false, true] {
            let mut artifact = HOSTED;
            if !compiled {
                artifact.module.program.compiled = data::compiled::CompiledFunctions::interpreted();
            }
            let artifact = Box::leak(Box::new(artifact));
            for budget in [1, 2, 3, 7, 1024] {
                let (mut execution, entries, _) = artifact
                    .admit(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
                    .unwrap()
                    .into_execution();
                let function = *entries.ints[0].function();
                let (plan, stores, captures) = execution.parts_mut();
                let host = TestHost::default();
                let mut state = ();
                let mut echo = Vec::new();
                let domain = Domain::new(
                    Arc::clone(plan),
                    &host,
                    &mut state,
                    stores,
                    &mut echo,
                    captures.clone(),
                    NonZeroUsize::new(budget).unwrap(),
                );
                let context = domain.context();
                let mut expected_echo = Vec::new();
                host.block_on(domain.drive(async {
                    for count in [0, 2, 129] {
                        for initial in [BigInt::from(3), BigInt::from(i64::MAX)] {
                            for connected in [false, true] {
                                for fail in [false, true] {
                                    let mut inputs = RetainedValues::empty();
                                    inputs.push_int(count.into());
                                    inputs.push_bool(fail);
                                    inputs.push_bool(connected);
                                    inputs.push_int(initial.clone().into());
                                    let result = context
                                        .call(function, HostCallOrigin::Entry, inputs)
                                        .await
                                        .unwrap();
                                    if fail {
                                        assert!(matches!(result, Err(crate::ExecutionError::Panic(panic))
                                            if panic.kind() == crate::PanicKind::Panic
                                                && panic.site().function() == "walk"));
                                    } else {
                                        let total = &initial + 3 * count;
                                        assert_eq!(result.unwrap().into_bigint(), total);
                                        expected_echo.push(total);
                                    }
                                }
                            }
                        }
                    }
                }))
                .unwrap();
                assert_eq!(echo.len(), expected_echo.len());
                for (output, expected) in echo.iter().zip(expected_echo) {
                    assert_eq!(output.value(), &crate::Value::Int(expected));
                }
            }
        }
    }

    struct TurnTrace {
        turns: usize,
        echo: Vec<(usize, String)>,
        result: crate::runtime::error::ExecutionResult<num_bigint::BigInt>,
    }

    fn trace_int_turns(plan: &ExecutionPlan, budget: NonZeroUsize) -> TurnTrace {
        trace_entry_turns(plan, IntFunctionId(0), RetainedValues::empty(), budget)
    }

    fn trace_entry_turns(
        plan: &ExecutionPlan,
        function: IntFunctionId,
        inputs: RetainedValues,
        budget: NonZeroUsize,
    ) -> TurnTrace {
        let mut execution = Execution::new(function, HostCallOrigin::Entry, inputs);
        // Evaluation retains the real capture domain and list storage across
        // turns, just as the production asynchronous driver does.
        let mut evaluation = crate::runtime::execution::Evaluation::new(Default::default());
        let mut turns = 0;
        let mut observed = Vec::new();
        loop {
            turns += 1;
            let result = evaluation.access(|state| execution.advance(plan, state, budget));
            observed.extend(
                evaluation
                    .take_echo()
                    .into_iter()
                    .map(|output| (turns, output.value().inspect().to_string())),
            );
            let result = match result {
                Ok(Progress::Continue(next)) => {
                    execution = next;
                    continue;
                }
                Ok(Progress::Host(invoke)) => match invoke {},
                Ok(Progress::Complete(value)) => Ok(value.into_bigint()),
                Err(error) => Err(error),
            };
            return TurnTrace {
                turns,
                echo: observed,
                result,
            };
        }
    }

    fn continuing(
        progress: Progress<'_, ExecutionPlan, IntFunctionId>,
    ) -> Execution<'_, ExecutionPlan, IntFunctionId> {
        match progress {
            Progress::Continue(next) => next,
            Progress::Complete(_) => panic!("fixture entry must still be running"),
            Progress::Host(invoke) => match invoke {},
        }
    }
    #[test]
    fn native_grants_and_native_tail_targets_resume_in_the_original_function_owner() {
        use super::Position;
        use crate::plan::execution::HostedProgram;
        use crate::plan::execution::compiled::{
            CallTarget, CompiledCheckpoint, CompiledImplementation, FunctionCallsImplementation,
        };
        use crate::plan::execution::function::{
            ExecutionFunctionEntry, ExecutionFunctionRef, StringFunctionId,
        };
        use crate::plan::execution::graph::BlockId;
        use crate::plan::execution::host::HostedFunctionTarget;
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        use crate::plan::{HostCallSite, SourceSpan};
        use crate::runtime::ExecutableRuntimePlan;
        use crate::runtime::compiled::calls::{
            CallExecution, CallInputs, CallOps, CallProgress, CallStorage, CallValues,
            StringNativeExecution, StringNativeRequest,
        };
        use crate::runtime::graph::{BlockEnvironment, GraphExecution, GraphStorage};
        use crate::{
            HostCall, HostCallContinuation, HostCallError, HostConstructions, HostOwnedCompletion,
            HostProfile, HostProvider, HostProviderModule, HostProviderSet, HostTypeListEnd,
            ModuleSource, PackageSource, StringValue,
        };
        use num_bigint::BigInt;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        const SOURCE: &str = r#"
@external(erlang, "native", "append")
fn append(value: String) -> String
@external(erlang, "native", "answer")
fn answer() -> Int
@external(erlang, "native", "exercise")
fn exercise() -> Nil
pub fn handoff() { let _ = append("input") answer() }
pub fn main() { let _ = handoff exercise() True }
"#;
        struct State {
            plan: Option<Arc<HostedProgram<Profile>>>,
            native_calls: Arc<AtomicUsize>,
        }
        struct Profile;
        impl HostProfile for Profile {
            type RunState = State;
            type ExternalStores = ();
            type ExecutionState = ();
        }
        impl HostProvider<Profile> for Profile {
            type State = State;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }
        // This owner models two published boundaries. Selection, generated
        // arithmetic and full charge equivalence belong to public fixtures.
        struct AppendThenTail {
            delivered: bool,
        }
        impl CallExecution for AppendThenTail {
            fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
                false
            }
            fn retained_bytes(&self) -> usize {
                0
            }
            fn advance(self: Box<Self>, _: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                if *budget == 0 {
                    return CallProgress::Yield(self);
                }
                *budget -= 1;
                if self.delivered {
                    CallProgress::Interpreted {
                        target: CallTarget::Int(IntFunctionId(1)),
                        point: CHECKPOINT,
                        values: Box::default(),
                    }
                } else {
                    CallProgress::StringNative(StringNativeRequest {
                        function: StringFunctionId(0),
                        site: HostCallSite::from_static(
                            "example",
                            "handoff",
                            SourceSpan::new(0, SOURCE.len()),
                        ),
                        root_tail: false,
                        arguments: Box::new(CallValues {
                            strings: vec!["input".into()],
                            ..Default::default()
                        }),
                        execution: self,
                    })
                }
            }
        }
        impl StringNativeExecution for AppendThenTail {
            fn resume_native(mut self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> {
                assert_eq!(value.as_str(), Ok("input!"));
                self.delivered = true;
                self
            }
        }
        const CHECKPOINT: CompiledCheckpoint = CompiledCheckpoint {
            block: BlockId(0),
            instruction: 0,
            ints: 0,
            bools: 0,
            bit_arrays: 0,
            int_lists: 0,
            strings: 0,
            customs: 0,
            custom_lists: 0,
            int_functions: 0,
            bool_functions: 0,
        };
        fn start(
            _: usize,
            _: CallInputs<'_>,
            _: &mut CallStorage,
        ) -> Option<Box<dyn CallExecution>> {
            Some(Box::new(AppendThenTail { delivered: false }))
        }
        fn exercise<'call>(
            mut call: HostCall<'call, Profile, Profile, ()>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
            let plan = call.state().plan.as_ref().unwrap().clone();
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let functions = [
                        plan.int_function(IntFunctionId(0)),
                        plan.int_function(IntFunctionId(1)),
                    ];
                    let bodies: Vec<_> = functions
                        .iter()
                        .filter_map(|function| match function.as_ref() {
                            ExecutionFunctionRef::Graph(entry) => Some(entry.body()),
                            _ => None,
                        })
                        .collect();
                    assert_eq!(bodies.len(), 1);
                    let body = bodies[0];
                    let natives: Vec<_> = functions
                        .iter()
                        .filter_map(|function| match function.as_ref() {
                            ExecutionFunctionRef::Host(HostedFunctionTarget::Value(target)) => {
                                Some(plan.host_value_function(target).name())
                            }
                            _ => None,
                        })
                        .collect();
                    assert_eq!(natives, ["answer"]);
                    let mut engine = AppendThenTail { delivered: false };
                    let empty = BlockEnvironment::from_retained(RetainedValues::empty());
                    assert!(!engine.restart(
                        CallTarget::Int(IntFunctionId(0)),
                        0,
                        CallInputs::new(&empty)
                    ));
                    assert_eq!(engine.retained_bytes(), 0);
                    let implementation = CompiledImplementation::FunctionCalls(
                        Box::new(FunctionCallsImplementation {
                            root: true,
                            entry: 0,
                            checkpoints: vec![CHECKPOINT].into(),
                            locals: Vec::new().into(),
                            calls: Vec::new().into(),
                            creations: Vec::new().into(),
                            returns: Vec::new().into(),
                            tails: Vec::new().into(),
                            start,
                        })
                        .into(),
                    );
                    let execution = Execution {
                        function: IntFunctionId(0),
                        position: Position::Graph {
                            body,
                            execution: GraphExecution::new(
                                body.block_graph().as_view(),
                                RetainedValues::empty(),
                                Some(&implementation),
                            ),
                        },
                        storage: Box::new(GraphStorage::new()),
                    };
                    let value = execution
                        .drive(&*plan, context.execution().services(), NonZeroUsize::MIN)
                        .await
                        .unwrap()
                        .unwrap()
                        .into_bigint();
                    Ok(HostOwnedCompletion::new(move |call, _| {
                        assert_eq!(value, BigInt::from(42));
                        Ok(call.return_value(()))
                    }))
                })
            }))
        }
        fn append_continuing<'call>(
            mut call: HostCall<'call, Profile, Profile, StringValue>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            value: StringValue,
        ) -> Result<HostCallContinuation<'call, StringValue>, HostCallError> {
            call.state().native_calls.fetch_add(1, Ordering::SeqCst);
            Ok(call.resume(constructions, move |_| {
                Box::pin(async move {
                    Ok(HostOwnedCompletion::new(move |call, _| {
                        Ok(call.return_value(format!("{}!", value.as_str().unwrap()).into()))
                    }))
                })
            }))
        }
        // The same published request must keep its original owner when the
        // synchronous capability is declined and ordinary host execution resumes.
        for continuing in [false, true] {
            let native_calls = Arc::new(AtomicUsize::new(0));
            let observed = native_calls.clone();
            let provider = HostProviderModule::<Profile>::new("example", "example")
                .unwrap()
                .with_function::<(), BigInt, _>("answer", || 42.into())
                .unwrap()
                .with_resumable_function::<Profile, (), (), HostTypeListEnd, _>(
                    "exercise", exercise,
                )
                .unwrap();
            let provider = if continuing {
                provider.with_resumable_function::<Profile, (StringValue,), StringValue, HostTypeListEnd, _>("append", append_continuing).unwrap()
            } else {
                provider
                    .with_function::<(StringValue,), StringValue, _>(
                        "append",
                        move |value: StringValue| {
                            observed.fetch_add(1, Ordering::SeqCst);
                            format!("{}!", value.as_str().unwrap()).into()
                        },
                    )
                    .unwrap()
            };
            let typed = crate::compile_typed_host_program(
                "example",
                "example",
                [PackageSource::new(
                    "example",
                    Vec::<String>::new(),
                    [ModuleSource::new("example", "src/example.gleam", SOURCE)],
                )],
                HostProviderSet::from_providers([provider]).unwrap(),
            )
            .unwrap();
            let mut hosted = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let (plan, _, _) = hosted.parts_mut();
            assert_eq!(plan.synchronous_strings(), &[!continuing]);
            let mut state = State {
                plan: Some(plan.clone()),
                native_calls: native_calls.clone(),
            };
            let host = crate::execution_fixture::TestHost::default();
            let mut echo = Vec::new();
            assert_eq!(
                host.block_on(hosted.run_main(&host, &mut state, &mut echo))
                    .unwrap()
                    .try_into_value()
                    .unwrap(),
                crate::Value::Bool(true)
            );
            assert_eq!(native_calls.load(Ordering::SeqCst), 1);
            assert!(echo.is_empty());
        }
    }
}
