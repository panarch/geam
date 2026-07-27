use super::super::{evaluate, run_tail};
use crate::plan::execution::function::NilFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_nil<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: NilFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<()> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.nil_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}
