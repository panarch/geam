use super::super::{evaluate, run_tail};
use crate::plan::execution::function::StringFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;
use ecow::EcoString;

pub(in crate::runtime) fn run_string<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
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
