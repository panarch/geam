use crate::execution::ExecutionUnit;
use crate::host::{HostCallError, HostRetainedCallback};
use crate::plan::HostCallSite;
use crate::plan::execution::compiled::NativeLoopTarget;
use crate::plan::execution::function::{BoolFunctionId, IntFunctionId};
use crate::plan::execution::runtime::RuntimeValueMetadata;
use crate::runtime::compiled::calls::{
    CallExecution, CallInteger, CallNativeInput, CallOps, CallProgress,
};
use crate::runtime::compiled::native_loop::NativeLoopOps;
use crate::runtime::compiled::numeric::NumericValues;
use crate::runtime::error::HostCallOrigin;
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::state::list::RuntimeListStorage;
use crate::runtime::{BorrowedValue, CaptureStorage, StoredRuntimeValue};

/// Generated execution and numeric storage stay together while the worker
/// retains the graph Frame and its original exit.
pub(in crate::runtime) struct NativeCallsState<Binding> {
    pub binding: Binding,
    pub progress: NativeCallsProgress,
}

pub(in crate::runtime) struct NativeCallsMachine {
    target: CallNativeTarget,
    numeric: NumericValues,
    execution: Box<dyn CallExecution>,
}

#[expect(
    clippy::large_enum_variant,
    reason = "The canonical handoff reuses its owned call progress without another allocation."
)]
pub(in crate::runtime) enum NativeCallsProgress {
    Running(NativeCallsMachine),
    Canonical {
        progress: CallProgress,
        numeric: NumericValues,
    },
}

pub(in crate::runtime) enum CallNativeTarget {
    Int(IntFunctionId),
    Bool(BoolFunctionId),
}

/// The actual retained binding selects one return family. Its borrow never
/// escapes an advance; generated state owns the input, caller and return.
pub enum CallNativeOps<'advance> {
    Int {
        function: IntFunctionId,
        native: CallNativeFunction<'advance, CallInteger>,
    },
    Bool {
        function: BoolFunctionId,
        native: CallNativeFunction<'advance, bool>,
    },
}

pub struct CallNativeFunction<'advance, Value> {
    native: &'advance HostRetainedCallback,
    unit: Option<&'advance ExecutionUnit>,
    metadata: RuntimeValueMetadata<'advance>,
    read: fn(&StoredRuntimeValue) -> Value,
    ran: &'advance mut bool,
}

/// The actual returned owner survives a yield before the charged Return.
/// The selected typed reader is consumed together with that owner.
pub struct CallNativeReturn<Value> {
    stored: StoredRuntimeValue,
    read: fn(&StoredRuntimeValue) -> Value,
}

#[derive(Debug)]
pub struct CallNativeFailure {
    pub(in crate::runtime) origin: HostCallOrigin,
    pub(in crate::runtime) error: Box<HostCallError>,
}

impl NativeCallsMachine {
    pub fn new(
        target: CallNativeTarget,
        numeric: NumericValues,
        execution: Box<dyn CallExecution>,
    ) -> Self {
        Self {
            target,
            numeric,
            execution,
        }
    }

    pub fn run(
        mut self,
        captures: &CaptureStorage,
        lists: &RuntimeListStorage,
        metadata: RuntimeValueMetadata<'_>,
        ops: NativeLoopOps<'_>,
        budget: &mut usize,
    ) -> Result<Option<NativeCallsProgress>, CallNativeFailure> {
        let mut ran = false;
        let progress = {
            let mut native = match self.target {
                CallNativeTarget::Int(function) => CallNativeOps::Int {
                    function,
                    native: CallNativeFunction {
                        native: ops.native,
                        unit: ops.unit,
                        metadata,
                        read: |value| CallInteger(BorrowedValue::from_stored(value).int().clone()),
                        ran: &mut ran,
                    },
                },
                CallNativeTarget::Bool(function) => CallNativeOps::Bool {
                    function,
                    native: CallNativeFunction {
                        native: ops.native,
                        unit: ops.unit,
                        metadata,
                        read: |value| BorrowedValue::from_stored(value).bool(),
                        ran: &mut ran,
                    },
                },
            };
            self.execution.advance_native(
                &mut CallOps::new(captures, &mut self.numeric, lists),
                budget,
                &mut native,
            )?
        };
        let Some(progress) = progress else {
            return Ok(None);
        };
        Ok(Some(match progress {
            CallProgress::Yield(execution) if ran => NativeCallsProgress::Running(Self {
                target: self.target,
                numeric: self.numeric,
                execution,
            }),
            progress => NativeCallsProgress::Canonical {
                progress,
                numeric: self.numeric,
            },
        }))
    }
}

impl CallNativeTarget {
    pub fn select(target: NativeLoopTarget) -> Option<Self> {
        match target {
            NativeLoopTarget::Int(function) => Some(Self::Int(function)),
            NativeLoopTarget::Bool(function) => Some(Self::Bool(function)),
            NativeLoopTarget::Float(_)
            | NativeLoopTarget::String(_)
            | NativeLoopTarget::BitArray(_)
            | NativeLoopTarget::UtfCodepoint(_)
            | NativeLoopTarget::Nil(_) => None,
        }
    }
}

