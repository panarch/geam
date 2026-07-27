use crate::plan::execution::constant::ConstantProgram;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::{self, GraphValue, RetainedValues};
use crate::runtime::state::RuntimeState;

pub(super) fn evaluate<Return>(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    program: &ConstantProgram<Return>,
) -> ExecutionResult<Return::Evaluated>
where
    Return: GraphValue,
{
    graph::execute(plan, state, program.block_graph(), RetainedValues::empty()).map(|completed| {
        let return_ = program.return_(completed.exit());
        completed.into_value(state, return_)
    })
}
