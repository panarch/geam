mod list;
mod returning_function;
mod value;

pub(in crate::runtime) use list::{
    run_bit_array_list, run_bool_list, run_custom_list, run_float_list, run_function_list,
    run_int_list, run_list, run_list_list, run_nil_list, run_parameter_list,
    run_parameter_list_list, run_string_list, run_tuple_list, run_utf_codepoint_list,
};
pub(in crate::runtime) use returning_function::run_function;
pub(in crate::runtime) use value::{RuntimeBoolFunction, RuntimeIntFunction};
pub(in crate::runtime) use value::{
    run_bit_array, run_bool, run_custom, run_float, run_int, run_never, run_never_value, run_nil,
    run_string, run_tuple, run_utf_codepoint,
};

use crate::plan::execution::function::{FunctionBody, FunctionExit};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::{self, GraphValue, RetainedValues};
use crate::runtime::state::RuntimeStateFor;

pub(super) enum EvaluatedFunctionExit<Return, TailCall> {
    Return(Return),
    TailCall {
        function: TailCall,
        args: RetainedValues,
    },
}

pub(super) fn evaluate<Plan, Return, TailCall>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: &FunctionBody<Return, TailCall>,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedFunctionExit<Return::Evaluated, TailCall>>
where
    Plan: ExecutableRuntimePlan,
    Return: GraphValue,
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
    mut inputs: RetainedValues,
    execute: impl Fn(
        &Plan,
        &mut RuntimeStateFor<'_, Plan>,
        &Id,
        RetainedValues,
    ) -> ExecutionResult<EvaluatedFunctionExit<Return, TailCall>>,
    next: impl Fn(&Plan, &Id, TailCall) -> Id,
) -> ExecutionResult<Return>
where
    Plan: ExecutableRuntimePlan,
{
    loop {
        match execute(plan, state, &function, inputs)? {
            EvaluatedFunctionExit::Return(value) => return Ok(value),
            EvaluatedFunctionExit::TailCall {
                function: target,
                args,
            } => {
                function = next(plan, &function, target);
                inputs = args;
            }
        }
    }
}
