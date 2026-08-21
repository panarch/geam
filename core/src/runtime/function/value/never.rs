use super::super::{evaluate_never_entry, run_tail};
use crate::plan::execution::function::NeverFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::EvaluatedNeverFunction;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;
use std::convert::Infallible;

pub(in crate::runtime) fn run_never<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: NeverFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<Infallible> {
    run_tail(
        plan,
        state,
        function,
        origin,
        inputs,
        |plan, state, function, origin, inputs| {
            evaluate_never_entry(plan, state, plan.never_function(*function), origin, inputs)
        },
        |_, _, target| {
            (
                *target.function(),
                HostCallOrigin::source(target.site().clone()),
            )
        },
    )
}

pub(in crate::runtime) fn run_never_value<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: EvaluatedNeverFunction,
    origin: HostCallOrigin,
    mut inputs: RetainedValues,
) -> ExecutionResult<Infallible> {
    inputs.append_captures(function.captures());
    run_never(plan, state, function.runtime_id(), origin, inputs)
}
