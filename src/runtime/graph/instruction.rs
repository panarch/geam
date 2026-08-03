mod external;
mod function;
mod list;
mod value;

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
    match instruction.kind() {
        ProfiledInstructionKind::Int(instruction) => {
            value::int(plan, state, environment, instruction, &expected)
                .map(|value| environment.push_int(value))
        }
        ProfiledInstructionKind::Float(instruction) => {
            value::float(plan, state, environment, instruction, &expected)
                .map(|value| environment.push_float(value))
        }
        ProfiledInstructionKind::String(instruction) => {
            value::string(plan, state, environment, instruction, &expected)
                .map(|value| environment.push_string(value))
        }
        ProfiledInstructionKind::BitArray(instruction) => {
            value::bit_array(plan, state, environment, instruction, &expected)
                .map(|value| environment.push_bit_array(value))
        }
        ProfiledInstructionKind::UtfCodepoint(instruction) => {
            value::utf_codepoint(plan, state, environment, instruction, &expected)
                .map(|value| environment.push_utf_codepoint(value))
        }
        ProfiledInstructionKind::Custom(instruction) => {
            value::custom(plan, state, environment, instruction, &expected)
                .map(|value| environment.push_custom(value))
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
            value::bool(plan, state, environment, instruction, &expected)
                .map(|value| environment.push_bool(value))
        }
        ProfiledInstructionKind::Nil(instruction) => {
            value::nil(plan, state, environment, instruction, &expected)
                .map(|()| environment.push_nil())
        }
        ProfiledInstructionKind::Tuple(instruction) => {
            value::tuple(plan, state, environment, instruction, &expected)
                .map(|value| environment.push_tuple(value))
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
