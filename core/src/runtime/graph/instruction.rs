mod external;
mod function;
mod list;
mod value;

pub(in crate::runtime) use external::evaluate_action as evaluate_external;
pub(in crate::runtime) use function::{
    CoreFunctionInstructionValue, ExternalFunctionInstructionValue,
    evaluate_action as evaluate_function, evaluate_external_action as evaluate_external_function,
    validate_return_family,
};
pub(in crate::runtime) use list::{
    ExternalListInstructionValue, ListInstructionValue, evaluate as evaluate_list,
    evaluate_external as evaluate_external_list,
};
pub(in crate::runtime) use value::{
    InstructionValue, InstructionValueWithoutConstant, bit_array as evaluate_bit_array,
    bool as evaluate_bool, custom as evaluate_custom, float as evaluate_float, int as evaluate_int,
    nil as evaluate_nil, string as evaluate_string, tuple as evaluate_tuple,
    utf_codepoint as evaluate_utf_codepoint,
};

use super::environment::BlockEnvironment;
use crate::plan::execution::graph::{ProfiledInstruction, ProfiledInstructionKind};
use crate::runtime::error::ExecutionResult;
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::{ExecutableRuntimePlan, RuntimeGraph};

pub(super) fn execute<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    environment: &mut BlockEnvironment,
    instruction: &ProfiledInstruction<RuntimeGraph<Plan>>,
) -> ExecutionResult<()> {
    let expected = plan.value_type(&plan.shape_value_type(instruction.output().shape()));
    macro_rules! evaluate_value {
        ($evaluate:ident, $run:ident, $push:ident, $instruction:expr) => {{
            let value = match value::$evaluate(plan, state, environment, $instruction, &expected)? {
                value::InstructionValue::Ready(value) => value,
                value::InstructionValue::Constant(id) => value::constant(plan, state, id)?,
                value::InstructionValue::Call {
                    function,
                    origin,
                    inputs,
                } => crate::runtime::function::$run(plan, state, function, origin, inputs)?,
            };
            environment.$push(value);
            Ok(())
        }};
    }

    match instruction.kind() {
        ProfiledInstructionKind::Int(instruction) => {
            evaluate_value!(int, run_int, push_int, instruction)
        }
        ProfiledInstructionKind::Float(instruction) => {
            evaluate_value!(float, run_float, push_float, instruction)
        }
        ProfiledInstructionKind::String(instruction) => {
            evaluate_value!(string, run_string, push_string, instruction)
        }
        ProfiledInstructionKind::BitArray(instruction) => {
            evaluate_value!(bit_array, run_bit_array, push_bit_array, instruction)
        }
        ProfiledInstructionKind::UtfCodepoint(instruction) => {
            value::utf_codepoint(plan, state, environment, instruction, &expected)
                .and_then(|instruction| match instruction {
                    value::InstructionValueWithoutConstant::Ready(value) => Ok(value),
                    value::InstructionValueWithoutConstant::Call {
                        function,
                        origin,
                        inputs,
                    } => crate::runtime::function::run_utf_codepoint(
                        plan, state, function, origin, inputs,
                    ),
                })
                .map(|value| environment.push_utf_codepoint(value))
        }
        ProfiledInstructionKind::Custom(instruction) => {
            evaluate_value!(custom, run_custom, push_custom, instruction)
        }
        ProfiledInstructionKind::External(instruction) => {
            external::evaluate(plan, state, environment, instruction, &expected)
                .map(|value| environment.push_external(value))
        }
        ProfiledInstructionKind::ExternalList(instruction) => {
            plan.execute_external_list_instruction(state, environment, instruction, &expected)
        }
        ProfiledInstructionKind::ExternalFunction(instruction) => {
            plan.execute_external_function_instruction(state, environment, instruction)
        }
        ProfiledInstructionKind::Bool(instruction) => {
            evaluate_value!(bool, run_bool, push_bool, instruction)
        }
        ProfiledInstructionKind::Nil(instruction) => {
            value::nil(plan, state, environment, instruction, &expected)
                .and_then(|instruction| match instruction {
                    value::InstructionValue::Ready(()) => Ok(()),
                    value::InstructionValue::Constant(id) => value::constant(plan, state, id),
                    value::InstructionValue::Call {
                        function,
                        origin,
                        inputs,
                    } => crate::runtime::function::run_nil(plan, state, function, origin, inputs),
                })
                .map(|()| environment.push_nil())
        }
        ProfiledInstructionKind::Tuple(instruction) => {
            evaluate_value!(tuple, run_tuple, push_tuple, instruction)
        }
        ProfiledInstructionKind::List(instruction) => {
            list::execute(plan, state, environment, instruction, &expected)
        }
        ProfiledInstructionKind::Function(instruction) => {
            function::evaluate(plan, state, environment, instruction, &expected)
                .map(|value| function::push(environment, value))
        }
    }
}

pub(super) fn execute_external_list<Plan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    environment: &mut BlockEnvironment,
    instruction: &crate::plan::execution::graph::ExternalListInstruction,
    expected: &crate::plan::ValueType,
) -> ExecutionResult<()>
where
    Plan: ExecutableRuntimePlan,
{
    list::execute_external(plan, state, environment, instruction, expected)
}

pub(super) fn execute_external_function<Plan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    environment: &mut BlockEnvironment,
    instruction: &crate::plan::execution::graph::ExternalFunctionInstruction,
) -> ExecutionResult<()>
where
    Plan: ExecutableRuntimePlan
        + crate::plan::execution::runtime::RuntimeExecutionPlan<
            Profile = crate::plan::execution::host::HostedExecutionProfile,
        >,
{
    function::evaluate_external(plan, state, environment, instruction)
        .map(|value| function::push(environment, value))
}
