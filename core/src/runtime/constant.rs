use crate::plan::execution::constant::ProfiledConstantProgram;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::{self, GraphValue, RetainedValues};
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn evaluate_resumable<'call, Profile, Return>(
    plan: &'call crate::plan::execution::AsyncHostedExecution<Profile>,
    state: &'call mut crate::runtime::resumable::ResumableState<'_, Profile>,
    program: &'call ProfiledConstantProgram<
        Return,
        crate::plan::execution::function::HostedExecutionGraph,
    >,
) -> crate::runtime::resumable::ResumableFuture<
    'call,
    <Return as GraphValue<crate::runtime::TransferValues>>::Evaluated,
>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Return: GraphValue<crate::runtime::TransferValues> + Sync,
    <Return as GraphValue<crate::runtime::TransferValues>>::Evaluated: Send,
    Profile::ExternalStores: Send,
{
    Box::pin(async move {
        graph::execute_resumable(
            plan,
            state,
            program.block_graph(),
            crate::runtime::graph::ProfiledRetainedValues::empty(),
        )
        .await
        .map(|completed| {
            let return_ = program.return_(completed.exit());
            completed.into_value(state, return_)
        })
    })
}

pub(super) fn evaluate<Plan, Return>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    program: &ProfiledConstantProgram<Return, crate::runtime::RuntimeGraph<Plan>>,
) -> ExecutionResult<Return::Evaluated>
where
    Plan: ExecutableRuntimePlan,
    Return: GraphValue,
{
    graph::execute(plan, state, program.block_graph(), RetainedValues::empty()).map(|completed| {
        let return_ = program.return_(completed.exit());
        completed.into_value(state, return_)
    })
}
