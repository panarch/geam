use super::RuntimeGraphState;
use super::{BlockEnvironment, CompletedGraph, GraphPosition, RetainedValues};
use crate::plan::execution::constant::{ConstantId, ConstantValue};
use crate::plan::execution::function::{
    ExecutionFunctionEntry, ExecutionFunctionRef, FunctionBodyOwner, FunctionExit,
};
use crate::plan::execution::graph::ProfiledBlockGraph;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
    EvaluatedValue,
};
use crate::runtime::function::EntryTarget;
use crate::runtime::state::list::{
    BitArrayListValueId, BoolListValueId, CustomListValueId, ExternalListValueId, FloatListValueId,
    FunctionListValueId, IntListValueId, ListListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, StringListValueId, TupleListValueId, UtfCodepointListValueId,
};
use crate::runtime::{ExecutableRuntimePlan, RuntimeGraph};
use ecow::EcoString;
use num_bigint::BigInt;
use std::marker::PhantomData;

pub(in crate::runtime) struct Execution<'plan, Plan: ExecutableRuntimePlan> {
    active: Activation<'plan, Plan>,
}

pub(in crate::runtime) enum Progress<'plan, Plan: ExecutableRuntimePlan + 'plan> {
    Continue(Execution<'plan, Plan>),
    Host(Plan::HostInvocation<'plan, Execution<'plan, Plan>>),
    Complete(CompletedGraph),
}

pub(in crate::runtime) enum Activation<'plan, Plan: ExecutableRuntimePlan + 'plan> {
    Graph(Frame<'plan, Plan>),
    Host(Plan::HostInvocation<'plan, Activation<'plan, Plan>>),
    Return(Return<'plan, Plan>),
    Complete(CompletedGraph),
}

pub(in crate::runtime) struct Frame<'plan, Plan: ExecutableRuntimePlan> {
    pub(super) graph: &'plan ProfiledBlockGraph<RuntimeGraph<Plan>>,
    pub(super) position: GraphPosition,
    exit: GraphExit<'plan, Plan>,
}

type GraphExit<'plan, Plan> = Box<
    dyn FnOnce(
            CompletedGraph,
            &mut Returns<'plan, Plan>,
        ) -> ExecutionResult<Activation<'plan, Plan>>
        + Send
        + 'plan,
>;
type Return<'plan, Plan> =
    Box<dyn FnOnce(&mut Returns<'plan, Plan>) -> Activation<'plan, Plan> + Send + 'plan>;

pub(super) struct Destination<Value> {
    index: usize,
    value: PhantomData<fn(Value)>,
}

pub(in crate::runtime) struct Returns<'plan, Plan: ExecutableRuntimePlan> {
    ints: Vec<Frame<'plan, Plan>>,
    floats: Vec<Frame<'plan, Plan>>,
    strings: Vec<Frame<'plan, Plan>>,
    bit_arrays: Vec<Frame<'plan, Plan>>,
    utf_codepoints: Vec<Frame<'plan, Plan>>,
    customs: Vec<Frame<'plan, Plan>>,
    externals: Vec<Frame<'plan, Plan>>,
    bools: Vec<Frame<'plan, Plan>>,
    nils: Vec<Frame<'plan, Plan>>,
    tuples: Vec<Frame<'plan, Plan>>,
    functions: Vec<Frame<'plan, Plan>>,
    parameter_lists: Vec<Frame<'plan, Plan>>,
    parameter_list_lists: Vec<Frame<'plan, Plan>>,
    int_lists: Vec<Frame<'plan, Plan>>,
    string_lists: Vec<Frame<'plan, Plan>>,
    bit_array_lists: Vec<Frame<'plan, Plan>>,
    utf_codepoint_lists: Vec<Frame<'plan, Plan>>,
    custom_lists: Vec<Frame<'plan, Plan>>,
    external_lists: Vec<Frame<'plan, Plan>>,
    float_lists: Vec<Frame<'plan, Plan>>,
    bool_lists: Vec<Frame<'plan, Plan>>,
    nil_lists: Vec<Frame<'plan, Plan>>,
    tuple_lists: Vec<Frame<'plan, Plan>>,
    list_lists: Vec<Frame<'plan, Plan>>,
    function_lists: Vec<Frame<'plan, Plan>>,
}

pub(super) trait ReturnValue: Send + 'static {
    fn frames<'stack, 'plan, Plan: ExecutableRuntimePlan>(
        returns: &'stack mut Returns<'plan, Plan>,
    ) -> &'stack mut Vec<Frame<'plan, Plan>>;

