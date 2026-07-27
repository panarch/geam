use super::super::{evaluate, run_tail};
use crate::plan::execution::function::NeverFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::EvaluatedNeverFunction;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;
use std::convert::Infallible;

pub(in crate::runtime) fn run_never<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: NeverFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<Infallible> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.never_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_never_value<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: EvaluatedNeverFunction,
    mut inputs: RetainedValues,
) -> ExecutionResult<Infallible> {
    inputs.append_captures(function.captures());
    run_never(plan, state, function.runtime_id(), inputs)
}
