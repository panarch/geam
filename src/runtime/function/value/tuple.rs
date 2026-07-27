use super::super::{evaluate, run_tail};
use crate::plan::execution::function::TupleFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_tuple<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: TupleFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<Vec<EvaluatedValue>> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.tuple_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}
