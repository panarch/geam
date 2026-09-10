use super::super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::CustomFunctionId;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::EvaluatedCustomValue;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn run_custom(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: CustomFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedCustomValue> {
    run(plan, state, function, origin, inputs)
}
