use super::super::{evaluate, run_tail};
use crate::plan::execution::function::UtfCodepointFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn run_utf_codepoint(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: UtfCodepointFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<char> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.utf_codepoint_function(*function).body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}