    fn push(self, environment: &mut BlockEnvironment);
}

impl<'plan, Plan: ExecutableRuntimePlan> Execution<'plan, Plan> {
    pub(in crate::runtime) fn new(
        graph: &'plan ProfiledBlockGraph<RuntimeGraph<Plan>>,
        inputs: RetainedValues,
    ) -> Self {
        Self {
            active: Activation::Graph(Frame {
                graph,
                position: GraphPosition::new(graph.entry(), inputs),
                exit: Box::new(|completed, _| Ok(Activation::Complete(completed))),
            }),
        }
    }

    pub(in crate::runtime) fn step(
        self,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
        returns: &mut Returns<'plan, Plan>,
    ) -> ExecutionResult<Progress<'plan, Plan>> {
        let active = match self.active {
            Activation::Graph(frame) => frame.step(plan, state, returns)?,
            Activation::Host(invoke) => {
                return Ok(Progress::Host(Plan::map_host(invoke, |active| {
                    Ok(Self { active })
                })));
            }
            Activation::Return(resume) => resume(returns),
            Activation::Complete(completed) => return Ok(Progress::Complete(completed)),
        };
        Ok(Progress::Continue(Self { active }))
    }
}

impl<'plan, Plan: ExecutableRuntimePlan> Frame<'plan, Plan> {
    fn step(
        mut self,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
        returns: &mut Returns<'plan, Plan>,
    ) -> ExecutionResult<Activation<'plan, Plan>> {
        use super::terminator::{GraphAction, NeverCall, terminator_action};

        let block = self.graph.block(self.position.block);
        if let Some(instruction) = block.instructions().get(self.position.instruction) {
            self.position.instruction += 1;
            return super::instruction::advance(plan, state, self, returns, instruction);
        }
        match terminator_action(plan, state, &self.position.environment, block.terminator())? {
            GraphAction::Continue { block, inputs } => {
                self.position = GraphPosition::new(block, inputs);
                Ok(Activation::Graph(self))
            }
            GraphAction::Exit(exit) => (self.exit)(
                CompletedGraph {
                    exit,
                    environment: self.position.environment,
                },
                returns,
            ),
            GraphAction::NeverCall {
                function,
                mut inputs,
                site,
            } => {
                drop(self);
                let function = match function {
                    NeverCall::Direct(function) => function,
                    NeverCall::Value(function) => {
                        inputs.append_captures(function.captures());
                        function.runtime_id()
                    }
                };
                Ok(enter_never(
                    plan,
                    function,
                    HostCallOrigin::source(site),
                    inputs,
                ))
            }
        }
    }

    pub(super) fn store<Value: ReturnValue>(mut self, value: Value) -> Activation<'plan, Plan> {
        value.push(&mut self.position.environment);
        Activation::Graph(self)
    }
}

impl<'plan, Plan: ExecutableRuntimePlan> Returns<'plan, Plan> {
    pub(in crate::runtime) fn new() -> Self {
        Self {
            ints: Vec::new(),
            floats: Vec::new(),
            strings: Vec::new(),
            bit_arrays: Vec::new(),
            utf_codepoints: Vec::new(),
            customs: Vec::new(),
            externals: Vec::new(),
            bools: Vec::new(),
            nils: Vec::new(),
            tuples: Vec::new(),
            functions: Vec::new(),
            parameter_lists: Vec::new(),
            parameter_list_lists: Vec::new(),
            int_lists: Vec::new(),
            string_lists: Vec::new(),
            bit_array_lists: Vec::new(),
            utf_codepoint_lists: Vec::new(),
            custom_lists: Vec::new(),
            external_lists: Vec::new(),
            float_lists: Vec::new(),
            bool_lists: Vec::new(),
            nil_lists: Vec::new(),
            tuple_lists: Vec::new(),
            list_lists: Vec::new(),
            function_lists: Vec::new(),
        }
    }

    pub(super) fn suspend<Value: ReturnValue>(
        &mut self,
        frame: Frame<'plan, Plan>,
    ) -> Destination<Value> {
        let frames = Value::frames(self);
        let index = frames.len();
        frames.push(frame);
        Destination {
            index,
            value: PhantomData,
        }
    }
}

