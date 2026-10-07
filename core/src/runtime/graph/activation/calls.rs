use super::{
    Activation, Frame, FunctionContinuation, ReturnDestination, Returns, Storage, enter_function,
};
use crate::plan::execution::compiled::{CompiledCheckpoint, CompiledImplementation};
use crate::plan::execution::function::{ExecutionFunctionRef, FunctionBodyOwner};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::captures::ExecutionDomain;
use crate::runtime::compiled::calls::{
    BoolCallable, CallExecution, CallInteger, CallOps, CallProgress, CallResume, CallValues,
    IntCallable,
};
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::function::EntryTarget;
use crate::runtime::graph::{CompletedGraph, GraphPosition, GraphValue, RuntimeGraphState};

struct GeneratedDestination<'plan, Plan: ExecutableRuntimePlan, Value> {
    frame: Frame<'plan, Plan>,
    domain: Option<ExecutionDomain>,
    resume: CallResume<Value>,
}

impl<'plan, Plan: ExecutableRuntimePlan, Value: Send + 'static> ReturnDestination<'plan, Plan>
    for GeneratedDestination<'plan, Plan, Value>
{
    type Value = Value;

    fn domain(&self) -> Option<ExecutionDomain> {
        self.domain
    }

    fn resume(self, _returns: &mut Returns<'plan, Plan>, value: Value) -> Activation<'plan, Plan> {
        Activation::FunctionCalls {
            frame: self.frame,
            execution: (self.resume)(value),
        }
    }
}

pub(super) fn advance<'plan, Plan: ExecutableRuntimePlan>(
    mut frame: Frame<'plan, Plan>,
    implementation: &'plan CompiledImplementation,
    point: usize,
    plan: &'plan Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    storage: &mut Storage<'plan, Plan>,
    remaining: &mut usize,
) -> ExecutionResult<Activation<'plan, Plan>> {
    if let CompiledImplementation::FunctionCalls(calls) = implementation
        && let Some(execution) = (calls.start)(
            point,
            frame.position.environment.call_inputs(),
            &mut storage.function_calls,
        )
    {
        frame.position.environment.clear_call_values();
        return resume(frame, execution, plan, state, storage, remaining);
    }
    frame.advance(plan, state, storage, remaining)
}

pub(super) fn resume<'plan, Plan: ExecutableRuntimePlan>(
    frame: Frame<'plan, Plan>,
    execution: Box<dyn CallExecution>,
    plan: &'plan Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    storage: &mut Storage<'plan, Plan>,
    remaining: &mut usize,
) -> ExecutionResult<Activation<'plan, Plan>> {
    let mut budget = *remaining + 1;
    let progress = execution.advance(
        &mut CallOps::new(state.captures(), &mut storage.numeric, state.lists()),
        &mut budget,
    );
    *remaining = budget;
    resume_progress(frame, progress, plan, state, storage)
}

