use crate::execution::ExecutionUnit;
use crate::host::{HostCallError, HostRetainedCallback};
use crate::plan::execution::host::HostedFunctionMetadata;
use crate::runtime::StoredRuntimeValue;
use crate::runtime::error::HostCallOrigin;
use crate::runtime::integer::IntegerValue;
use std::ops::ControlFlow;
use std::sync::Arc;

/// Generated code owns only the canonical repetition cursor. The native
/// implementation and active execution unit are borrowed for this advance.
pub struct NativeLoopCursor {
    pub(in crate::runtime) counter: IntegerValue,
    pub(in crate::runtime) input: StoredRuntimeValue,
    pub(in crate::runtime) phase: NativeLoopPhase,
    pub(in crate::runtime) producer_literal: bool,
}

pub(in crate::runtime) enum NativeLoopPhase {
    ProducerCall,
    ProducerLiteral,
    ProducerReturn,
    NativeCall,
    NativeRun,
    NativeReturn(StoredRuntimeValue),
    SwitchEntry(StoredRuntimeValue),
    Switch(StoredRuntimeValue),
    Backedge(StoredRuntimeValue),
}

impl NativeLoopCursor {
    pub(in crate::runtime) fn into_return(self) -> ControlFlow<StoredRuntimeValue, Self> {
        match self.phase {
            NativeLoopPhase::Switch(returned) if self.counter.small() == Some(1) => {
                ControlFlow::Break(returned)
            }
            _ => ControlFlow::Continue(self),
        }
    }
}

pub struct NativeLoopOps<'advance> {
    pub(in crate::runtime) native: &'advance HostRetainedCallback,
    pub(in crate::runtime) unit: Option<&'advance ExecutionUnit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeLoopProgress {
    Yield,
    Complete,
    Cancelled,
}

pub type NativeLoopKernel = fn(
    &mut NativeLoopCursor,
    &NativeLoopOps<'_>,
    &mut usize,
) -> Result<NativeLoopProgress, HostCallError>;

pub(in crate::runtime) struct NativeLoopBinding {
    pub(in crate::runtime) native: Arc<HostRetainedCallback>,
    pub(in crate::runtime) metadata: Arc<HostedFunctionMetadata>,
    pub(in crate::runtime) origin: HostCallOrigin,
}

pub(in crate::runtime) struct NativeLoopState<Binding> {
    pub(in crate::runtime) binding: Binding,
    pub(in crate::runtime) cursor: NativeLoopCursor,
    pub(in crate::runtime) kernel: NativeLoopKernel,
}