impl<Value: ReturnValue> Destination<Value> {
    fn resume<'plan, Plan: ExecutableRuntimePlan>(
        self,
        returns: &mut Returns<'plan, Plan>,
        value: Value,
    ) -> Activation<'plan, Plan> {
        // Only this active call chain can create or consume its typed destinations.
        Value::frames(returns).swap_remove(self.index).store(value)
    }
}

pub(super) fn enter_function<'plan, Plan, Id, Value>(
    plan: &'plan Plan,
    id: Id,
    origin: HostCallOrigin,
    inputs: RetainedValues,
    destination: Destination<Value>,
    map: impl FnOnce(
        <<Id::Body as FunctionBodyOwner>::Return as super::GraphValue>::Evaluated,
    ) -> ExecutionResult<Value>
    + Send
    + 'plan,
) -> Activation<'plan, Plan>
where
    Plan: ExecutableRuntimePlan,
    Id: EntryTarget<Plan> + 'plan,
    Id::Body: 'plan,
    Value: ReturnValue,
{
    match id.entry(plan) {
        ExecutionFunctionRef::Graph(function) => {
            let body = function.body().function_body();
            let graph = body.block_graph();
            Activation::Graph(Frame {
                graph,
                position: GraphPosition::new(graph.entry(), inputs),
                exit: Box::new(
                    move |completed, returns| match body.exit(completed.exit()) {
                        FunctionExit::Return(value) => {
                            let value = map(completed.into_value(value))?;
                            Ok(destination.resume(returns, value))
                        }
                        FunctionExit::TailCall { function, args } => {
                            let (id, origin) = id.next(function);
                            let inputs = completed.into_retained(args);
                            Ok(enter_function(plan, id, origin, inputs, destination, map))
                        }
                    },
                ),
            })
        }
        ExecutionFunctionRef::Host(target) => Activation::Host(Plan::map_host(
            Id::prepare_host(plan, origin, target, inputs),
            move |value| {
                let value = map(value)?;
                Ok(Activation::Return(Box::new(move |returns| {
                    destination.resume(returns, value)
                })))
            },
        )),
    }
}

pub(super) fn enter_constant<'plan, Plan, Local, Value>(
    plan: &'plan Plan,
    id: ConstantId<Local>,
    destination: Destination<Value>,
    map: impl FnOnce(Local::Evaluated) -> Value + Send + 'plan,
) -> Activation<'plan, Plan>
where
    Plan: ExecutableRuntimePlan,
    Local: ConstantValue + super::GraphValue + Sync + 'plan,
    Value: ReturnValue,
{
    let constant = plan.constant(id);
    let graph = constant.block_graph();
    Activation::Graph(Frame {
        graph,
        position: GraphPosition::new(graph.entry(), RetainedValues::empty()),
        exit: Box::new(move |completed, returns| {
            let local = constant.return_(completed.exit());
            let value = map(completed.into_value(local));
            Ok(destination.resume(returns, value))
        }),
    })
}

fn enter_never<'plan, Plan: ExecutableRuntimePlan>(
    plan: &'plan Plan,
    id: crate::plan::execution::function::NeverFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> Activation<'plan, Plan> {
    match plan.never_function(id).as_ref() {
        ExecutionFunctionRef::Graph(function) => {
            let body = function.body().function_body();
            let graph = body.block_graph();
            Activation::Graph(Frame {
                graph,
                position: GraphPosition::new(graph.entry(), inputs),
                exit: Box::new(move |completed, _| match body.exit(completed.exit()) {
                    FunctionExit::Return(never) => match *never {},
                    FunctionExit::TailCall { function, args } => {
                        let inputs = completed.into_retained(args);
                        Ok(enter_never(
                            plan,
                            *function.function(),
                            HostCallOrigin::source(function.site().clone()),
                            inputs,
                        ))
                    }
                }),
            })
        }
        ExecutionFunctionRef::Host(target) => Activation::Host(Plan::map_host(
            plan.prepare_host_never(origin, target, inputs),
            |never| match never {},
        )),
    }
}

macro_rules! return_value {
    ($value:ty, $frames:ident, $push:ident) => {
        impl ReturnValue for $value {
            fn frames<'stack, 'plan, Plan: ExecutableRuntimePlan>(
                returns: &'stack mut Returns<'plan, Plan>,
            ) -> &'stack mut Vec<Frame<'plan, Plan>> {
                &mut returns.$frames
            }

            fn push(self, environment: &mut BlockEnvironment) {
                environment.$push(self);
            }
        }
    };
}