pub(super) fn resume_progress<'plan, Plan: ExecutableRuntimePlan>(
    mut frame: Frame<'plan, Plan>,
    progress: CallProgress,
    plan: &'plan Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    storage: &mut Storage<'plan, Plan>,
) -> ExecutionResult<Activation<'plan, Plan>> {
    let domain = Some(state.captures().domain());
    macro_rules! call {
        ($function:expr, $site:expr, $arguments:expr, $resume:expr, $map:expr) => {
            enter_function(
                plan,
                $function,
                HostCallOrigin::source($site),
                $arguments.into_retained(),
                GeneratedDestination {
                    frame,
                    domain,
                    resume: $resume,
                },
                $map,
            )
        };
    }
    macro_rules! interpreted {
        ($function:expr, $site:expr, $point:expr, $values:expr, $resume:expr, $map:expr) => {
            enter_interpreted(
                plan,
                $function,
                HostCallOrigin::source($site),
                $point,
                $values,
                GeneratedDestination {
                    frame,
                    domain,
                    resume: $resume,
                },
                $map,
            )
        };
    }
    Ok(match progress {
        CallProgress::Yield(execution) => Activation::FunctionCalls { frame, execution },
        CallProgress::Complete {
            exit,
            output,
            execution,
        } => {
            storage.function_calls.recycle(execution);
            frame.position.environment.store_call_output(output);
            return frame.exit.exit(
                CompletedGraph {
                    exit,
                    environment: frame.position.environment,
                    direct_return: true,
                },
                storage,
            );
        }
        CallProgress::Interpreted { point, values } => {
            frame.position.environment.restore_call_values(values);
            frame.position.block = point.block;
            frame.position.instruction = point.instruction;
            Activation::Graph(frame)
        }
        CallProgress::Int {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, |value| Ok(CallInteger(
            value
        ))),
        CallProgress::Bool {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, Ok),
        CallProgress::IntFunction {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, |value| Ok(IntCallable(
            value
        ))),
        CallProgress::BoolFunction {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, |value| Ok(BoolCallable(
            value
        ))),
        CallProgress::InterpretedInt {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, |value| Ok(
            CallInteger(value)
        )),
        CallProgress::InterpretedBool {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, Ok),
        CallProgress::InterpretedIntFunction {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, |value| Ok(
            IntCallable(value)
        )),
        CallProgress::InterpretedBoolFunction {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, |value| Ok(
            BoolCallable(value)
        )),
    })
}

fn enter_interpreted<'plan, Plan, Id, Value>(
    plan: &'plan Plan,
    id: Id,
    origin: HostCallOrigin,
    point: CompiledCheckpoint,
    values: CallValues,
    destination: GeneratedDestination<'plan, Plan, Value>,
    map: impl FnOnce(
        <<Id::Body as FunctionBodyOwner>::Return as GraphValue>::Evaluated,
    ) -> ExecutionResult<Value>
    + Send
    + 'plan,
) -> Activation<'plan, Plan>
where
    Plan: ExecutableRuntimePlan,
    Id: EntryTarget<Plan> + 'plan,
    Id::Body: 'plan,
    Value: Send + 'static,
{
    let inputs = values.into_retained();
    match id.entry(plan) {
        ExecutionFunctionRef::Graph(function) => {
            let graph = function.body().function_body().block_graph().as_view();
            let mut position = GraphPosition::new(graph.entry(), inputs);
            position.block = point.block;
            position.instruction = point.instruction;
            Activation::Graph(Frame {
                graph,
                position,
                exit: Box::new(FunctionContinuation {
                    plan,
                    id,
                    body: function.body(),
                    destination,
                    map,
                }),
            })
        }
        ExecutionFunctionRef::Host(_) => enter_function(plan, id, origin, inputs, destination, map),
    }
}

