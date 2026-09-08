use crate::plan::execution::constant::ProfiledConstantProgram;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::{self, GraphValue, ProfiledRetainedValues};
use crate::runtime::state::RuntimeStateFor;

pub(super) fn evaluate<Plan, Return>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    program: &ProfiledConstantProgram<Return, crate::runtime::RuntimeGraph<Plan>>,
) -> ExecutionResult<Return::Evaluated, Plan::Values>
where
    Plan: ExecutableRuntimePlan,
    Return: GraphValue<Plan::Values>,
{
    graph::execute(
        plan,
        state,
        program.block_graph(),
        ProfiledRetainedValues::empty(),
    )
    .map(|completed| {
        let return_ = program.return_(completed.exit());
        completed.into_value(state, return_)
    })
}