return_value!(BigInt, ints, push_int);
return_value!(f64, floats, push_float);
return_value!(EcoString, strings, push_string);
return_value!(EvaluatedBitArray, bit_arrays, push_bit_array);
return_value!(char, utf_codepoints, push_utf_codepoint);
return_value!(EvaluatedCustomValue, customs, push_custom);
return_value!(EvaluatedExternalValue, externals, push_external);
return_value!(bool, bools, push_bool);
return_value!(Vec<EvaluatedValue>, tuples, push_tuple);
return_value!(EvaluatedFunctionValue, functions, push_function_value);
return_value!(ParameterListValueId, parameter_lists, push_parameter_list);
return_value!(
    ParameterListListValueId,
    parameter_list_lists,
    push_parameter_list_list
);
return_value!(IntListValueId, int_lists, push_int_list);
return_value!(StringListValueId, string_lists, push_string_list);
return_value!(BitArrayListValueId, bit_array_lists, push_bit_array_list);
return_value!(
    UtfCodepointListValueId,
    utf_codepoint_lists,
    push_utf_codepoint_list
);
return_value!(CustomListValueId, custom_lists, push_custom_list);
return_value!(ExternalListValueId, external_lists, push_external_list);
return_value!(FloatListValueId, float_lists, push_float_list);
return_value!(BoolListValueId, bool_lists, push_bool_list);
return_value!(NilListValueId, nil_lists, push_nil_list);
return_value!(TupleListValueId, tuple_lists, push_tuple_list);
return_value!(ListListValueId, list_lists, push_list_list);
return_value!(FunctionListValueId, function_lists, push_function_list);

impl ReturnValue for () {
    fn frames<'stack, 'plan, Plan: ExecutableRuntimePlan>(
        returns: &'stack mut Returns<'plan, Plan>,
    ) -> &'stack mut Vec<Frame<'plan, Plan>> {
        &mut returns.nils
    }

    fn push(self, environment: &mut BlockEnvironment) {
        environment.push_nil();
    }
}

#[cfg(test)]
mod tests {
    use super::{Execution, Progress, Returns};
    use crate::ExecutionPlan;
    use crate::plan::execution::function::{FunctionExit, IntFunctionId};
    use crate::runtime::graph::{CompletedGraph, RetainedValues};
    use crate::runtime::state::RuntimeState;
    use crate::runtime::{RuntimeListStorage, Value};

    fn returned_int(plan: &ExecutionPlan, completed: CompletedGraph) -> num_bigint::BigInt {
        let body = plan.int_function(IntFunctionId(0)).body();
        match body.exit(completed.exit()) {
            FunctionExit::Return(value) => completed.into_value(value),
            FunctionExit::TailCall { .. } => {
                panic!("fixture root returns rather than tail-calling")
            }
        }
    }

