use super::super::{evaluate, run_tail};
use crate::plan::execution::function::StringFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;
use ecow::EcoString;

pub(in crate::runtime) fn run_string(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: StringFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EcoString> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.string_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}
