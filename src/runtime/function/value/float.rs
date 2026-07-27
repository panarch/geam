use super::super::{evaluate, run_tail};
use crate::plan::execution::function::FloatFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn run_float(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: FloatFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<f64> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.float_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}
