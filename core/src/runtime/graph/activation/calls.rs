use super::{
    Activation, CallFrame, Frame, FunctionContinuation, ReturnDestination, Returns, Storage,
    enter_function,
};
use crate::plan::execution::compiled::{CompiledCheckpoint, FunctionCallsImplementation};
use crate::plan::execution::function::{ExecutionFunctionRef, FunctionBodyOwner};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::captures::ExecutionDomain;
use crate::runtime::compiled::calls::{
    BitArrayCallable, BoolCallable, CallBitArray, CallExecution, CallInteger, CallOps,
    CallProgress, CallResume, CallValues, FloatCallable, GeneratedNativePhase,
    GeneratedNativeState, IntCallable, NilCallable, StringCallable, UtfCodepointCallable,
};
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::function::EntryTarget;
use crate::runtime::graph::{GraphPosition, GraphValue, RuntimeGraphState};

pub(super) enum CallEntry<'plan, Plan: ExecutableRuntimePlan> {
    Running(CallFrame<'plan, Plan>, Box<dyn CallExecution>),
    Canonical(CallFrame<'plan, Plan>),
}

struct GeneratedDestination<'plan, Plan: ExecutableRuntimePlan, Value> {
    frame: CallFrame<'plan, Plan>,
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
    frame: CallFrame<'plan, Plan>,
    calls: &'plan FunctionCallsImplementation,
    point: usize,
    plan: &'plan Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    storage: &mut Storage<'plan, Plan>,
    remaining: &mut usize,
) -> ExecutionResult<Activation<'plan, Plan>> {
    advance_entry(
        enter(frame, calls, point, storage),
        plan,
        state,
        storage,
        remaining,
    )
}

pub(super) fn enter<'plan, Plan: ExecutableRuntimePlan>(
    mut frame: CallFrame<'plan, Plan>,
    calls: &'plan FunctionCallsImplementation,
    point: usize,
    storage: &mut Storage<'plan, Plan>,
) -> CallEntry<'plan, Plan> {
    if let Some(execution) = (calls.start)(
        point,
        frame.position.environment.call_inputs(),
        &mut storage.function_calls,
    ) {
        frame.position.environment.clear_call_values();
        CallEntry::Running(frame, execution)
    } else {
        CallEntry::Canonical(frame)
    }
}

pub(super) fn advance_entry<'plan, Plan: ExecutableRuntimePlan>(
    entry: CallEntry<'plan, Plan>,
    plan: &'plan Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    storage: &mut Storage<'plan, Plan>,
    remaining: &mut usize,
) -> ExecutionResult<Activation<'plan, Plan>> {
    match entry {
        CallEntry::Running(frame, execution) => {
            resume(frame, execution, plan, state, storage, remaining)
        }
        CallEntry::Canonical(frame) => frame.into_graph().advance(plan, state, storage, remaining),
    }
}

pub(super) fn resume<'plan, Plan: ExecutableRuntimePlan>(
    frame: CallFrame<'plan, Plan>,
    execution: Box<dyn CallExecution>,
    plan: &'plan Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    storage: &mut Storage<'plan, Plan>,
    remaining: &mut usize,
) -> ExecutionResult<Activation<'plan, Plan>> {
    let mut budget = *remaining + 1;
    let progress = execution.advance(
        &mut CallOps::new(
            state.captures(),
            &mut storage.numeric,
            state.lists(),
            &mut storage.string,
            &mut storage.bit_array_loop,
        )
        .with_root_tail_entry(frame.exit.root_tail_entry())
        .with_synchronous_strings(plan.synchronous_strings()),
        &mut budget,
    );
    *remaining = budget;
    finish_progress(frame, progress, plan, storage, state.captures().domain())
}

pub(super) fn resume_progress<'plan, Plan: ExecutableRuntimePlan>(
    frame: CallFrame<'plan, Plan>,
    progress: CallProgress,
    plan: &'plan Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    storage: &mut Storage<'plan, Plan>,
) -> ExecutionResult<Activation<'plan, Plan>> {
    finish_progress(frame, progress, plan, storage, state.captures().domain())
}

