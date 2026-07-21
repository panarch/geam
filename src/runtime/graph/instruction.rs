mod function;
mod list;
mod value;

use crate::plan::execution::{ExecutionPlan, Instruction, InstructionKind};
use crate::runtime::environment::BlockEnvironment;
use crate::runtime::error::ExecutionResult;
use crate::runtime::state::RuntimeState;

pub(super) fn execute(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    environment: &mut BlockEnvironment,
    instruction: &Instruction,
) -> ExecutionResult<()> {
    let expected = plan.value_type(&plan.shape_value_type(instruction.output().shape()));
    match instruction.kind() {
        InstructionKind::Int(instruction) => {
            let value = value::int(plan, state, environment, instruction, &expected)?;
            environment.push_int(value);
        }
        InstructionKind::Float(instruction) => {
            let value = value::float(plan, state, environment, instruction, &expected)?;
            environment.push_float(value);
        }
        InstructionKind::String(instruction) => {
            let value = value::string(plan, state, environment, instruction, &expected)?;
            environment.push_string(value);
        }
        InstructionKind::BitArray(instruction) => {
            let value = value::bit_array(plan, state, environment, instruction, &expected)?;
            environment.push_bit_array(value);
        }
        InstructionKind::UtfCodepoint(instruction) => {
            let value = value::utf_codepoint(plan, state, environment, instruction, &expected)?;
            environment.push_utf_codepoint(value);
        }
        InstructionKind::Custom(instruction) => {
            let value = value::custom(plan, state, environment, instruction, &expected)?;
            environment.push_custom(value);
        }
        InstructionKind::Bool(instruction) => {
            let value = value::bool(plan, state, environment, instruction, &expected)?;
            environment.push_bool(value);
        }
        InstructionKind::Nil(instruction) => {
            value::nil(plan, state, environment, instruction, &expected)?;
            environment.push_nil();
        }
        InstructionKind::Tuple(instruction) => {
            let value = value::tuple(plan, state, environment, instruction, &expected)?;
            environment.push_tuple(value);
        }
        InstructionKind::List(instruction) => {
            list::execute(plan, state, environment, instruction, &expected)?;
        }
        InstructionKind::Function(instruction) => {
            let value = function::evaluate(plan, state, environment, instruction, &expected)?;
            function::push(environment, value);
        }
    }
    Ok(())
}
