use super::{Activation, CallFrame, Execution, NativeCallsExecution, Progress, Storage};
use crate::plan::execution::compiled::{NativeLoopImplementation, NativeLoopProducer};
use crate::plan::execution::function::{
    ExecutionFunctionRef, FunctionBodyOwner, FunctionExit, NilFunctionId,
};
use crate::plan::execution::graph::{
    BlockView, IntLocalId, NilInstruction, ProfiledInstructionKind, Terminator,
};
use crate::runtime::compiled::native_loop::{NativeLoopCursor, NativeLoopPhase, NativeLoopState};
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::{EvaluatedFunction, EvaluatedValue};
use crate::runtime::function::EntryTarget;
use crate::runtime::graph::{BlockEnvironment, CompletedGraph, RetainedValues, RuntimeGraphState};
use crate::runtime::{ExecutableRuntimePlan, RuntimeGraph, StoredRuntimeValue};
use std::ops::ControlFlow;

pub(in crate::runtime::graph) struct NativeLoopExecution<'plan, Plan: ExecutableRuntimePlan> {
    frame: CallFrame<'plan, Plan>,
    implementation: &'plan NativeLoopImplementation,
    state: NativeLoopState<Plan::NativeLoopBinding>,
}

impl<'plan, Plan: ExecutableRuntimePlan> NativeLoopExecution<'plan, Plan> {
    pub(super) fn enter(
        frame: CallFrame<'plan, Plan>,
        implementation: &'plan NativeLoopImplementation,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
        storage: &mut Storage<'plan, Plan>,
        remaining: &mut usize,
    ) -> ExecutionResult<Progress<'plan, Plan>> {
        if let Some(selected) = Self::select(&frame, implementation, plan, state) {
            Self::new(frame, implementation, selected).advance(plan, storage, remaining)
        } else {
            let active = if let Some(fallback) = plan.native_loop_fallback(implementation.function)
            {
                return NativeCallsExecution::enter(
                    frame,
                    fallback,
                    &implementation.contract,
                    plan,
                    state,
                    storage,
                    remaining,
                );
            } else {
                frame.into_graph().advance(plan, state, storage, remaining)
            };
            active.map(|active| Progress::Continue(Execution { active }))
        }
    }

    fn select(
        frame: &CallFrame<'plan, Plan>,
        implementation: &NativeLoopImplementation,
        plan: &Plan,
        state: &impl RuntimeGraphState<Error = crate::ExecutionError>,
    ) -> Option<NativeLoopState<Plan::NativeLoopBinding>> {
        let binding = plan.bind_native_loop(&implementation.contract)?;
        let (input, producer_literal) = match &implementation.contract.producer {
            NativeLoopProducer::Int(local) => {
                retained_producer(frame.position.environment.int_function(*local), plan, state)
            }
            NativeLoopProducer::Float(local) => retained_producer(
                frame.position.environment.float_function(*local),
                plan,
                state,
            ),
            NativeLoopProducer::String(local) => retained_producer(
                frame.position.environment.string_function(*local),
                plan,
                state,
            ),
            NativeLoopProducer::BitArray(local) => retained_producer(
                frame.position.environment.bit_array_function(*local),
                plan,
                state,
            ),
            NativeLoopProducer::UtfCodepoint(local) => retained_producer(
                frame.position.environment.utf_codepoint_function(*local),
                plan,
                state,
            ),
            NativeLoopProducer::Bool(local) => retained_producer(
                frame.position.environment.bool_function(*local),
                plan,
                state,
            ),
            NativeLoopProducer::Nil(local) => {
                retained_nil_producer(frame.position.environment.nil_function(*local), plan, state)
            }
        }?;
        Some(NativeLoopState {
            binding,
            cursor: NativeLoopCursor {
                counter: frame.position.environment.int(IntLocalId(0)),
                input: StoredRuntimeValue::new(input, plan.value_metadata()),
                phase: NativeLoopPhase::ProducerCall,
                producer_literal,
            },
            kernel: implementation.run,
        })
    }

    fn new(
        frame: CallFrame<'plan, Plan>,
        implementation: &'plan NativeLoopImplementation,
        state: NativeLoopState<Plan::NativeLoopBinding>,
    ) -> Self {
        Self {
            frame,
            implementation,
            state,
        }
    }

    pub(super) fn advance(
        self,
        plan: &'plan Plan,
        storage: &mut Storage<'plan, Plan>,
        remaining: &mut usize,
    ) -> ExecutionResult<Progress<'plan, Plan>> {
        let Self {
            frame,
            implementation,
            state,
        } = self;
        let NativeLoopState {
            binding,
            cursor,
            kernel,
        } = state;
        match cursor.into_return() {
            ControlFlow::Break(returned) => {
                let mut values = RetainedValues::empty();
                values.push_evaluated(returned.value().clone());
                let active = frame.exit.exit(
                    CompletedGraph {
                        exit: implementation.contract.exit,
                        environment: BlockEnvironment::from_retained(values),
                    },
                    storage,
                )?;
                Ok(Progress::Continue(Execution { active }))
            }
            ControlFlow::Continue(cursor) => {
                // The graph driver already charged the first canonical step.
                let allowance = *remaining + 1;
                *remaining = 0;
                let state = NativeLoopState {
                    binding,
                    cursor,
                    kernel,
                };
                Ok(Progress::Host(Plan::map_host(
                    plan.prepare_native_loop(state, allowance),
                    move |state| {
                        Ok(Execution {
                            active: Activation::NativeLoop(Box::new(Self {
                                frame,
                                implementation,
                                state,
                            })),
                        })
                    },
                )))
            }
        }
    }
}

