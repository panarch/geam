use super::super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::ParameterFunctionId;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn run_parameter(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: ParameterFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<crate::ParameterValue> {
    run(plan, state, function, origin, inputs)
}
