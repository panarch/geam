use super::{Activation, CallFrame, Execution, Progress, Storage, calls};
use crate::plan::execution::compiled::{FunctionCallsImplementation, NativeLoopContract};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::compiled::native_calls::{
    CallNativeTarget, NativeCallsMachine, NativeCallsProgress, NativeCallsState,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::RuntimeGraphState;

pub(in crate::runtime::graph) struct NativeCallsExecution<'plan, Plan: ExecutableRuntimePlan> {
    frame: CallFrame<'plan, Plan>,
    state: NativeCallsState<Plan::NativeLoopBinding>,
}

impl<'plan, Plan: ExecutableRuntimePlan> NativeCallsExecution<'plan, Plan> {
    pub(super) fn enter(
        frame: CallFrame<'plan, Plan>,
        fallback: &'plan FunctionCallsImplementation,
        contract: &NativeLoopContract,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
        storage: &mut Storage<'plan, Plan>,
        remaining: &mut usize,
    ) -> ExecutionResult<Progress<'plan, Plan>> {
        if let Some(target) = CallNativeTarget::select(contract.native)
            && let Some(binding) = plan.bind_native_loop(contract)
        {
            let entered = calls::enter(frame, fallback, fallback.entry, storage);
            let (frame, execution) = match entered {
                calls::CallEntry::Running(frame, execution) => (frame, execution),
                canonical @ calls::CallEntry::Canonical(_) => {
                    return calls::advance_entry(canonical, plan, state, storage, remaining)
                        .map(|active| Progress::Continue(Execution { active }));
                }
            };
            let root_tail_entry = frame.exit.root_tail_entry();
            let machine =
                NativeCallsMachine::new(target, std::mem::take(&mut storage.numeric), execution)
                    .with_workspace(
                        storage.string.take(),
                        storage.bit_array_loop.take(),
                        root_tail_entry,
                    );
            let selected = Self {
                frame,
                state: NativeCallsState {
                    binding,
                    progress: NativeCallsProgress::Running(machine),
                },
            };
            return selected.advance(plan, state, storage, remaining);
        }
        calls::advance(
            frame,
            fallback,
            fallback.entry,
            plan,
            state,
            storage,
            remaining,
        )
        .map(|active| Progress::Continue(Execution { active }))
    }

    pub(super) fn advance(
        self,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
        storage: &mut Storage<'plan, Plan>,
        remaining: &mut usize,
    ) -> ExecutionResult<Progress<'plan, Plan>> {
        let Self {
            frame,
            state: selected,
        } = self;
        let NativeCallsState { binding, progress } = selected;
        match progress {
            NativeCallsProgress::Canonical {
                numeric,
                strings,
                bit_arrays,
                progress,
            } => {
                storage.numeric = numeric;
                storage.string = strings;
                storage.bit_array_loop = bit_arrays;
                // Delivery is not a new source step: the generated advance
                // already paid for this bridge/return, or stopped before the
                // original interpreted checkpoint.
                *remaining += 1;
                calls::resume_progress(frame, progress, plan, state, storage)
                    .map(|active| Progress::Continue(Execution { active }))
            }
            NativeCallsProgress::Running(machine) => {
                let allowance = *remaining + 1;
                *remaining = 0;
                Ok(Progress::Host(Plan::map_host(
                    plan.prepare_native_calls(binding, machine, allowance),
                    move |state| {
                        Ok(Execution {
                            active: Activation::NativeCalls(Box::new(Self { frame, state })),
                        })
                    },
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NativeCallsExecution, NativeCallsMachine, NativeCallsState};
    use crate::execution_fixture::TestHost;
    use crate::plan::execution::HostedProgram;
    use crate::plan::execution::compiled::{CallTarget, NativeLoopContract, NativeLoopTarget};
    use crate::plan::execution::function::{
        ExecutionFunctionEntry, ExecutionFunctionRef, IntFunctionId, TupleFunctionId,
    };
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::runtime::compiled::calls::{
        CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress,
    };
    use crate::runtime::compiled::native_calls::CallNativeTarget;
    use crate::runtime::compiled::native_loop::NativeLoopOps;
    use crate::runtime::compiled::numeric::NumericValues;
    use crate::runtime::execution::Domain;
    use crate::runtime::graph::activation::{
        Activation, CallFrame, Execution, Progress, RootExit, Storage,
    };
    use crate::runtime::graph::{BlockEnvironment, GraphPosition, RetainedValues};
    use crate::runtime::state::RuntimeState;
    use crate::runtime::{
        BorrowedValue, EvaluatedValue, ExecutableRuntimePlan, HostCallOrigin, RuntimeListStorage,
        StoredRuntimeValue,
    };
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostProvider, HostProviderModule,
        HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile,
    };
    use num_bigint::BigInt;
    use std::num::NonZeroUsize;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn completed_native_workspace_returns_to_its_original_frame_without_replay() {
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
            Ok(call.return_value(value + 1))
        }

        // This fixture carries the actual native result through delivery to
        // its original return destination. It never replays the source body.
        struct CompletedNative {
            returned: StoredRuntimeValue,
            drops: Arc<AtomicUsize>,
        }
        impl Drop for CompletedNative {
            fn drop(&mut self) {
                self.drops.fetch_add(1, Ordering::SeqCst);
            }
        }
        impl CallExecution for CompletedNative {
            fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
                false
            }
            fn retained_bytes(&self) -> usize {
                std::mem::size_of::<Self>()
            }
            fn advance(self: Box<Self>, _: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                *budget -= 1;
                CallProgress::Complete {
                    output: CallOutput::Int(CallInteger(
                        BorrowedValue::from_stored(&self.returned).int().clone(),
                    )),
                    execution: self,
                }
            }
        }
        let source = r#"
@external(erlang, "native", "observe")
fn observe(value: Int) -> Int
fn cycle(counter: Int, producer: fn() -> Int) -> Int {
  let returned = observe(producer())
  case counter { 1 -> returned _ -> cycle(counter - 1, producer) }
}
fn captured(value: Int) { fn() { value } }
pub fn main() { #(cycle, captured(7)) }
"#;
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
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
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores, captures) = hosted.parts_mut();
        let mut graphs = Vec::new();
        let mut natives = Vec::new();
        for index in 0..3 {
            let function = IntFunctionId(index);
            match plan.int_function(function).as_ref() {
                ExecutionFunctionRef::Graph(entry) => graphs.push((function, entry.body())),
                ExecutionFunctionRef::Host(_) => natives.push(function),
            }
        }
        assert_eq!(graphs.len(), 2);
        assert_eq!(natives.len(), 1);
        let (function, body) = graphs[0];
        let contract = NativeLoopContract::inspect(body.block_graph()).unwrap();
        assert_eq!(contract.site.function(), "cycle");
        assert_eq!(contract.native, NativeLoopTarget::Int(natives[0]));
        let graph = body.block_graph().as_view();
        for budget in [1, 7, 1024] {
            calls.store(0, Ordering::SeqCst);
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
                NonZeroUsize::new(budget).unwrap(),
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
                let mut native_inputs = RetainedValues::empty();
                native_inputs.push_int(7.into());
                let canonical_output = context
                    .call(natives[0], HostCallOrigin::Entry, native_inputs)
                    .await
                    .unwrap()
                    .unwrap()
                    .into_bigint();
                assert_eq!(canonical_output, BigInt::from(8));
                assert_eq!(calls.load(Ordering::SeqCst), 0);
                let mut pending_inputs = RetainedValues::empty();
                pending_inputs.push_int(1.into());
                pending_inputs.push_evaluated(values[1].clone());
                let rejected = catch_unwind(AssertUnwindSafe(|| {
                    completed(Progress::Continue(Execution::new(
                        graph,
                        pending_inputs,
                        None,
                    )))
                }));
                let failure = rejected.err().unwrap();
                assert_eq!(
                    failure.downcast_ref::<&str>(),
                    Some(&"native workspace fixture must have completed"),
                );
                assert_eq!(calls.load(Ordering::SeqCst), 0);
                let binding = plan.bind_native_loop(&contract).unwrap();
                let returned = (binding.native)(&StoredRuntimeValue::new(
                    EvaluatedValue::Int(7.into()),
                    plan.value_metadata(),
                ))
                .unwrap();
                let drops = Arc::new(AtomicUsize::new(0));
                let workspace = Box::new(CompletedNative {
                    returned,
                    drops: Arc::clone(&drops),
                });
                assert_eq!(
                    workspace.retained_bytes(),
                    std::mem::size_of::<CompletedNative>()
                );
                let mut allowance = budget;
                let progress = NativeCallsMachine::new(
                    CallNativeTarget::select(contract.native).unwrap(),
                    NumericValues::default(),
                    workspace,
                )
                .run(
                    context.captures(),
                    &RuntimeListStorage::default(),
                    plan.value_metadata(),
                    NativeLoopOps {
                        native: binding.native.as_ref(),
                        unit: None,
                    },
                    &mut allowance,
                )
                .unwrap()
                .unwrap();
                assert_eq!(allowance, budget - 1);
                let mut position = GraphPosition::new(graph.entry(), inputs);
                // Entry has transferred these arguments to the workspace;
                // canonical delivery stores only its actual completed output.
                position.environment.clear_call_values();
                let execution = Execution {
                    active: Activation::NativeCalls(Box::new(NativeCallsExecution {
                        frame: CallFrame {
                            graph,
                            position,
                            exit: Box::new(RootExit),
                        },
                        state: NativeCallsState { binding, progress },
                    })),
                };
                let mut storage = Storage::new();
                let mut worker_echo = Vec::new();
                let mut state = RuntimeState::with_host_storage(
                    &mut worker_echo,
                    (),
                    RuntimeListStorage::default(),
                    context.captures().clone(),
                );
                let mut progress = execution
                    .advance(&**plan, &mut state, &mut storage, &mut 0)
                    .unwrap();
                while let Progress::Continue(next) = progress {
                    progress = next
                        .advance(&**plan, &mut state, &mut storage, &mut 0)
                        .unwrap();
                }
                let completed = completed(progress);
                assert_eq!(completed.0.into_bigint(), BigInt::from(8));
                assert_eq!(calls.load(Ordering::SeqCst), 1);
                assert_eq!(drops.load(Ordering::SeqCst), 0);
                let mut inputs = RetainedValues::empty();
                inputs.push_int(1.into());
                inputs.push_evaluated(values[1].clone());
                let environment = BlockEnvironment::from_retained(inputs);
                assert!(
                    storage
                        .function_calls
                        .reuse(CallTarget::Int(function), 0, environment.call_inputs(),)
                        .is_none()
                );
                assert_eq!(drops.load(Ordering::SeqCst), 0);
                drop(storage);
                assert_eq!(drops.load(Ordering::SeqCst), 1);
                assert!(worker_echo.is_empty());
            }))
            .unwrap();
            assert!(echo.is_empty());
        }
    }

    fn completed(progress: Progress<'_, HostedProgram<StatelessHostProfile>>) -> CallInteger {
        match progress {
            Progress::CallComplete(CallOutput::Int(completed)) => completed,
            _ => panic!("native workspace fixture must have completed"),
        }
    }
}