/// Retains the exact prefix at each charged canonical boundary. The final
/// source exit is left to the worker, where its original GraphExit still lives.
pub fn run(
    cursor: &mut NativeLoopCursor,
    ops: &NativeLoopOps<'_>,
    budget: &mut usize,
) -> Result<NativeLoopProgress, HostCallError> {
    loop {
        if matches!(cursor.phase, NativeLoopPhase::Switch(_)) && cursor.counter.small() == Some(1) {
            return Ok(NativeLoopProgress::Complete);
        }
        if *budget == 0 {
            return Ok(NativeLoopProgress::Yield);
        }
        *budget -= 1;
        match std::mem::replace(&mut cursor.phase, NativeLoopPhase::ProducerCall) {
            // Callback call/return and native call have no new source effects.
            NativeLoopPhase::ProducerCall => {
                cursor.phase = if cursor.producer_literal {
                    NativeLoopPhase::ProducerLiteral
                } else {
                    NativeLoopPhase::ProducerReturn
                };
            }
            NativeLoopPhase::ProducerReturn => cursor.phase = NativeLoopPhase::NativeCall,
            NativeLoopPhase::NativeCall => cursor.phase = NativeLoopPhase::NativeRun,
            // A capture-free Nil producer still charges its original literal.
            NativeLoopPhase::ProducerLiteral => cursor.phase = NativeLoopPhase::ProducerReturn,
            NativeLoopPhase::NativeRun => {
                cursor.phase = NativeLoopPhase::NativeRun;
                if ops.unit.is_some_and(|unit| !unit.is_active()) {
                    return Ok(NativeLoopProgress::Cancelled);
                }
                cursor.phase = NativeLoopPhase::NativeReturn((ops.native)(&cursor.input)?);
            }
            NativeLoopPhase::NativeReturn(returned) => {
                cursor.phase = NativeLoopPhase::SwitchEntry(returned)
            }
            NativeLoopPhase::SwitchEntry(returned) => {
                cursor.phase = NativeLoopPhase::Switch(returned)
            }
            NativeLoopPhase::Switch(returned) => {
                cursor.counter = cursor.counter.subtract(&IntegerValue::from(1_i64));
                cursor.phase = NativeLoopPhase::Backedge(returned);
            }
            // The canonical backedge drops its discarded native result before
            // the next callback. It never chains that result into the input.
            NativeLoopPhase::Backedge(returned) => drop(returned),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NativeLoopCursor, NativeLoopOps, NativeLoopPhase, NativeLoopProgress, run};
    use crate::HostFailure;
    use crate::execution::UnitOwner;
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::runtime::evaluated::EvaluatedValue;
    use crate::runtime::{BorrowedValue, StoredRuntimeValue};
    use std::mem::discriminant;
    use std::sync::{Arc, Mutex};

    #[test]
    fn every_budget_preserves_the_native_prefix_original_input_and_last_result() {
        let plan = crate::runtime::plan_src("pub fn main() { 7 }");
        for count in [1, 2, 5, 129] {
            for allowance in [0, 1, 2, 3, 6, 7, 8, 31, 1024] {
                let calls = Arc::new(Mutex::new(Vec::new()));
                let observed = Arc::clone(&calls);
                let native = move |input: &StoredRuntimeValue| {
                    let original = BorrowedValue::from_stored(input)
                        .int()
                        .bigint()
                        .into_owned();
                    let mut calls = observed.lock().unwrap();
                    calls.push(original.clone());
                    Ok(StoredRuntimeValue::new(
                        EvaluatedValue::Int((original + calls.len()).into()),
                        input.metadata(),
                    ))
                };
                let ops = NativeLoopOps {
                    native: &native,
                    unit: None,
                };
                let mut cursor = NativeLoopCursor {
                    counter: count.into(),
                    input: StoredRuntimeValue::new(
                        EvaluatedValue::Int(7.into()),
                        plan.value_metadata(),
                    ),
                    phase: NativeLoopPhase::ProducerCall,
                    producer_literal: false,
                };
                let mut zero = 0;
                assert_eq!(
                    run(&mut cursor, &ops, &mut zero),
                    Ok(NativeLoopProgress::Yield)
                );
                assert_eq!(
                    discriminant(&cursor.phase),
                    discriminant(&NativeLoopPhase::ProducerCall)
                );
                assert_eq!(calls.lock().unwrap().as_slice(), []);
                if allowance == 0 {
                    continue;
                }
                // A graph callback Call/Return, native Call/Host/Return, switch,
                // subtract and backedge are eight steps. The last iteration
                // omits subtract/backedge; its final Exit stays on the worker.
                let total = 8 * count as usize - 2;
                let mut consumed = 0;
                loop {
                    let mut budget = allowance;
                    let result = run(&mut cursor, &ops, &mut budget).unwrap();
                    let charged = allowance.min(total - consumed);
                    assert_eq!(allowance - budget, charged);
                    consumed += charged;
                    let expected_calls = (consumed + 4) / 8;
                    assert_eq!(
                        calls.lock().unwrap().as_slice(),
                        vec![7.into(); expected_calls]
                    );
                    if consumed == total {
                        assert_eq!(result, NativeLoopProgress::Complete);
                        assert_eq!(cursor.counter.small(), Some(1));
                        assert_eq!(
                            BorrowedValue::from_stored(
                                &cursor.into_return().break_value().unwrap()
                            )
                            .int(),
                            &num_bigint::BigInt::from(7 + count)
                        );
                        break;
                    }
                    assert_eq!(result, NativeLoopProgress::Yield);
                }
            }
        }
    }

    #[test]
    fn a_literal_nil_producer_charges_its_original_instruction_before_each_native_call() {
        let plan = crate::runtime::plan_src("pub fn main() { Nil }");
        for count in [1, 2, 129] {
            for allowance in [1, 2, 4, 5, 8, 9, 128] {
                let calls = Arc::new(Mutex::new(0));
                let observed = Arc::clone(&calls);
                let native = move |input: &StoredRuntimeValue| {
                    assert_eq!(input.value(), &EvaluatedValue::Nil);
                    *observed.lock().unwrap() += 1;
                    Ok(input.clone_retained())
                };
                let ops = NativeLoopOps {
                    native: &native,
                    unit: None,
                };
                let mut cursor = NativeLoopCursor {
                    counter: count.into(),
                    input: StoredRuntimeValue::new(EvaluatedValue::Nil, plan.value_metadata()),
                    phase: NativeLoopPhase::ProducerCall,
                    producer_literal: true,
                };
                let total = 9 * count as usize - 2;
                let mut consumed = 0;
                loop {
                    let mut budget = allowance;
                    let progress = run(&mut cursor, &ops, &mut budget).unwrap();
                    let charged = allowance.min(total - consumed);
                    assert_eq!(allowance - budget, charged);
                    consumed += charged;
                    assert_eq!(*calls.lock().unwrap(), (consumed + 4) / 9);
                    if consumed == total {
                        assert_eq!(progress, NativeLoopProgress::Complete);
                        assert_eq!(
                            cursor.into_return().break_value().unwrap().value(),
                            &EvaluatedValue::Nil
                        );
                        break;
                    }
                    assert_eq!(progress, NativeLoopProgress::Yield);
                    cursor = cursor.into_return().continue_value().unwrap();
                }
            }
        }
    }

    #[test]
    fn big_and_nonterminating_counters_remain_bounded_and_cancellable() {
        let plan = crate::runtime::plan_src("pub fn main() { 7 }");
        for counter in [
            num_bigint::BigInt::from(0),
            (-3).into(),
            num_bigint::BigInt::from(i128::MIN),
            num_bigint::BigInt::from(i128::MAX) + 1,
        ] {
            let (events, _) = futures_channel::mpsc::unbounded();
            let owner = UnitOwner::new(events);
            let unit = owner.handle();
            let calls = Arc::new(Mutex::new(0));
            let observed = Arc::clone(&calls);
            let native = move |input: &StoredRuntimeValue| {
                *observed.lock().unwrap() += 1;
                Ok(input.clone_retained())
            };
            let ops = NativeLoopOps {
                native: &native,
                unit: Some(&unit),
            };
            let mut cursor = NativeLoopCursor {
                counter: counter.clone().into(),
                input: StoredRuntimeValue::new(
                    EvaluatedValue::Int(7.into()),
                    plan.value_metadata(),
                ),
                phase: NativeLoopPhase::ProducerCall,
                producer_literal: false,
            };
            assert_eq!(
                run(&mut cursor, &ops, &mut 8),
                Ok(NativeLoopProgress::Yield)
            );
            assert_eq!(*calls.lock().unwrap(), 1);
            assert_eq!(cursor.counter.bigint().as_ref(), &(counter - 1));
            assert_eq!(
                discriminant(&cursor.phase),
                discriminant(&NativeLoopPhase::ProducerCall)
            );
            assert!(unit.cancel());
            assert_eq!(
                run(&mut cursor, &ops, &mut 1024),
                Ok(NativeLoopProgress::Cancelled)
            );
            assert_eq!(*calls.lock().unwrap(), 1);
        }
    }

    #[test]
    fn native_cancellation_and_failure_never_replay_an_effect() {
        let plan = crate::runtime::plan_src("pub fn main() { 7 }");
        for stop_at in [1, 3, 5] {
            for cancel in [false, true] {
                let (events, _) = futures_channel::mpsc::unbounded();
                let owner = UnitOwner::new(events);
                let unit = owner.handle();
                let calls = Arc::new(Mutex::new(0));
                let observed = Arc::clone(&calls);
                let cancelling = unit.clone();
                let native = move |input: &StoredRuntimeValue| {
                    let mut calls = observed.lock().unwrap();
                    *calls += 1;
                    if *calls == stop_at {
                        if cancel {
                            cancelling.cancel();
                        } else {
                            return Err(HostFailure::new("native stopped").into());
                        }
                    }
                    Ok(input.clone_retained())
                };
                let mut cursor = NativeLoopCursor {
                    counter: 5.into(),
                    input: StoredRuntimeValue::new(
                        EvaluatedValue::Int(7.into()),
                        plan.value_metadata(),
                    ),
                    phase: NativeLoopPhase::ProducerCall,
                    producer_literal: false,
                };
                let ops = NativeLoopOps {
                    native: &native,
                    unit: Some(&unit),
                };
                let expected = if !cancel {
                    Err(HostFailure::new("native stopped").into())
                } else if stop_at == 5 {
                    Ok(NativeLoopProgress::Complete)
                } else {
                    Ok(NativeLoopProgress::Cancelled)
                };
                assert_eq!(run(&mut cursor, &ops, &mut 1024), expected);
                assert_eq!(*calls.lock().unwrap(), stop_at);
            }
        }
    }
}
