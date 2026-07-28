use super::super::{evaluate_entry, run_tail};
use crate::plan::execution::function::TupleFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_tuple<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: TupleFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<Vec<EvaluatedValue>> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(plan, state, plan.tuple_function(*function), origin, inputs)
        },
        |_, _, target| {
            (
                *target.function(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}