fn producer_block<'plan, Plan: ExecutableRuntimePlan, Id: EntryTarget<Plan> + Clone>(
    producer: &EvaluatedFunction<Id>,
    plan: &'plan Plan,
    state: &impl RuntimeGraphState<Error = crate::ExecutionError>,
) -> Option<BlockView<'plan, RuntimeGraph<Plan>>> {
    if producer
        .capture_frame()
        .domain()
        .is_some_and(|domain| domain != state.captures().domain())
    {
        return None;
    }
    let ExecutionFunctionRef::Graph(entry) = producer.runtime_id().entry(plan) else {
        return None;
    };
    let body = entry.body().function_body();
    let graph = body.block_graph();
    let block = graph.block(graph.entry());
    if graph.blocks().len() != 1 {
        return None;
    }
    let Terminator::Exit(exit) = block.terminator() else {
        return None;
    };
    let FunctionExit::Return(_) = body.exit(*exit) else {
        return None;
    };
    Some(block)
}

fn retained_nil_producer<Plan: ExecutableRuntimePlan>(
    producer: &EvaluatedFunction<NilFunctionId>,
    plan: &Plan,
    state: &impl RuntimeGraphState<Error = crate::ExecutionError>,
) -> Option<(EvaluatedValue, bool)> {
    let block = producer_block(producer, plan, state)?;
    // Nil has one value, even when an admitted body retains unused captures.
    // Preserve its instruction charge without requiring a capture-free body.
    let literal = match block.instructions() {
        [] => false,
        [instruction] if matches!(instruction.value(), Some(value) if matches!(value.kind, ProfiledInstructionKind::Nil(NilInstruction::Value))) => {
            true
        }
        _ => return None,
    };
    Some((EvaluatedValue::Nil, literal))
}

fn retained_producer<Plan: ExecutableRuntimePlan, Id: EntryTarget<Plan> + Clone>(
    producer: &EvaluatedFunction<Id>,
    plan: &Plan,
    state: &impl RuntimeGraphState<Error = crate::ExecutionError>,
) -> Option<(EvaluatedValue, bool)> {
    let block = producer_block(producer, plan, state)?;
    if !block.instructions().is_empty() {
        return None;
    }
    let [capture] = producer.captures() else {
        return None;
    };
    // The caller's argument-free function type and the sealed capture pack fix
    // this block's sole parameter. With no instructions, its admitted non-Nil
    // return must be that parameter; there is no other defined scalar local.
    Some((capture.clone().into_value(), false))
}

#[cfg(test)]
mod tests {
    use super::{NativeLoopExecution, retained_nil_producer, retained_producer};
    use crate::execution_fixture::TestHost;
    use crate::plan::execution::function::TupleFunctionId;
    use crate::plan::execution::graph::{IntFunctionLocalId, NilFunctionLocalId};
    use crate::runtime::execution::Domain;
    use crate::runtime::graph::{BlockEnvironment, RetainedValues};
    use crate::runtime::state::RuntimeState;
    use crate::runtime::{EvaluatedValue, HostCallOrigin, RuntimeListStorage};
    use crate::{HostProviderModule, HostProviderSet, StatelessHostProfile};
    use std::sync::Arc;

