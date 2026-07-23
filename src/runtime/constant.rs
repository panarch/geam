use crate::plan::execution::{ConstantProgram, ExecutionPlan};
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::{self, GraphValue, RetainedValues};
use crate::runtime::state::RuntimeState;

pub(super) fn evaluate<Value>(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    program: &ConstantProgram<Value>,
) -> ExecutionResult<Value::Evaluated>
where
    Value: GraphValue,
{
    graph::execute(plan, state, program.graph(), RetainedValues::empty()).map(|completed| {
        let return_ = program.return_(completed.exit());
        completed.into_value(state, return_)
    })
}