impl<Value> CallNativeFunction<'_, Value> {
    pub fn call(
        &mut self,
        input: CallNativeInput,
        site: HostCallSite,
    ) -> Result<Option<CallNativeReturn<Value>>, CallNativeFailure> {
        if self.unit.is_some_and(|unit| !unit.is_active()) {
            return Ok(None);
        }
        let value = match input {
            CallNativeInput::Int(value) => EvaluatedValue::Int(value.0),
            CallNativeInput::Bool(value) => EvaluatedValue::Bool(value),
        };
        let input = StoredRuntimeValue::new(value, self.metadata);
        let stored = (self.native)(&input).map_err(|error| CallNativeFailure {
            origin: HostCallOrigin::source(site),
            error: Box::new(error),
        })?;
        *self.ran = true;
        Ok(Some(CallNativeReturn {
            stored,
            read: self.read,
        }))
    }
}

impl<Value> CallNativeReturn<Value> {
    pub fn into_value(self) -> Value {
        (self.read)(&self.stored)
    }
}

#[cfg(test)]
mod tests {
    use super::{CallNativeFunction, CallNativeTarget, NativeCallsMachine};
    use crate::HostFailure;
    use crate::execution::UnitOwner;
    use crate::plan::execution::compiled::{CallTarget, NativeLoopTarget};
    use crate::plan::execution::function::{
        BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionId, NilFunctionId,
        StringFunctionId, UtfCodepointFunctionId,
    };
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::{HostCallSite, SourceSpan};
    use crate::runtime::compiled::calls::{
        CallExecution, CallInputs, CallInteger, CallNativeInput, CallOps, CallProgress,
    };
    use crate::runtime::compiled::native_loop::NativeLoopOps;
    use crate::runtime::compiled::numeric::NumericValues;
    use crate::runtime::evaluated::EvaluatedValue;
    use crate::runtime::state::list::RuntimeListStorage;
    use crate::runtime::{BorrowedValue, CaptureStorage, StoredRuntimeValue};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    #[test]
    fn selected_return_family_keeps_each_original_scalar_and_actual_owned_return() {
        let plan = crate::runtime::plan_src("pub fn main() { 7 }");
        let site = HostCallSite::from_static("example", "main", SourceSpan::new(16, 17));
        for boolean_input in [false, true] {
            let inputs = Arc::new(Mutex::new(Vec::new()));
            let observed = Arc::clone(&inputs);
            let native = move |input: &StoredRuntimeValue| {
                let row = BorrowedValue::from_stored(input);
                observed.lock().unwrap().push(if boolean_input {
                    i128::from(row.bool())
                } else {
                    i128::from(row.int().small().unwrap())
                });
                Ok(StoredRuntimeValue::new(
                    EvaluatedValue::Int(73.into()),
                    input.metadata(),
                ))
            };
            let mut ran = false;
            let returned = {
                let mut selected = CallNativeFunction {
                    native: &native,
                    unit: None,
                    metadata: plan.value_metadata(),
                    read: |value| CallInteger(BorrowedValue::from_stored(value).int().clone()),
                    ran: &mut ran,
                };
                let input = if boolean_input {
                    CallNativeInput::Bool(true)
                } else {
                    CallNativeInput::Int(7.into())
                };
                selected.call(input, site.clone()).unwrap().unwrap()
            };
            assert!(ran);
            assert_eq!(*inputs.lock().unwrap(), [if boolean_input { 1 } else { 7 }]);
            assert_eq!(returned.into_value().small(), Some(73));
        }

        let aliases = Arc::new(Mutex::new(Vec::new()));
        let observed = Arc::clone(&aliases);
        let native = move |input: &StoredRuntimeValue| {
            observed.lock().unwrap().push(input.clone_retained());
            Ok(input.clone_retained())
        };
        let mut ran = false;
        let returned = {
            let mut selected = CallNativeFunction {
                native: &native,
                unit: None,
                metadata: plan.value_metadata(),
                read: |value| BorrowedValue::from_stored(value).bool(),
                ran: &mut ran,
            };
            selected
                .call(CallNativeInput::Bool(false), site)
                .unwrap()
                .unwrap()
        };
        drop(plan);
        assert!(ran);
        assert!(!returned.into_value());
        assert!(!BorrowedValue::from_stored(&aliases.lock().unwrap()[0]).bool());
    }

