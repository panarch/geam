use super::super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::TupleFunctionId;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn run_tuple(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: TupleFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<Vec<EvaluatedValue>> {
    run(plan, state, function, origin, inputs)
}