    fn continuing(progress: Progress<'_, ExecutionPlan>) -> Execution<'_, ExecutionPlan> {
        match progress {
            Progress::Continue(next) => next,
            Progress::Complete(_) => panic!("fixture graph must still be running"),
            Progress::Host(invoke) => match invoke {},
        }
    }

    fn complete_int_graph(plan: &ExecutionPlan) -> CompletedGraph {
        let body = plan.int_function(IntFunctionId(0)).body();
        let mut execution = Execution::new(body.block_graph(), RetainedValues::empty());
        let mut returns = Returns::new();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        loop {
            match execution.step(plan, &mut state, &mut returns).unwrap() {
                Progress::Continue(next) => execution = next,
                Progress::Complete(completed) => return completed,
                Progress::Host(invoke) => match invoke {},
            }
        }
    }

    #[test]
    #[should_panic(expected = "fixture root returns rather than tail-calling")]
    fn returned_int_rejects_a_source_tail_call() {
        let plan = crate::runtime::plan_src("fn answer() { 42 } pub fn main() { answer() }");
        returned_int(&plan, complete_int_graph(&plan));
    }

    #[test]
    #[should_panic(expected = "fixture graph must still be running")]
    fn continuing_rejects_a_completed_source_graph() {
        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        continuing(Progress::Complete(complete_int_graph(&plan)));
    }

    #[test]
    fn nested_calls_transfer_between_steps_without_replaying_echo_or_losing_captures() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Boxed { Boxed(List(Int)) }

fn nested(value: Int) -> Int {
  echo value
  case value {
    0 -> 0
    _ -> nested(value - 1) + 1
  }
}

pub fn main() {
  let offset = 40
  let add = fn(value) { offset + nested(value) }
  let boxed = Boxed([1, 2])
  let Boxed(values) = boxed
  echo values
  case values {
    [first, ..] -> add(first) + 1
    [] -> 0
  }
}
"#,
        );
        let body = plan.int_function(IntFunctionId(0)).body();
        let mut execution = Execution::new(body.block_graph(), RetainedValues::empty());
        let mut returns = Returns::new();
        let mut lists = RuntimeListStorage::default();
        let mut output = Vec::new();
        let mut steps = 0;
        let (completed, output) = std::thread::scope(|scope| {
            loop {
                let plan = &plan;
                let (step, retained_returns, retained_lists, retained_output) = scope
                    .spawn(move || {
                        let mut state = RuntimeState::with_host_and_lists(&mut output, (), lists);
                        let step = execution
                            .step(plan, &mut state, &mut returns)
                            .expect("one actual evaluator step");
                        let lists = state.lists().clone();
                        drop(state);
                        (step, returns, lists, output)
                    })
                    .join()
                    .expect("owned activation transfers");
                steps += 1;
                assert!(steps < 200, "the finite source must make progress");
                lists = retained_lists;
                returns = retained_returns;
                output = retained_output;
                match step {
                    Progress::Continue(next) => execution = next,
                    Progress::Host(invoke) => match invoke {},
                    Progress::Complete(completed) => break (completed, output),
                }
            }
        });
        assert_eq!(returned_int(&plan, completed), 42.into());
        assert!(steps > 20);
        assert_eq!(
            output
                .iter()
                .map(|output| output.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["[1, 2]", "1", "0"]
        );
    }

    #[test]
    fn deep_non_tail_calls_use_owned_frames_instead_of_the_rust_stack() {
        let plan = crate::runtime::plan_src(
            r#"
fn count(value: Int) -> Int {
  case value {
    0 -> 0
    _ -> count(value - 1) + 1
  }
}
pub fn main() { count(20_000) }
"#,
        );
        let result = std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(move || crate::run_main(&plan, &mut Vec::new()))
            .expect("small stack worker")
            .join()
            .expect("non-tail execution completes");
        assert_eq!(result, Ok(Value::Int(20_000.into())));
    }

    #[test]
    fn abandoning_a_deep_call_chain_drops_frames_without_recursive_destruction() {
        let plan = crate::runtime::plan_src(
            r#"
fn count(value: Int) -> Int { count(value + 1) + 1 }
pub fn main() { count(0) + 1 }
"#,
        );
        std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(move || {
                let body = plan.int_function(IntFunctionId(0)).body();
                let mut execution = Execution::new(body.block_graph(), RetainedValues::empty());
                let mut returns = Returns::new();
                let mut echo = Vec::new();
                let mut state = RuntimeState::new(&mut echo);
                for _ in 0..50_000 {
                    execution = continuing(
                        execution
                            .step(&plan, &mut state, &mut returns)
                            .expect("recursive source step"),
                    );
                }
                assert!(returns.ints.len() > 5_000);
                drop(execution);
                drop(returns);
            })
            .expect("small stack worker")
            .join()
            .expect("flat activation destruction");
    }