    #[test]
    fn retained_failure_keeps_the_exact_call_origin_and_cancel_blocks_new_effects() {
        let plan = crate::runtime::plan_src("pub fn main() { 7 }");
        let site = HostCallSite::from_static("example", "main", SourceSpan::new(16, 17));
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let native = move |input: &StoredRuntimeValue| {
            if observed.fetch_add(1, Ordering::SeqCst) == 0 {
                Ok(input.clone_retained())
            } else {
                Err(HostFailure::new("actual retained failure").into())
            }
        };
        let (finished, _) = futures_channel::mpsc::unbounded();
        let owner = UnitOwner::new(finished);
        let unit = owner.handle();
        let mut ran = false;
        {
            let mut selected = CallNativeFunction {
                native: &native,
                unit: Some(&unit),
                metadata: plan.value_metadata(),
                read: |value| CallInteger(BorrowedValue::from_stored(value).int().clone()),
                ran: &mut ran,
            };
            let completed = selected
                .call(CallNativeInput::Int(7.into()), site.clone())
                .unwrap()
                .unwrap();
            assert_eq!(completed.into_value().small(), Some(7));
            let failure = selected
                .call(CallNativeInput::Int(7.into()), site.clone())
                .err()
                .unwrap();
            assert_eq!(failure.error.to_string(), "actual retained failure");
            assert_eq!(
                format!("{failure:?}"),
                "CallNativeFailure { origin: Source(HostCallSite { module: \"example\", function: \"main\", span: SourceSpan { start: 16, end: 17 } }), error: HostCallError { kind: Failure(HostFailure { message: \"actual retained failure\" }) } }",
            );
            assert_eq!(failure.origin.into_source_site(&site), Ok(site.clone()));
            assert!(unit.cancel());
            assert!(
                selected
                    .call(CallNativeInput::Int(7.into()), site)
                    .unwrap()
                    .is_none()
            );
        }
        assert!(ran);
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    struct LegacyYield {
        dropped: Arc<AtomicUsize>,
        owner: usize,
        completed: StoredRuntimeValue,
    }

    impl CallExecution for LegacyYield {
        fn restart(&mut self, _: CallTarget, _: usize, _: CallInputs<'_>) -> bool {
            false
        }

        fn retained_bytes(&self) -> usize {
            std::mem::size_of::<Self>()
        }

        fn advance(self: Box<Self>, _: &mut CallOps<'_>, _: &mut usize) -> CallProgress {
            assert_eq!(std::ptr::from_ref(&*self).addr(), self.owner);
            assert_eq!(
                BorrowedValue::from_stored(&self.completed).int().small(),
                Some(7)
            );
            CallProgress::Yield(self)
        }
    }

    impl Drop for LegacyYield {
        fn drop(&mut self) {
            self.dropped.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn an_old_generated_yield_transfers_the_same_workspace_without_replaying_native() {
        let plan = crate::runtime::plan_src("pub fn main() { 7 }");
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let native = move |input: &StoredRuntimeValue| {
            observed.fetch_add(1, Ordering::SeqCst);
            Ok(input.clone_retained())
        };
        let captures = CaptureStorage::default();
        let lists = RuntimeListStorage::default();
        let dropped = Arc::new(AtomicUsize::new(0));
        for target in [
            NativeLoopTarget::Int(IntFunctionId(2)),
            NativeLoopTarget::Bool(BoolFunctionId(3)),
        ] {
            for allowance in [0, 1, 2, 7, 1024] {
                let completed = native(&StoredRuntimeValue::new(
                    EvaluatedValue::Int(7.into()),
                    plan.value_metadata(),
                ))
                .unwrap();
                let completed_calls = calls.load(Ordering::SeqCst);
                let mut execution = Box::new(LegacyYield {
                    dropped: dropped.clone(),
                    owner: 0,
                    completed,
                });
                execution.owner = std::ptr::from_ref(&*execution).addr();
                let completed_drops = dropped.load(Ordering::SeqCst);
                assert_eq!(
                    execution.retained_bytes(),
                    std::mem::size_of::<LegacyYield>()
                );
                assert!(!execution.restart(
                    CallTarget::Int(IntFunctionId(2)),
                    0,
                    CallInputs::new(&[], &[], &[], &[], &[]),
                ));
                let machine = NativeCallsMachine::new(
                    CallNativeTarget::select(target).unwrap(),
                    NumericValues::default(),
                    execution,
                );
                let mut budget = allowance;
                let result = machine
                    .run(
                        &captures,
                        &lists,
                        plan.value_metadata(),
                        NativeLoopOps {
                            native: &native,
                            unit: None,
                        },
                        &mut budget,
                    )
                    .unwrap()
                    .unwrap();
                assert_eq!(budget, allowance);
                assert_eq!(calls.load(Ordering::SeqCst), completed_calls);
                assert_eq!(dropped.load(Ordering::SeqCst), completed_drops);
                drop(result);
                assert_eq!(dropped.load(Ordering::SeqCst), completed_drops + 1);
            }
        }
        assert_eq!(calls.load(Ordering::SeqCst), 10);
        assert_eq!(dropped.load(Ordering::SeqCst), 10);
        for target in [
            NativeLoopTarget::Float(FloatFunctionId(0)),
            NativeLoopTarget::String(StringFunctionId(0)),
            NativeLoopTarget::BitArray(BitArrayFunctionId(0)),
            NativeLoopTarget::UtfCodepoint(UtfCodepointFunctionId(0)),
            NativeLoopTarget::Nil(NilFunctionId(0)),
        ] {
            assert!(CallNativeTarget::select(target).is_none());
        }
    }
}
