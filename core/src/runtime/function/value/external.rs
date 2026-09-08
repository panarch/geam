use super::super::{evaluate_entry, run_tail};
use crate::plan::execution::function::ExternalFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::EvaluatedExternalValue;
use crate::runtime::graph::ProfiledRetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_external<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ExternalFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedExternalValue<Plan::Values>, Plan::Values> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_entry(
                plan,
                state,
                plan.external_function(*function),
                origin,
                inputs,
            )
        },
        |_, function, target| {
            (
                function.with_index(*target.function()),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}