    #[test]
    fn source_and_native_never_entries_propagate_their_original_failure() {
        use crate::{HostFailure, HostProviderModule, HostProviderSet, StatelessHostProfile};
        use std::convert::Infallible;

        for (callee, expected) in [
            ("source_stop", "panic: source stopped"),
            (
                "native_stop",
                "host function application::main.native_stop failed: native stopped",
            ),
        ] {
            let source = format!(
                r#"
@external(erlang, "native", "stop")
fn native_stop() -> value
fn source_stop() {{ panic as "source stopped" }}
fn forward() {{ let stop = {callee} stop() }}
pub fn main() {{ let stop = forward stop() }}
"#
            );
            let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
                .unwrap()
                .with_fallible_function("native_stop", || -> Result<Infallible, HostFailure> {
                    Err(HostFailure::new("native stopped"))
                })
                .unwrap();
            let typed = crate::compile_typed_host_program(
                "application",
                "main",
                [crate::PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [crate::ModuleSource::new(
                        "main",
                        "main.gleam",
                        source.as_str(),
                    )],
                )],
                HostProviderSet::from_providers([provider]).unwrap(),
            )
            .unwrap();
            let mut execution = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let host = crate::execution_fixture::TestHost::default();
            let mut echo = Vec::new();
            let error = host
                .block_on(execution.run_main(&host, &mut (), &mut echo))
                .unwrap_err();
            assert_eq!(error.to_string(), expected);
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn source_and_native_function_returns_check_the_evaluated_target_family() {
        use crate::execution_fixture::TestHost;
        use crate::host::{
            HostCall, HostCallCompletion, HostCallError, HostCallable, HostFunctionType,
            HostProfile, HostProvider, HostProviderModule, HostProviderSet, HostTypeListEnd,
        };
        use crate::plan::execution::function::{FunctionReturnFamily, TupleFunctionId};
        use crate::runtime::execution::Domain;
        use crate::runtime::{ExecutionError, HostCallOrigin, InvariantError};
        use std::sync::Arc;

        struct Profile;
        impl HostProfile for Profile {
            type RunState = usize;
            type ExternalStores = ();
        }
        struct Provider;
        impl HostProvider<Profile> for Provider {
            type State = usize;
            fn project(state: &mut usize) -> &mut usize {
                state
            }
        }
        type Callback = HostFunctionType<HostTypeListEnd, f64>;
        fn forward<'call>(
            mut call: HostCall<'call, Profile, Provider, Callback>,
            callback: HostCallable<'call, HostTypeListEnd, f64>,
        ) -> Result<HostCallCompletion<'call, Callback>, HostCallError> {
            *call.state() += 1;
            Ok(call.return_value(callback))
        }

        let source = r#"
@external(erlang, "native", "forward")
fn forward(value: fn() -> Float) -> fn() -> Float

fn apply_int(factory: fn(fn() -> Float) -> fn() -> Int, value: fn() -> Float) {
  let result = factory(value)
  result()
}
fn apply_float(factory: fn(fn() -> Float) -> fn() -> Float, value: fn() -> Float) {
  let result = factory(value)
  let assert 1.5 = result()
  42
}
fn integer(_value: fn() -> Float) { fn() { 42 } }
fn floating(value: fn() -> Float) { leaf(value) }
fn leaf(value: fn() -> Float) { value }
pub fn main() { #(apply_int, apply_float, integer, floating, forward, fn() { 1.5 }) }
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                Vec::<String>::new(),
                [crate::ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::from_providers([HostProviderModule::new("application", "main")
                .unwrap()
                .with_scoped_function::<Provider, (Callback,), Callback, _>("forward", forward)
                .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores) = execution.parts_mut();
        let host = TestHost::default();
        let mut state = 0;
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut state,
            stores,
            &mut echo,
            Domain::<Profile>::DEFAULT_BUDGET,
        );
        let context = domain.context();
        host.block_on(domain.drive(async {
            let values = context
                .call(
                    TupleFunctionId(0),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )
                .await
                .unwrap()
                .unwrap();
            assert_eq!(values.len(), 6);
            for (entry, factory, expected) in [
                (IntFunctionId(0), 2, Ok(42.into())),
                (IntFunctionId(1), 3, Ok(42.into())),
                (IntFunctionId(1), 4, Ok(42.into())),
                (
                    IntFunctionId(0),
                    3,
                    Err(ExecutionError::Invariant(
                        InvariantError::FunctionReturnFamilyMismatch {
                            expected: FunctionReturnFamily::Int,
                            actual: FunctionReturnFamily::Float,
                        },
                    )),
                ),
                (
                    IntFunctionId(0),
                    4,
                    Err(ExecutionError::Invariant(
                        InvariantError::FunctionReturnFamilyMismatch {
                            expected: FunctionReturnFamily::Int,
                            actual: FunctionReturnFamily::Float,
                        },
                    )),
                ),
            ] {
                // The last two cases corrupt only the evaluated factory argument.
                // The compiled entries, native adapter and return slots are unchanged.
                let mut inputs = RetainedValues::empty();
                inputs.push_evaluated(values[factory].clone());
                inputs.push_evaluated(values[5].clone());
                let result = context
                    .call(entry, HostCallOrigin::Entry, inputs)
                    .await
                    .unwrap();
                assert_eq!(result, expected);
            }
        }))
        .unwrap();
        assert_eq!(state, 2);
        assert!(echo.is_empty());
    }
}