    #[test]
    fn a_completed_native_loop_preserves_the_existing_fallible_return_mapper() {
        use super::super::{CallFrame, Frame, FunctionContinuation, RootExit, Storage};
        use crate::plan::execution::compiled::{
            NativeLoopContract, NativeLoopImplementation, NativeLoopTarget,
        };
        use crate::plan::execution::function::{
            ExecutionFunctionEntry, ExecutionFunctionRef, FunctionReturnFamily, IntFunctionId,
        };
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        use crate::runtime::compiled::native_loop::{NativeLoopOps, NativeLoopProgress, run};
        use crate::runtime::graph::GraphPosition;
        use crate::runtime::integer::IntegerValue;
        use crate::{
            ExecutionError, HostCall, HostCallCompletion, HostCallError, HostProvider,
            InvariantError,
        };
        use num_bigint::BigInt;
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        fn observe<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            let _ = call.state();
            assert_eq!(value, BigInt::from(7));
            Ok(call.return_value(value + 1))
        }
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let source = r#"
@external(erlang, "native", "observe")
fn observe(value: Int) -> Int
fn cycle(counter: Int, producer: fn() -> Int) -> Int {
  let returned = observe(producer())
  case counter { 1 -> returned _ -> cycle(counter - 1, producer) }
}
fn captured(value: Int) { fn() { value } }
pub fn main() {
  let unused = observe(7)
  #(cycle, captured(7))
}
"#;
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
            HostProviderSet::from_providers([HostProviderModule::new("example", "example")
                .unwrap()
                .with_scoped_retained_function::<Provider, BigInt, BigInt, _, _>(
                    "observe",
                    observe,
                    move |value: BigInt| {
                        assert_eq!(value, BigInt::from(7));
                        observed.fetch_add(1, Ordering::SeqCst);
                        Ok(value + 1)
                    },
                )
                .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores, captures) = execution.parts_mut();
        let graphs = (0..3)
            .filter_map(
                |index| match plan.int_function(IntFunctionId(index)).as_ref() {
                    ExecutionFunctionRef::Graph(entry) => {
                        Some((IntFunctionId(index), entry.body()))
                    }
                    ExecutionFunctionRef::Host(_) => None,
                },
            )
            .collect::<Vec<_>>();
        assert_eq!(graphs.len(), 2);
        let (id, body) = graphs[0];
        let contract = NativeLoopContract::inspect(body.block_graph()).unwrap();
        assert_eq!(contract.site.function(), "cycle");
        let implementation = NativeLoopImplementation {
            function: NativeLoopTarget::Int(id),
            entry: 0,
            checkpoints: vec![contract.checkpoint()].into(),
            contract,
            run,
        };
        let host = TestHost::default();
        let mut run_state = ();
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut run_state,
            stores,
            &mut echo,
            captures.clone(),
            Domain::<StatelessHostProfile>::DEFAULT_BUDGET,
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
            assert_eq!(values.len(), 2);
            let mut inputs = RetainedValues::empty();
            inputs.push_int(1.into());
            inputs.push_evaluated(values[1].clone());
            let graph = body.block_graph().as_view();
            let mut storage = Storage::new();
            let mut caller_inputs = RetainedValues::empty();
            caller_inputs.push_int(1.into());
            caller_inputs.push_evaluated(values[1].clone());
            let destination = storage.returns.suspend(Frame {
                graph,
                position: GraphPosition::new(graph.entry(), caller_inputs),
                exit: Box::new(RootExit),
            });
            let expected = InvariantError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Int,
                actual: FunctionReturnFamily::Float,
            };
            let failure = expected.clone();
            let mapped = Arc::new(AtomicUsize::new(0));
            let observed = Arc::clone(&mapped);
            let frame = CallFrame {
                graph,
                position: GraphPosition::new(graph.entry(), inputs),
                exit: Box::new(FunctionContinuation {
                    plan: &**plan,
                    id,
                    body,
                    destination,
                    // Exercise the existing fallible return adapter after a real
                    // selected loop completes; this is not a new runtime failure.
                    map: move |value: IntegerValue| -> Result<IntegerValue, ExecutionError> {
                        assert_eq!(value, IntegerValue::from(8_i64));
                        observed.fetch_add(1, Ordering::SeqCst);
                        Err(ExecutionError::Invariant(failure))
                    },
                }),
            };
            let mut worker_echo = Vec::new();
            let state = RuntimeState::with_host_storage(
                &mut worker_echo,
                (),
                RuntimeListStorage::default(),
                context.captures().clone(),
            );
            let mut selected =
                NativeLoopExecution::select(&frame, &implementation, &**plan, &state).unwrap();
            let ops = NativeLoopOps {
                native: selected.binding.native.as_ref(),
                unit: None,
            };
            let mut allowance = 6;
            assert_eq!(
                run(&mut selected.cursor, &ops, &mut allowance),
                Ok(NativeLoopProgress::Complete)
            );
            assert_eq!(allowance, 0);
            let error = NativeLoopExecution::new(frame, &implementation, selected)
                .advance(&**plan, &mut storage, &mut 0)
                .err()
                .unwrap();
            assert_eq!(error, ExecutionError::Invariant(expected));
            assert_eq!(calls.load(Ordering::SeqCst), 1);
            assert_eq!(mapped.load(Ordering::SeqCst), 1);
            assert!(worker_echo.is_empty());
        }))
        .unwrap();
        assert!(echo.is_empty());
    }

    #[test]
    fn native_loop_producers_retain_values_but_leave_computation_and_other_domains_canonical() {
        let source = r#"
@external(erlang, "native", "nil")
fn native_nil() -> Nil
@external(erlang, "native", "int")
fn native_int() -> Int
fn captured(value: Nil) { fn() { value } }
fn effect(value: Nil) { fn() { echo value value } }
fn branched(flag: Bool) { fn() { case flag { True -> Nil False -> panic } } }
fn recursive() -> Nil { recursive() }
fn forward() -> Nil { recursive() }
fn unused(value: Nil) { fn() { let unused = 1 value } }
fn integer(value: Int) { fn() { value } }
fn extra_capture(value: Int, flag: Bool) { fn() { let unused = flag value } }
fn computed(value: Int) { fn() { value + 1 } }
pub fn main() {
  #(captured(Nil), fn() { Nil }, native_nil, recursive, effect(Nil),
    branched(True), unused(Nil), forward, integer(7), extra_capture(7, True), computed(7), native_int)
}
"#;
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
            HostProviderSet::<StatelessHostProfile>::from_providers([HostProviderModule::new(
                "example", "example",
            )
            .unwrap()
            .with_function::<(), (), _>("native_nil", || ())
            .unwrap()
            .with_function::<(), num_bigint::BigInt, _>("native_int", || 7.into())
            .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores, captures) = execution.parts_mut();
        let host = TestHost::default();
        let mut run_state = ();
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut run_state,
            stores,
            &mut echo,
            captures.clone(),
            Domain::<StatelessHostProfile>::DEFAULT_BUDGET,
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
            assert_eq!(values.len(), 12);
            let mut retained = RetainedValues::empty();
            for value in values {
                retained.push_evaluated(value);
            }
            let environment = BlockEnvironment::from_retained(retained);
            let native = environment.int_function(IntFunctionLocalId(3));
            let returned = context
                .call(
                    native.runtime_id(),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )
                .await
                .unwrap()
                .unwrap();
            assert_eq!(returned.small(), Some(7));
            let mut worker_echo = Vec::new();
            let state = RuntimeState::with_host_storage(
                &mut worker_echo,
                (),
                RuntimeListStorage::default(),
                context.captures().clone(),
            );
            for (index, expected) in [
                (0, Some((EvaluatedValue::Nil, false))),
                (1, Some((EvaluatedValue::Nil, true))),
                (2, None),
                (3, None),
                (4, None),
                (5, None),
                (6, None),
                (7, None),
            ] {
                assert_eq!(
                    retained_nil_producer(
                        environment.nil_function(NilFunctionLocalId(index)),
                        &**plan,
                        &state
                    ),
                    expected
                );
            }
            for (index, expected) in [
                (0, Some((EvaluatedValue::Int(7.into()), false))),
                (1, None),
                (2, None),
                (3, None),
            ] {
                assert_eq!(
                    retained_producer(
                        environment.int_function(IntFunctionLocalId(index)),
                        &**plan,
                        &state,
                    ),
                    expected,
                );
            }
            let mut foreign_echo = Vec::new();
            let foreign = RuntimeState::new(&mut foreign_echo);
            assert_eq!(
                retained_nil_producer(
                    environment.nil_function(NilFunctionLocalId(0)),
                    &**plan,
                    &foreign
                ),
                None
            );
            assert!(worker_echo.is_empty());
            assert!(foreign_echo.is_empty());
        }))
        .unwrap();
        assert!(echo.is_empty());
    }
}