fn finish_progress<'plan, Plan: ExecutableRuntimePlan>(
    frame: CallFrame<'plan, Plan>,
    progress: CallProgress,
    plan: &'plan Plan,
    storage: &mut Storage<'plan, Plan>,
    domain: ExecutionDomain,
) -> ExecutionResult<Activation<'plan, Plan>> {
    macro_rules! call {
        ($function:expr, $site:expr, $arguments:expr, $resume:expr, $map:expr) => {
            enter_function(
                plan,
                $function,
                HostCallOrigin::source($site),
                $arguments.into_retained(),
                GeneratedDestination {
                    frame,
                    domain: Some(domain),
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
                    domain: Some(domain),
                    resume: $resume,
                },
                $map,
            )
        };
    }
    Ok(match progress {
        CallProgress::StringNative(request) => {
            let root_tail_entry = frame.exit.root_tail_entry();
            // Every generated request retains its original capture domain
            // across the service grant and the eventual return.
            Activation::GeneratedNative {
                frame,
                state: Box::new(GeneratedNativeState {
                    phase: GeneratedNativePhase::Progress(CallProgress::StringNative(request)),
                    numeric: std::mem::take(&mut storage.numeric),
                    strings: storage.string.take(),
                    bit_arrays: storage.bit_array_loop.take(),
                    root_tail_entry,
                    domain,
                    prepaid_completion: false,
                }),
            }
        }
        CallProgress::Yield(execution) => Activation::FunctionCalls { frame, execution },
        CallProgress::Complete { output, execution } => {
            storage.function_calls.recycle(execution);
            return frame
                .exit
                .complete(output, frame.position.environment, storage);
        }
        CallProgress::Interpreted {
            target,
            point,
            values,
        } => {
            return frame.exit.interpreted(
                target,
                point,
                values,
                frame.position.environment,
                storage,
            );
        }
        CallProgress::Int {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, |value| Ok(CallInteger(
            value
        ))),
        CallProgress::IntScalar {
            function,
            site,
            input,
            resume,
        } => call!(function, site, input, resume, |value| Ok(CallInteger(
            value
        ))),
        CallProgress::Bool {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, Ok),
        CallProgress::BoolScalar {
            function,
            site,
            input,
            resume,
        } => call!(function, site, input, resume, Ok),
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
        CallProgress::Float {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, Ok),
        CallProgress::FloatFunction {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, |value| Ok(
            FloatCallable(value)
        )),
        CallProgress::InterpretedFloat {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, Ok),
        CallProgress::InterpretedFloatFunction {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, |value| Ok(
            FloatCallable(value)
        )),
        CallProgress::String {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, Ok),
        CallProgress::StringFunction {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, |value| Ok(
            StringCallable(value)
        )),
        CallProgress::InterpretedString {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, Ok),
        CallProgress::InterpretedStringFunction {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, |value| Ok(
            StringCallable(value)
        )),
        CallProgress::BitArray {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, |value| Ok(CallBitArray(
            value
        ))),
        CallProgress::BitArrayFunction {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, |value| Ok(
            BitArrayCallable(value)
        )),
        CallProgress::InterpretedBitArray {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, |value| Ok(
            CallBitArray(value)
        )),
        CallProgress::InterpretedBitArrayFunction {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, |value| Ok(
            BitArrayCallable(value)
        )),
        CallProgress::UtfCodepoint {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, Ok),
        CallProgress::UtfCodepointFunction {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, |value| Ok(
            UtfCodepointCallable(value)
        )),
        CallProgress::InterpretedUtfCodepoint {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, Ok),
        CallProgress::InterpretedUtfCodepointFunction {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, |value| Ok(
            UtfCodepointCallable(value)
        )),
        CallProgress::Nil {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, Ok),
        CallProgress::NilFunction {
            function,
            site,
            arguments,
            resume,
        } => call!(function, site, arguments, resume, |value| Ok(NilCallable(
            value
        ))),
        CallProgress::InterpretedNil {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, Ok),
        CallProgress::InterpretedNilFunction {
            function,
            site,
            point,
            values,
            resume,
        } => interpreted!(function, site, point, values, resume, |value| Ok(
            NilCallable(value)
        )),
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
    values: Box<CallValues>,
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

fn bridge_native<'plan, Plan: ExecutableRuntimePlan>(
    frame: CallFrame<'plan, Plan>,
    request: crate::runtime::compiled::calls::StringNativeRequest,
    plan: &'plan Plan,
    domain: ExecutionDomain,
) -> Activation<'plan, Plan> {
    use crate::runtime::compiled::calls::CallArguments;
    enter_function(
        plan,
        request.function,
        HostCallOrigin::source(request.site),
        CallArguments {
            values: request.arguments,
            captures: None,
        }
        .into_retained(),
        GeneratedDestination {
            frame,
            domain: Some(domain),
            resume: Box::new(move |value| request.execution.resume_native(value)),
        },
        Ok,
    )
}

pub(super) fn finish_native<'plan, Plan: ExecutableRuntimePlan>(
    frame: CallFrame<'plan, Plan>,
    state: Box<GeneratedNativeState>,
    plan: &'plan Plan,
    storage: &mut Storage<'plan, Plan>,
    declined: bool,
) -> ExecutionResult<Activation<'plan, Plan>> {
    if !declined
        && (!matches!(&state.phase, GeneratedNativePhase::Progress(_))
            || matches!(
                &state.phase,
                GeneratedNativePhase::Progress(CallProgress::Complete { .. })
            ) && !state.prepaid_completion)
    {
        return Ok(Activation::GeneratedNative { frame, state });
    }
    let GeneratedNativeState {
        phase,
        numeric,
        strings,
        bit_arrays,
        domain,
        prepaid_completion: _,
        root_tail_entry: _,
    } = *state;
    storage.numeric = numeric;
    storage.string = strings;
    storage.bit_array_loop = bit_arrays;
    let active = match phase {
        GeneratedNativePhase::Progress(CallProgress::StringNative(request)) if declined => {
            bridge_native(frame, request, plan, domain)
        }
        // A non-declined Invoke is retained above with its original owner.
        GeneratedNativePhase::Invoke { request, .. } => bridge_native(frame, request, plan, domain),
        GeneratedNativePhase::Progress(progress) => {
            finish_progress(frame, progress, plan, storage, domain)?
        }
        GeneratedNativePhase::Deliver {
            value, execution, ..
        } => Activation::FunctionCalls {
            frame,
            execution: execution.resume_native(value),
        },
    };
    Ok(active)
}

