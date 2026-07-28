use super::super::{evaluate_entry, run_tail};
use crate::plan::execution::function::CustomFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::EvaluatedCustomValue;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_custom<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: CustomFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedCustomValue> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(plan, state, plan.custom_function(*function), origin, inputs)
        },
        |_, function, target| {
            (
                function.with_index(*target.function()),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}
