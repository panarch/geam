use super::super::{evaluate, run_tail};
use crate::plan::execution::function::CustomFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::EvaluatedCustomValue;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_custom<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: CustomFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedCustomValue> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.custom_function(*function).body().function_body(),
                inputs,
            )
        },
        |plan, function, target| plan.custom_function(*function).body().function_id(target),
    )
}