#[cfg(test)]
mod tests {
    use super::{Activation, GeneratedDestination, enter_interpreted};
    use crate::plan::execution::compiled::{CallTarget, CompiledCheckpoint};
    use crate::plan::execution::function::{
        BitArrayFunctionFunctionId, BitArrayFunctionId, FloatFunctionFunctionId, FloatFunctionId,
        NilFunctionFunctionId, NilFunctionId, StringFunctionFunctionId, StringFunctionId,
        UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    };
    use crate::plan::execution::function::{
        BoolFunctionFunctionId, BoolFunctionId, IntFunctionFunctionId, IntFunctionId,
    };
    use crate::plan::execution::graph::{BlockId, IntLocalId};
    use crate::plan::{HostCallSite, SourceSpan};
    use crate::runtime::compiled::calls::{
        BitArrayCallable, CallBitArray, FloatCallable, NilCallable, StringCallable,
        UtfCodepointCallable,
    };
    use crate::runtime::compiled::calls::{
        BoolCallable, CallArguments, CallExecution, CallInputs, CallInteger, CallNativeInput,
        CallOps, CallOutput, CallProgress, IntCallable,
    };
    use crate::runtime::error::HostCallOrigin;
    use crate::runtime::graph::BlockEnvironment;
    use crate::runtime::graph::GraphValue;
    use crate::runtime::graph::RetainedValues;
    use crate::runtime::graph::activation::tests::int_body;
    use crate::runtime::graph::activation::{
        CallFrame, Execution, GraphPosition, Progress, RootExit, Storage,
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
    fn native_phases_retain_pending_owners_and_bridge_declined_calls_without_polling() {
        use crate::StringValue;
        use crate::plan::execution::function::StringFunctionId;
        use crate::runtime::ExecutableRuntimePlan;
        use crate::runtime::compiled::calls::{
            GeneratedNativePhase, GeneratedNativeState, StringNativeExecution, StringNativeRequest,
        };
        use std::ptr;

        struct Completion;
        impl CallExecution for Completion {
            fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
                false
            }
            fn retained_bytes(&self) -> usize {
                0
            }
            fn advance(self: Box<Self>, _: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                *budget -= 1;
                CallProgress::Complete {
                    output: CallOutput::Int(42_i128.into()),
                    execution: self,
                }
            }
        }
        impl StringNativeExecution for Completion {
            fn resume_native(self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> {
                assert_eq!(value.as_str(), Ok("input!"));
                self
            }
        }
        let source = r#"
@external(erlang, "example", "append")
fn append(value: String) -> String
pub fn main() { let _ = append("input") 42 }
"#;
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = calls.clone();
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
            .with_function::<(StringValue,), StringValue, _>("append", move |value: StringValue| {
                observed.fetch_add(1, Ordering::SeqCst);
                format!("{}!", value.as_str().unwrap()).into()
            })
            .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, captures) = hosted.parts_mut();
        let plan = &**plan;
        let graph = int_body(plan, IntFunctionId(0)).block_graph().as_view();
        let frame = || CallFrame {
            graph,
            position: GraphPosition::new(BlockId(0), RetainedValues::empty()),
            exit: Box::new(RootExit),
        };
        let request = || StringNativeRequest {
            function: StringFunctionId(0),
            site: HostCallSite::from_static("example", "main", SourceSpan::new(0, source.len())),
            root_tail: false,
            arguments: Box::new(crate::runtime::compiled::calls::CallValues {
                strings: vec!["input".into()],
                ..Default::default()
            }),
            execution: Box::new(Completion),
        };
        let mut completion = Completion;
        assert!(!completion.restart(
            CallTarget::Int(IntFunctionId(0)),
            0,
            CallInputs::new(&BlockEnvironment::from_retained(RetainedValues::empty()))
        ));
        assert_eq!(completion.retained_bytes(), 0);
        for declined in [false, true] {
            for phase in [
                GeneratedNativePhase::Progress(CallProgress::StringNative(request())),
                GeneratedNativePhase::Invoke {
                    request: request(),
                    before: 1,
                },
                GeneratedNativePhase::Deliver {
                    value: "input!".into(),
                    execution: Box::new(Completion),
                    charge: true,
                },
            ] {
                let pending = !declined && !matches!(phase, GeneratedNativePhase::Progress(_));
                let delivered = matches!(phase, GeneratedNativePhase::Deliver { .. });
                let state = Box::new(GeneratedNativeState {
                    phase,
                    numeric: Default::default(),
                    strings: None,
                    bit_arrays: None,
                    root_tail_entry: true,
                    domain: captures.domain(),
                    prepaid_completion: false,
                });
                let owner = ptr::from_ref(&*state);
                let mut storage = Storage::new();
                let active =
                    super::finish_native(frame(), state, plan, &mut storage, declined).unwrap();
                assert_eq!(
                    matches!(&active, Activation::Host(_)),
                    declined && !delivered
                );
                assert_eq!(
                    matches!(&active, Activation::GeneratedNative { .. }),
                    !declined
                );
                if pending {
                    assert_eq!(native_owner(&active), owner);
                } else if delivered {
                    let mut echo = Vec::new();
                    let mut state = RuntimeState::new(&mut echo);
                    let (frame, execution) = generated_execution(active);
                    let completed =
                        super::resume(frame, execution, plan, &mut state, &mut storage, &mut 0)
                            .unwrap();
                    assert_eq!(completed_int(completed), IntegerValue::from(42_i64));
                    assert!(echo.is_empty());
                } else if !declined {
                    let mut echo = Vec::new();
                    let mut runtime = RuntimeState::new(&mut echo);
                    let progress = Execution { active }
                        .advance(plan, &mut runtime, &mut storage, &mut 0)
                        .unwrap();
                    let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        crate::runtime::graph::tests::canonical_progress(progress);
                    }))
                    .unwrap_err();
                    assert_eq!(
                        *rejected.downcast::<&str>().unwrap(),
                        "canonical graph fixture has no host calls"
                    );
                }
            }
        }
        for prepaid in [false, true] {
            let state = Box::new(GeneratedNativeState {
                phase: GeneratedNativePhase::Progress(CallProgress::Complete {
                    output: CallOutput::Int(42_i128.into()),
                    execution: Box::new(Completion),
                }),
                numeric: Default::default(),
                strings: None,
                bit_arrays: None,
                root_tail_entry: true,
                domain: captures.domain(),
                prepaid_completion: prepaid,
            });
            let owner = ptr::from_ref(&*state);
            let mut storage = Storage::new();
            let active = super::finish_native(frame(), state, plan, &mut storage, false).unwrap();
            if prepaid {
                assert_eq!(completed_int(active), IntegerValue::from(42_i64));
            } else {
                assert_eq!(native_owner(&active), owner);
            }
        }
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
        let state = Box::new(GeneratedNativeState {
            phase: GeneratedNativePhase::Progress(CallProgress::Interpreted {
                target: CallTarget::Int(IntFunctionId(0)),
                point,
                values: Box::default(),
            }),
            numeric: Default::default(),
            strings: None,
            bit_arrays: None,
            root_tail_entry: true,
            domain: captures.domain(),
            prepaid_completion: false,
        });
        let progress =
            super::super::Execution::generated_ready(frame(), state, plan, &mut Storage::new())
                .unwrap();
        let expected: super::super::Progress<
            '_,
            crate::plan::execution::HostedProgram<StatelessHostProfile>,
        > = super::super::Progress::CallInterpreted {
            target: CallTarget::Int(IntFunctionId(0)),
            point,
            values: Box::default(),
        };
        assert_eq!(
            std::mem::discriminant(&progress),
            std::mem::discriminant(&expected)
        );
        drop(progress);
        drop(expected);
        for (progress, completed) in [
            (
                CallProgress::Complete {
                    output: CallOutput::Int(42_i128.into()),
                    execution: Box::new(Completion),
                },
                true,
            ),
            (CallProgress::Yield(Box::new(Completion)), false),
        ] {
            let state = Box::new(GeneratedNativeState {
                phase: GeneratedNativePhase::Progress(progress),
                numeric: Default::default(),
                strings: None,
                bit_arrays: None,
                root_tail_entry: true,
                domain: captures.domain(),
                prepaid_completion: true,
            });
            let progress =
                Execution::generated_ready(frame(), state, plan, &mut Storage::new()).unwrap();
            assert_eq!(matches!(progress, Progress::CallComplete(_)), completed);
            assert_eq!(matches!(&progress, Progress::Continue(_)), !completed);
        }
        let mut echo = Vec::new();
        let mut runtime = RuntimeState::new(&mut echo);
        let progress = Execution {
            active: Activation::CallInterpreted {
                target: CallTarget::Int(IntFunctionId(0)),
                point,
                values: Box::default(),
            },
        }
        .advance(plan, &mut runtime, &mut Storage::new(), &mut 0)
        .unwrap();
        assert_eq!(
            std::mem::discriminant(&progress),
            std::mem::discriminant(&Progress::CallInterpreted {
                target: CallTarget::Int(IntFunctionId(0)),
                point,
                values: Box::default(),
            })
        );
        drop(progress);
        // A plain profile declines an already-owned completion without host
        // submission. The graph driver then delivers that exact result.
        let plain = crate::ExecutionPlan::from_module_plan(
            crate::plan_module(
                crate::compile_typed_module("example", "src/example.gleam", "fn append(value: String) { value <> \"!\" } pub fn main() { let _ = append(\"input\") 42 }")
                    .unwrap(),
            )
            .unwrap(),
        );
        let plain_root = int_body(&plain, IntFunctionId(0)).block_graph().as_view();
        let returned = Box::new(GeneratedNativeState {
            phase: GeneratedNativePhase::Progress(CallProgress::Complete {
                output: CallOutput::Int(42_i128.into()),
                execution: Box::new(Completion),
            }),
            numeric: Default::default(),
            strings: None,
            bit_arrays: None,
            root_tail_entry: true,
            domain: captures.domain(),
            prepaid_completion: true,
        });
        let owner = ptr::from_ref(&*returned);
        let returned = plain.prepare_generated_native(returned, 1).err().unwrap();
        assert_eq!(ptr::from_ref(&*returned), owner);
        let plain_frame = CallFrame {
            graph: plain_root,
            position: GraphPosition::new(BlockId(0), RetainedValues::empty()),
            exit: Box::new(RootExit),
        };
        let mut echo = Vec::new();
        let mut runtime = RuntimeState::new(&mut echo);
        let mut returned = Execution {
            active: Activation::GeneratedNative {
                frame: plain_frame,
                state: returned,
            },
        }
        .advance(&plain, &mut runtime, &mut Storage::new(), &mut 0)
        .unwrap();
        while let Progress::Continue(next) = returned {
            returned = next
                .advance(&plain, &mut runtime, &mut Storage::new(), &mut 0)
                .unwrap();
        }
        assert_eq!(
            generated_output(returned).unwrap(),
            IntegerValue::from(42_i64)
        );
        // A request naming an actual graph function stays canonical when the
        // profile has no native service. Its resumed completion still runs once.
        let state = Box::new(GeneratedNativeState {
            phase: GeneratedNativePhase::Invoke {
                request: request(),
                before: 1,
            },
            numeric: Default::default(),
            strings: None,
            bit_arrays: None,
            root_tail_entry: true,
            domain: runtime.captures().domain(),
            prepaid_completion: false,
        });
        let frame = CallFrame {
            graph: plain_root,
            position: GraphPosition::new(BlockId(0), RetainedValues::empty()),
            exit: Box::new(RootExit),
        };
        let mut storage = Storage::new();
        let mut progress = Execution {
            active: Activation::GeneratedNative { frame, state },
        }
        .advance(&plain, &mut runtime, &mut storage, &mut 0)
        .unwrap();
        while let Progress::Continue(next) = progress {
            progress = next
                .advance(&plain, &mut runtime, &mut storage, &mut 0)
                .unwrap();
        }
        assert_eq!(
            generated_output(progress).unwrap(),
            IntegerValue::from(42_i64)
        );
        // Both native-delivery entry points propagate the existing fallible
        // return mapper; they must not convert its error into a completion.
        use crate::plan::execution::function::FunctionReturnFamily;
        use crate::runtime::error::InvariantError;
        for ready in [false, true] {
            let mut storage = Storage::new();
            let destination = storage.returns.suspend(super::super::Frame {
                graph,
                position: GraphPosition::new(BlockId(0), RetainedValues::empty()),
                exit: Box::new(RootExit),
            });
            let failure = InvariantError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Int,
                actual: FunctionReturnFamily::Float,
            };
            let expected = failure.clone();
            let mapped = Arc::new(AtomicUsize::new(0));
            let observed = mapped.clone();
            let continuation = super::super::FunctionContinuation {
                plan,
                id: IntFunctionId(0),
                body: int_body(plan, IntFunctionId(0)),
                destination,
                map: move |value: IntegerValue| -> Result<IntegerValue, crate::ExecutionError> {
                    assert_eq!(value, IntegerValue::from(42_i64));
                    observed.fetch_add(1, Ordering::SeqCst);
                    Err(crate::ExecutionError::Invariant(failure))
                },
            };
            let frame = CallFrame {
                graph,
                position: GraphPosition::new(BlockId(0), RetainedValues::empty()),
                exit: Box::new(continuation),
            };
            let state = Box::new(GeneratedNativeState {
                phase: GeneratedNativePhase::Progress(CallProgress::Complete {
                    output: CallOutput::Int(42_i128.into()),
                    execution: Box::new(Completion),
                }),
                numeric: Default::default(),
                strings: None,
                bit_arrays: None,
                root_tail_entry: false,
                domain: runtime.captures().domain(),
                prepaid_completion: true,
            });
            let error = if ready {
                Execution::generated_ready(frame, state, plan, &mut storage)
                    .err()
                    .unwrap()
            } else {
                super::finish_native(frame, state, plan, &mut storage, false)
                    .err()
                    .unwrap()
            };
            assert_eq!(error, crate::ExecutionError::Invariant(expected));
            assert_eq!(mapped.load(Ordering::SeqCst), 1);
        }
        assert!(echo.is_empty());
        // Neither a bridge nor a retained phase may eagerly invoke the provider.
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        assert_eq!(
            host.block_on(hosted.run_main(&host, &mut (), &mut echo))
                .unwrap()
                .try_into_value()
                .unwrap(),
            crate::Value::Int(42.into())
        );
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(echo.is_empty());
    }

    fn completed_int<Plan: crate::runtime::ExecutableRuntimePlan>(
        active: Activation<'_, Plan>,
    ) -> IntegerValue {
        match active {
            Activation::CallComplete(output) => IntLocalId::from_call_output(output).unwrap(),
            _ => panic!("fixture generated Int activation must complete"),
        }
    }

    fn generated_output<Plan: crate::runtime::ExecutableRuntimePlan>(
        progress: Progress<'_, Plan>,
    ) -> Option<IntegerValue> {
        match progress {
            Progress::CallComplete(output) => Some(IntLocalId::from_call_output(output).unwrap()),
            Progress::Continue(_) => None,
            _ => panic!("fixture generated Int progress must return its owned result"),
        }
    }

    #[test]
    fn declined_native_requests_keep_the_graph_or_resumable_caller() {
        use crate::plan::execution::function::StringFunctionId;
        use crate::runtime::compiled::calls::{
            CallValues, GeneratedNativePhase, GeneratedNativeState, StringNativeExecution,
            StringNativeRequest,
        };
        use crate::{
            HostCall, HostCallContinuation, HostCallError, HostConstructions, HostOwnedCompletion,
            HostProvider, HostTypeListEnd, StringValue,
        };
        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        fn append<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, StringValue>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            value: StringValue,
        ) -> Result<HostCallContinuation<'call, StringValue>, HostCallError> {
            let _ = call.state();
            Ok(call.resume(constructions, move |_| {
                Box::pin(async move {
                    Ok(HostOwnedCompletion::new(move |call, _| {
                        Ok(call.return_value(format!("{}!", value.as_str().unwrap()).into()))
                    }))
                })
            }))
        }
        struct Completion;
        impl CallExecution for Completion {
            fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
                false
            }
            fn retained_bytes(&self) -> usize {
                0
            }
            fn advance(self: Box<Self>, _: &mut CallOps<'_>, _: &mut usize) -> CallProgress {
                CallProgress::Complete {
                    output: CallOutput::Int(42_i128.into()),
                    execution: self,
                }
            }
        }
        impl StringNativeExecution for Completion {
            fn resume_native(self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> {
                assert_eq!(value.as_str(), Ok("input!"));
                self
            }
        }
        for native in [false, true] {
            let source = if native {
                "@external(erlang, \"example\", \"append\") fn append(value: String) -> String pub fn main() { let _ = append(\"input\") 42 }"
            } else {
                "fn append(value: String) { value <> \"!\" } pub fn main() { let _ = append(\"input\") 42 }"
            };
            let providers = if native {
                HostProviderSet::from_providers([HostProviderModule::<StatelessHostProfile>::new("example", "example").unwrap().with_resumable_function::<Provider, (StringValue,), StringValue, HostTypeListEnd, _>("append", append).unwrap()]).unwrap()
            } else {
                HostProviderSet::<StatelessHostProfile>::new([]).unwrap()
            };
            let typed = crate::compile_typed_host_program(
                "example",
                "example",
                [PackageSource::new(
                    "example",
                    Vec::<String>::new(),
                    [ModuleSource::new("example", "src/example.gleam", source)],
                )],
                providers,
            )
            .unwrap();
            let mut hosted = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let (plan, _, captures) = hosted.parts_mut();
            let graph = int_body(&**plan, IntFunctionId(0)).block_graph().as_view();
            let mut engine = Completion;
            let empty = BlockEnvironment::from_retained(RetainedValues::empty());
            assert!(!engine.restart(
                CallTarget::Int(IntFunctionId(0)),
                0,
                CallInputs::new(&empty)
            ));
            assert_eq!(engine.retained_bytes(), 0);
            let state = Box::new(GeneratedNativeState {
                phase: GeneratedNativePhase::Progress(CallProgress::StringNative(
                    StringNativeRequest {
                        function: StringFunctionId(0),
                        site: HostCallSite::from_static(
                            "example",
                            "main",
                            SourceSpan::new(0, source.len()),
                        ),
                        root_tail: false,
                        arguments: Box::new(CallValues {
                            strings: vec!["input".into()],
                            ..Default::default()
                        }),
                        execution: Box::new(engine),
                    },
                )),
                numeric: Default::default(),
                strings: None,
                bit_arrays: None,
                root_tail_entry: true,
                domain: captures.domain(),
                prepaid_completion: false,
            });
            let frame = CallFrame {
                graph,
                position: GraphPosition::new(graph.entry(), RetainedValues::empty()),
                exit: Box::new(RootExit),
            };
            let mut storage = Storage::new();
            let mut echo = Vec::new();
            let mut runtime = RuntimeState::new(&mut echo);
            let mut progress = Execution {
                active: Activation::GeneratedNative { frame, state },
            }
            .advance(&**plan, &mut runtime, &mut storage, &mut 0)
            .unwrap();
            assert_eq!(matches!(&progress, Progress::Host(_)), native);
            if native {
                drop(progress);
            } else {
                while let Progress::Continue(execution) = progress {
                    progress = execution
                        .advance(&**plan, &mut runtime, &mut storage, &mut 8)
                        .unwrap();
                }
                assert_eq!(
                    generated_output(progress).unwrap(),
                    IntegerValue::from(42_i64)
                );
            }
            assert!(echo.is_empty());
            drop(storage);
            let mut source_echo = Vec::new();
            assert_eq!(
                crate::execution_fixture::run(&mut hosted, &mut (), &mut source_echo).unwrap(),
                crate::Value::Int(42.into())
            );
            assert!(source_echo.is_empty());
        }
    }

    #[test]
    fn generated_result_guards_reject_canonical_graph_owners() {
        use std::panic::{AssertUnwindSafe, catch_unwind};
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    "pub fn main() { 42 }",
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let plan = &**plan;
        let graph = int_body(plan, IntFunctionId(0)).block_graph().as_view();
        let failure = catch_unwind(AssertUnwindSafe(|| {
            completed_int::<crate::plan::execution::HostedProgram<StatelessHostProfile>>(
                Activation::Graph(super::super::Frame {
                    graph,
                    position: GraphPosition::new(BlockId(0), RetainedValues::empty()),
                    exit: Box::new(RootExit),
                }),
            )
        }))
        .err()
        .unwrap();
        assert_eq!(
            failure.downcast_ref::<&str>(),
            Some(&"fixture generated Int activation must complete")
        );
        let mut echo = Vec::new();
        let mut runtime = RuntimeState::new(&mut echo);
        let mut storage = Storage::new();
        let mut progress = Execution::new(graph, RetainedValues::empty(), None)
            .advance(plan, &mut runtime, &mut storage, &mut 1024)
            .unwrap();
        while let Progress::Continue(next) = progress {
            progress = next
                .advance(plan, &mut runtime, &mut storage, &mut 1024)
                .unwrap();
        }
        let failure = catch_unwind(AssertUnwindSafe(|| generated_output(progress)))
            .err()
            .unwrap();
        assert_eq!(
            failure.downcast_ref::<&str>(),
            Some(&"fixture generated Int progress must return its owned result")
        );
        let pending: Execution<'_, crate::plan::execution::HostedProgram<StatelessHostProfile>> =
            Execution::new(graph, RetainedValues::empty(), None);
        assert_eq!(generated_output(Progress::Continue(pending)), None);
        assert!(echo.is_empty());
    }

    fn generated_execution<'plan, Plan: crate::runtime::ExecutableRuntimePlan>(
        active: Activation<'plan, Plan>,
    ) -> (CallFrame<'plan, Plan>, Box<dyn CallExecution>) {
        match active {
            Activation::FunctionCalls { frame, execution } => (frame, execution),
            _ => panic!("fixture must resume its generated execution"),
        }
    }

    #[test]
    #[should_panic(expected = "fixture must resume its generated execution")]
    fn generated_execution_guard_rejects_a_completed_activation() {
        generated_execution(Activation::<
            crate::plan::execution::HostedProgram<StatelessHostProfile>,
        >::CallComplete(CallOutput::Int(42_i128.into())));
    }

    fn native_owner<Plan: crate::runtime::ExecutableRuntimePlan>(
        active: &Activation<'_, Plan>,
    ) -> *const crate::runtime::compiled::calls::GeneratedNativeState {
        match active {
            Activation::GeneratedNative { state, .. } => std::ptr::from_ref(&**state),
            _ => panic!("fixture must retain its pending Native owner"),
        }
    }

    #[test]
    #[should_panic(expected = "fixture must retain its pending Native owner")]
    fn native_owner_guard_rejects_a_completed_activation() {
        native_owner(&Activation::<
            crate::plan::execution::HostedProgram<StatelessHostProfile>,
        >::CallComplete(CallOutput::Int(42_i128.into())));
    }

    #[test]
    fn canonical_call_boundaries_resume_each_typed_family_once_in_the_original_execution() {
        let source = r#"
fn integer() -> Int { 42 }
fn boolean() -> Bool { True }
fn integer_function() -> fn() -> Int { integer }
fn boolean_function() -> fn() -> Bool { boolean }
fn floating() -> Float { 4.5 }
fn text() -> String { "λtyped" }
fn bits() -> BitArray { <<5:size(3)>> }
fn codepoint() -> UtfCodepoint { let assert <<value:utf8_codepoint>> = <<"λ">> value }
fn nil() -> Nil { Nil }
fn floating_function() -> fn() -> Float { floating }
fn text_function() -> fn() -> String { text }
fn bits_function() -> fn() -> BitArray { bits }
fn codepoint_function() -> fn() -> UtfCodepoint { codepoint }
fn nil_function() -> fn() -> Nil { nil }

pub fn main() {
  let number = integer_function()
  let predicate = boolean_function()
  let floating_callable = floating_function()
  let _ = floating_callable()
  let text_callable = text_function()
  let _ = text_callable()
  let bits_callable = bits_function()
  let _ = bits_callable()
  let codepoint_callable = codepoint_function()
  let _ = codepoint_callable()
  let nil_callable = nil_function()
  let _ = nil_callable()
  case predicate() { True -> number() False -> 0 }
}
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
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
                    output: CallOutput::Int(42_i128.into()),
                    execution: self,
                }
            }
        }
        let mut completion = Finish;
        assert!(!completion.restart(
            CallTarget::Int(IntFunctionId(0)),
            0,
            CallInputs::new(&BlockEnvironment::from_retained(RetainedValues::empty())),
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
            let floating_count = mapped.clone();
            let floating = Box::new(move |value: f64| -> Box<dyn CallExecution> {
                assert_eq!(value, 4.5);
                floating_count.fetch_add(1, Ordering::SeqCst);
                Box::new(Finish)
            });
            let floating_function_count = mapped.clone();
            let floating_function =
                Box::new(move |value: FloatCallable| -> Box<dyn CallExecution> {
                    assert_eq!(value.target(), FloatFunctionId(0));
                    floating_function_count.fetch_add(1, Ordering::SeqCst);
                    Box::new(Finish)
                });
            let text_count = mapped.clone();
            let text = Box::new(move |value: crate::StringValue| -> Box<dyn CallExecution> {
                assert_eq!(value.as_str(), Ok("λtyped"));
                text_count.fetch_add(1, Ordering::SeqCst);
                Box::new(Finish)
            });
            let text_function_count = mapped.clone();
            let text_function = Box::new(move |value: StringCallable| -> Box<dyn CallExecution> {
                assert_eq!(value.target(), StringFunctionId(0));
                text_function_count.fetch_add(1, Ordering::SeqCst);
                Box::new(Finish)
            });
            let bits_count = mapped.clone();
            let bits = Box::new(move |value: CallBitArray| -> Box<dyn CallExecution> {
                assert_eq!(value.0.as_value().bit_len(), 3);
                bits_count.fetch_add(1, Ordering::SeqCst);
                Box::new(Finish)
            });
            let bits_function_count = mapped.clone();
            let bits_function =
                Box::new(move |value: BitArrayCallable| -> Box<dyn CallExecution> {
                    assert_eq!(value.target(), BitArrayFunctionId(0));
                    bits_function_count.fetch_add(1, Ordering::SeqCst);
                    Box::new(Finish)
                });
            let codepoint_count = mapped.clone();
            let codepoint = Box::new(move |value: char| -> Box<dyn CallExecution> {
                assert_eq!(value, 'λ');
                codepoint_count.fetch_add(1, Ordering::SeqCst);
                Box::new(Finish)
            });
            let codepoint_function_count = mapped.clone();
            let codepoint_function = Box::new(
                move |value: UtfCodepointCallable| -> Box<dyn CallExecution> {
                    assert_eq!(value.target(), UtfCodepointFunctionId(0));
                    codepoint_function_count.fetch_add(1, Ordering::SeqCst);
                    Box::new(Finish)
                },
            );
            let nil_count = mapped.clone();
            let nil = Box::new(move |value: ()| -> Box<dyn CallExecution> {
                let () = value;
                nil_count.fetch_add(1, Ordering::SeqCst);
                Box::new(Finish)
            });
            let nil_function_count = mapped.clone();
            let nil_function = Box::new(move |value: NilCallable| -> Box<dyn CallExecution> {
                assert_eq!(value.target(), NilFunctionId(0));
                nil_function_count.fetch_add(1, Ordering::SeqCst);
                Box::new(Finish)
            });
            let progress = if interpreted {
                [
                    CallProgress::InterpretedInt {
                        function: IntFunctionId(1),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: integer,
                    },
                    CallProgress::InterpretedBool {
                        function: BoolFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: boolean,
                    },
                    CallProgress::InterpretedIntFunction {
                        function: IntFunctionFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: integer_function,
                    },
                    CallProgress::InterpretedBoolFunction {
                        function: BoolFunctionFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: boolean_function,
                    },
                    CallProgress::InterpretedFloat {
                        function: FloatFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: floating,
                    },
                    CallProgress::InterpretedFloatFunction {
                        function: FloatFunctionFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: floating_function,
                    },
                    CallProgress::InterpretedString {
                        function: StringFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: text,
                    },
                    CallProgress::InterpretedStringFunction {
                        function: StringFunctionFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: text_function,
                    },
                    CallProgress::InterpretedBitArray {
                        function: BitArrayFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: bits,
                    },
                    CallProgress::InterpretedBitArrayFunction {
                        function: BitArrayFunctionFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: bits_function,
                    },
                    CallProgress::InterpretedUtfCodepoint {
                        function: UtfCodepointFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: codepoint,
                    },
                    CallProgress::InterpretedUtfCodepointFunction {
                        function: UtfCodepointFunctionFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: codepoint_function,
                    },
                    CallProgress::InterpretedNil {
                        function: NilFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: nil,
                    },
                    CallProgress::InterpretedNilFunction {
                        function: NilFunctionFunctionId(0),
                        site: site.clone(),
                        point,
                        values: Box::default(),
                        resume: nil_function,
                    },
                ]
            } else {
                [
                    CallProgress::Int {
                        function: IntFunctionId(1),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: integer,
                    },
                    CallProgress::Bool {
                        function: BoolFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: boolean,
                    },
                    CallProgress::IntFunction {
                        function: IntFunctionFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: integer_function,
                    },
                    CallProgress::BoolFunction {
                        function: BoolFunctionFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: boolean_function,
                    },
                    CallProgress::Float {
                        function: FloatFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: floating,
                    },
                    CallProgress::FloatFunction {
                        function: FloatFunctionFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: floating_function,
                    },
                    CallProgress::String {
                        function: StringFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: text,
                    },
                    CallProgress::StringFunction {
                        function: StringFunctionFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: text_function,
                    },
                    CallProgress::BitArray {
                        function: BitArrayFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: bits,
                    },
                    CallProgress::BitArrayFunction {
                        function: BitArrayFunctionFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: bits_function,
                    },
                    CallProgress::UtfCodepoint {
                        function: UtfCodepointFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: codepoint,
                    },
                    CallProgress::UtfCodepointFunction {
                        function: UtfCodepointFunctionFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: codepoint_function,
                    },
                    CallProgress::Nil {
                        function: NilFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: nil,
                    },
                    CallProgress::NilFunction {
                        function: NilFunctionFunctionId(0),
                        site: site.clone(),
                        arguments: CallArguments {
                            values: Box::default(),
                            captures: None,
                        },
                        resume: nil_function,
                    },
                ]
            };
            for progress in progress {
                let before = mapped.load(Ordering::SeqCst);
                let mut boundary = Boundary(progress);
                assert!(!boundary.restart(
                    CallTarget::Int(IntFunctionId(0)),
                    0,
                    CallInputs::new(&BlockEnvironment::from_retained(RetainedValues::empty())),
                ));
                assert_eq!(boundary.retained_bytes(), 0);
                let execution = Execution {
                    active: Activation::FunctionCalls {
                        frame: CallFrame {
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
                let mut progress = execution
                    .advance(plan, &mut state, &mut storage, &mut 0)
                    .unwrap();
                while let Progress::Continue(next) = progress {
                    progress = next
                        .advance(plan, &mut state, &mut storage, &mut 0)
                        .unwrap();
                }
                assert_eq!(
                    generated_output(progress).unwrap(),
                    IntegerValue::from(42_i64)
                );
                assert_eq!(mapped.load(Ordering::SeqCst), before + 1);
                assert!(echo.is_empty());
            }
            assert_eq!(mapped.load(Ordering::SeqCst), 14);
        }
    }

    #[test]
    fn scalar_handoffs_preserve_typed_arguments_and_resume_once_after_source_effects() {
        let source = r#"
fn integer(value: Int) -> Int { echo value value }
fn boolean(value: Bool) -> Bool { echo value value }
pub fn main() { case boolean(True) { True -> integer(7) False -> 0 } }
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let plan = &**plan;
        let root = int_body(plan, IntFunctionId(0)).block_graph().as_view();
        let site = HostCallSite::from_static("example", "main", SourceSpan::new(0, source.len()));
        let mapped = Arc::new(AtomicUsize::new(0));

        struct Boundary(Option<CallProgress>);
        impl CallExecution for Boundary {
            fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
                false
            }
            fn retained_bytes(&self) -> usize {
                0
            }
            fn advance(mut self: Box<Self>, _: &mut CallOps<'_>, _: &mut usize) -> CallProgress {
                self.0.take().unwrap()
            }
        }
        struct Completed(Option<CallOutput>);
        impl CallExecution for Completed {
            fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
                false
            }
            fn retained_bytes(&self) -> usize {
                0
            }
            fn advance(mut self: Box<Self>, _: &mut CallOps<'_>, _: &mut usize) -> CallProgress {
                CallProgress::Complete {
                    output: self.0.take().unwrap(),
                    execution: self,
                }
            }
        }

        let big = BigInt::from(1) << 100_u32;
        let integer_mapped = mapped.clone();
        let boolean_mapped = mapped.clone();
        let cases = [
            (
                CallProgress::IntScalar {
                    function: IntFunctionId(1),
                    site: site.clone(),
                    input: CallNativeInput::Int(CallInteger(IntegerValue::from(big.clone()))),
                    resume: Box::new(move |value| {
                        integer_mapped.fetch_add(1, Ordering::SeqCst);
                        Box::new(Completed(Some(CallOutput::Int(value))))
                    }),
                },
                IntegerValue::from(big.clone()),
                crate::Value::Int(big),
            ),
            (
                CallProgress::BoolScalar {
                    function: BoolFunctionId(0),
                    site,
                    input: CallNativeInput::Bool(false),
                    resume: Box::new(move |value| {
                        boolean_mapped.fetch_add(1, Ordering::SeqCst);
                        Box::new(Completed(Some(CallOutput::Int(CallInteger(
                            IntegerValue::from(i64::from(value)),
                        )))))
                    }),
                },
                IntegerValue::from(0_i64),
                crate::Value::Bool(false),
            ),
        ];
        for (progress, expected, echoed) in cases {
            let before = mapped.load(Ordering::SeqCst);
            let mut boundary = Boundary(Some(progress));
            let empty = BlockEnvironment::from_retained(RetainedValues::empty());
            assert!(!boundary.restart(
                CallTarget::Int(IntFunctionId(0)),
                0,
                CallInputs::new(&empty)
            ));
            assert_eq!(boundary.retained_bytes(), 0);
            let execution = Execution {
                active: Activation::FunctionCalls {
                    frame: CallFrame {
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
            let mut progress = execution
                .advance(plan, &mut state, &mut storage, &mut 0)
                .unwrap();
            while let Progress::Continue(next) = progress {
                progress = next
                    .advance(plan, &mut state, &mut storage, &mut 0)
                    .unwrap();
            }
            assert_eq!(generated_output(progress).unwrap(), expected);
            assert!(
                storage
                    .function_calls
                    .reuse(
                        CallTarget::Int(IntFunctionId(0)),
                        0,
                        CallInputs::new(&empty)
                    )
                    .is_none()
            );
            assert_eq!(mapped.load(Ordering::SeqCst), before + 1);
            assert_eq!(echo.len(), 1);
            assert_eq!(echo[0].value(), &echoed);
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
                Box::default(),
                GeneratedDestination {
                    frame: CallFrame {
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