#[cfg(test)]
mod tests {
    use super::{Activation, GeneratedDestination, enter_interpreted};
    use crate::plan::execution::compiled::{CallTarget, CompiledCheckpoint};
    use crate::plan::execution::function::{
        BoolFunctionFunctionId, BoolFunctionId, IntFunctionFunctionId, IntFunctionId,
    };
    use crate::plan::execution::graph::{BlockGraphExitId, BlockId, IntLocalId};
    use crate::plan::{HostCallSite, SourceSpan};
    use crate::runtime::compiled::calls::{
        BoolCallable, CallArguments, CallExecution, CallInputs, CallInteger, CallOps, CallOutput,
        CallProgress, CallValues, IntCallable,
    };
    use crate::runtime::error::HostCallOrigin;
    use crate::runtime::graph::RetainedValues;
    use crate::runtime::graph::activation::tests::int_body;
    use crate::runtime::graph::activation::{
        Execution, Frame, GraphPosition, Progress, RootExit, Storage,
    };
    use crate::runtime::integer::IntegerValue;
    use crate::runtime::state::RuntimeState;
    use crate::{
        HostProviderModule, HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile,
    };
    use num_bigint::BigInt;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn canonical_call_boundaries_resume_each_typed_family_once_in_the_original_execution() {
        let source = r#"
fn integer() -> Int { 42 }
fn boolean() -> Bool { True }
fn integer_function() -> fn() -> Int { integer }
fn boolean_function() -> fn() -> Bool { boolean }
pub fn main() {
  let number = integer_function()
  let predicate = boolean_function()
  case predicate() { True -> number() False -> 0 }
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let root = int_body(&plan, IntFunctionId(0)).block_graph().as_view();
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
        let site = HostCallSite::from_static("example", "main", SourceSpan::new(0, source.len()));

        // The owner fixture supplies only the two protocol boundaries. The
        // selected source function runs through its real canonical graph.
        struct Boundary(CallProgress);
        impl CallExecution for Boundary {
            fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
                false
            }

            fn retained_bytes(&self) -> usize {
                0
            }

            fn advance(self: Box<Self>, _: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                *budget -= 1;
                self.0
            }
        }
        struct Finish;
        impl CallExecution for Finish {
            fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
                false
            }

            fn retained_bytes(&self) -> usize {
                0
            }

            fn advance(self: Box<Self>, _: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                *budget -= 1;
                CallProgress::Complete {
                    exit: BlockGraphExitId(0),
                    output: CallOutput::Int(42_i128.into()),
                    execution: self,
                }
            }
        }
        let mut completion = Finish;
        assert!(!completion.restart(
            CallTarget::Int(IntFunctionId(0)),
            0,
            CallInputs::new(&[], &[], &[], &[], &[]),
        ));
        assert_eq!(completion.retained_bytes(), 0);
        for interpreted in [false, true] {
            let mapped = Arc::new(AtomicUsize::new(0));
            let integer_count = mapped.clone();
            let integer = Box::new(move |value: CallInteger| -> Box<dyn CallExecution> {
                assert_eq!(value.0, IntegerValue::from(42_i64));
                integer_count.fetch_add(1, Ordering::SeqCst);
                Box::new(Finish)
            });
            let boolean_count = mapped.clone();
            let boolean = Box::new(move |value: bool| -> Box<dyn CallExecution> {
                assert!(value);
                boolean_count.fetch_add(1, Ordering::SeqCst);
                Box::new(Finish)
            });
            let integer_function_count = mapped.clone();
            let integer_function = Box::new(move |value: IntCallable| -> Box<dyn CallExecution> {
                assert_eq!(value.0.runtime_id(), IntFunctionId(1));
                integer_function_count.fetch_add(1, Ordering::SeqCst);
                Box::new(Finish)
            });
            let boolean_function_count = mapped.clone();
            let boolean_function = Box::new(move |value: BoolCallable| -> Box<dyn CallExecution> {
                assert_eq!(value.0.runtime_id(), BoolFunctionId(0));
                boolean_function_count.fetch_add(1, Ordering::SeqCst);
                Box::new(Finish)
            });
            let progress = if interpreted {
                [
                    CallProgress::InterpretedInt {
                        function: IntFunctionId(1),
                        site: site.clone(),
                        point,
                        values: CallValues::default(),
                        resume: integer,
                    },
                    CallProgress::InterpretedBool {
                        function: BoolFunctionId(0),
                        site: site.clone(),
                        point,
                        values: CallValues::default(),
                        resume: boolean,
                    },
                    CallProgress::InterpretedIntFunction {
                        function: IntFunctionFunctionId(0),
                        site: site.clone(),
                        point,
                        values: CallValues::default(),
                        resume: integer_function,
                    },
                    CallProgress::InterpretedBoolFunction {
                        function: BoolFunctionFunctionId(0),
                        site: site.clone(),
                        point,
                        values: CallValues::default(),
                        resume: boolean_function,
                    },
                ]
            } else {
                [
                    CallProgress::Int {
                        function: IntFunctionId(1),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: CallValues::default(),
                            captures: None,
                        },
                        resume: integer,
                    },
                    CallProgress::Bool {
                        function: BoolFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: CallValues::default(),
                            captures: None,
                        },
                        resume: boolean,
                    },
                    CallProgress::IntFunction {
                        function: IntFunctionFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: CallValues::default(),
                            captures: None,
                        },
                        resume: integer_function,
                    },
                    CallProgress::BoolFunction {
                        function: BoolFunctionFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: CallValues::default(),
                            captures: None,
                        },
                        resume: boolean_function,
                    },
                ]
            };
            for progress in progress {
                let before = mapped.load(Ordering::SeqCst);
                let mut boundary = Boundary(progress);
                assert!(!boundary.restart(
                    CallTarget::Int(IntFunctionId(0)),
                    0,
                    CallInputs::new(&[], &[], &[], &[], &[]),
                ));
                assert_eq!(boundary.retained_bytes(), 0);
                let mut execution = Execution {
                    active: Activation::FunctionCalls {
                        frame: Frame {
                            graph: root,
                            position: GraphPosition::new(BlockId(0), RetainedValues::empty()),
                            exit: Box::new(RootExit),
                        },
                        execution: Box::new(boundary),
                    },
                };
                let mut storage = Storage::new();
                let mut echo = Vec::new();
                let mut state = RuntimeState::new(&mut echo);
                let completed = loop {
                    match execution
                        .advance(&plan, &mut state, &mut storage, &mut 0)
                        .unwrap()
                    {
                        Progress::Continue(next) => execution = next,
                        Progress::Complete(completed) => break completed,
                        Progress::Host(never) => match never {},
                    }
                };
                assert_eq!(completed.exit(), BlockGraphExitId(0));
                assert_eq!(
                    completed.into_value(&IntLocalId(0)),
                    IntegerValue::from(42_i64)
                );
                assert_eq!(mapped.load(Ordering::SeqCst), before + 1);
                assert!(echo.is_empty());
            }
            assert_eq!(mapped.load(Ordering::SeqCst), 4);
        }
    }

    fn forbidden_resume(_: IntegerValue) -> Box<dyn CallExecution> {
        panic!("entry handoff must not resume or poll the selected function");
    }

    #[test]
    #[should_panic(expected = "entry handoff must not resume or poll the selected function")]
    fn entry_fixture_guard_rejects_an_eager_resume() {
        forbidden_resume(42_i64.into());
    }

    #[test]
    fn canonical_entry_keeps_graph_checkpoints_and_native_handoff_unpolled() {
        let source = r#"
@external(erlang, "native", "answer")
fn native_answer() -> Int
fn graph_answer() -> Int { 42 }
pub fn main() { graph_answer() + native_answer() }
"#;
        let called = Arc::new(AtomicUsize::new(0));
        let observed = called.clone();
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::from_providers([HostProviderModule::<StatelessHostProfile>::new(
                "example", "example",
            )
            .unwrap()
            .with_function::<(), BigInt, _>("native_answer", move || {
                observed.fetch_add(1, Ordering::SeqCst);
                BigInt::from(42)
            })
            .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = execution.parts_mut();
        let plan = &**plan;
        let root = int_body(plan, IntFunctionId(0)).block_graph().as_view();
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
        // The same entry owner accepts graph and native IDs. Generated graph
        // checkpoints and native submission remain different transitions.
        for (function, native) in [(IntFunctionId(1), false), (IntFunctionId(2), true)] {
            let active = enter_interpreted(
                plan,
                function,
                HostCallOrigin::Entry,
                point,
                CallValues::default(),
                GeneratedDestination {
                    frame: Frame {
                        graph: root,
                        position: GraphPosition::new(BlockId(0), RetainedValues::empty()),
                        exit: Box::new(RootExit),
                    },
                    domain: None,
                    resume: Box::new(forbidden_resume),
                },
                Ok,
            );
            assert_eq!(matches!(&active, Activation::Host(_)), native);
            assert_eq!(called.load(Ordering::SeqCst), 0);
            drop(active);
            assert_eq!(called.load(Ordering::SeqCst), 0);
        }
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        assert_eq!(
            host.block_on(execution.run_main(&host, &mut (), &mut echo))
                .unwrap()
                .try_into_value()
                .unwrap(),
            crate::Value::Int(84.into())
        );
        assert_eq!(called.load(Ordering::SeqCst), 1);
        assert!(echo.is_empty());
    }
}
