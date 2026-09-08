mod callable;
mod list;
mod returning_function;
mod value;

pub(crate) use callable::LocalCallable;
pub(in crate::runtime) use callable::{InvocableFunctionValue, StoredCallable, invoke_callable};
pub(in crate::runtime) use list::{
    run_bit_array_list, run_bool_list, run_custom_list, run_external_list, run_float_list,
    run_function_list, run_int_list, run_list, run_list_list, run_nil_list, run_parameter_list,
    run_parameter_list_list, run_string_list, run_tuple_list, run_utf_codepoint_list,
};
pub(in crate::runtime) use returning_function::{
    run_core_function, run_external_function_function,
};
pub(in crate::runtime) use value::{
    run_bit_array, run_bool, run_custom, run_external, run_float, run_int, run_never,
    run_never_value, run_nil, run_string, run_tuple, run_utf_codepoint,
};

use crate::plan::execution::function::ExecutionFunctionEntry;
use crate::plan::execution::function::{
    ExecutionFunction, ExecutionFunctionBody, ExecutionFunctionRef, ExecutionNeverFunction,
    FunctionBodyOwner, FunctionExit, ProfiledFunctionBody,
};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::{self, GraphValue, ProfiledRetainedValues};
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::{ExecutableRuntimePlan, RuntimeGraph};

pub(super) enum EvaluatedFunctionExit<Return, TailCall, Values: crate::runtime::RuntimeValueProfile>
{
    Return(Return),
    TailCall {
        function: TailCall,
        args: ProfiledRetainedValues<Values>,
    },
}

type EvaluatedEntryReturn<Plan, Body> = <<Body as FunctionBodyOwner>::Return as GraphValue<
    <Plan as RuntimeExecutionPlan>::Values,
>>::Evaluated;
type EvaluatedEntry<Plan, Body> = EvaluatedFunctionExit<
    EvaluatedEntryReturn<Plan, Body>,
    <Body as FunctionBodyOwner>::TailCall,
    <Plan as RuntimeExecutionPlan>::Values,
>;

pub(super) fn evaluate_entry<Plan, Body>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: &ExecutionFunction<Plan::Profile, Body>,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedEntry<Plan, Body>, <Plan as RuntimeExecutionPlan>::Values>
where
    Plan: ExecutableRuntimePlan,
    Body: ExecutionFunctionBody<Graph = RuntimeGraph<Plan>>,
    Body::Return: GraphValue<Plan::Values>,
    Body::TailCall: Clone,
{
    match function.as_ref() {
        ExecutionFunctionRef::Graph(function) => {
            evaluate(plan, state, function.body().function_body(), inputs)
        }
        ExecutionFunctionRef::Host(target) => plan
            .call_host(state, origin, target, inputs)
            .map(EvaluatedFunctionExit::Return),
    }
}

pub(super) fn evaluate_never_entry<Plan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: &ExecutionNeverFunction<Plan::Profile>,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<
    EvaluatedFunctionExit<
        std::convert::Infallible,
        crate::plan::FunctionCallTarget<crate::plan::execution::function::NeverFunctionId>,
        Plan::Values,
    >,
    Plan::Values,
>
where
    Plan: ExecutableRuntimePlan,
{
    match function.as_ref() {
        ExecutionFunctionRef::Graph(function) => {
            evaluate(plan, state, function.body().function_body(), inputs)
        }
        ExecutionFunctionRef::Host(target) => plan
            .call_host_never(state, origin, target, inputs)
            .map(EvaluatedFunctionExit::Return),
    }
}

pub(super) fn evaluate<Plan, Return, TailCall>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: &ProfiledFunctionBody<Return, TailCall, RuntimeGraph<Plan>>,
    inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<EvaluatedFunctionExit<Return::Evaluated, TailCall, Plan::Values>, Plan::Values>
where
    Plan: ExecutableRuntimePlan,
    Return: GraphValue<Plan::Values>,
    TailCall: Clone,
{
    graph::execute(plan, state, function.block_graph(), inputs).map(|completed| {
        let exit = function.exit(completed.exit());
        match exit {
            FunctionExit::Return(value) => {
                EvaluatedFunctionExit::Return(completed.into_value(state, value))
            }
            FunctionExit::TailCall { function, args } => {
                let function = function.clone();
                let args = completed.into_retained(state, args);
                EvaluatedFunctionExit::TailCall { function, args }
            }
        }
    })
}

fn run_tail<Plan, Id, Return, TailCall>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    mut function: Id,
    mut origin: HostCallOrigin,
    mut inputs: ProfiledRetainedValues<Plan::Values>,
    execute: impl Fn(
        &Plan,
        &mut RuntimeStateFor<'_, Plan>,
        &Id,
        HostCallOrigin,
        ProfiledRetainedValues<Plan::Values>,
    ) -> ExecutionResult<
        EvaluatedFunctionExit<Return, TailCall, Plan::Values>,
        Plan::Values,
    >,
    next: impl Fn(&Plan, &Id, TailCall) -> (Id, HostCallOrigin),
) -> ExecutionResult<Return, Plan::Values>
where
    Plan: ExecutableRuntimePlan,
{
    loop {
        match execute(plan, state, &function, origin, inputs)? {
            EvaluatedFunctionExit::Return(value) => return Ok(value),
            EvaluatedFunctionExit::TailCall {
                function: target,
                args,
            } => {
                (function, origin) = next(plan, &function, target);
                inputs = args;
            }
        }
    }
}
