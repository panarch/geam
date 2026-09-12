use super::super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::NeverFunctionId;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;
use std::convert::Infallible;

pub(in crate::runtime) fn run_never(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: NeverFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<Infallible> {
    run(plan, state, function, origin, inputs)
}
