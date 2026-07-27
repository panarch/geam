use super::super::{evaluate, run_tail};
use crate::plan::execution::function::BitArrayFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::EvaluatedBitArray;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn run_bit_array(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: BitArrayFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedBitArray> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.bit_array_function(*function).body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}
