mod bit_array;
mod function;
mod list;
mod operand;
mod value;

use self::bit_array::write_bit_array;
use self::function::write_function;
use self::list::write_list;
use self::value::{
    write_bool, write_custom, write_float, write_int, write_nil, write_string, write_tuple,
    write_utf_codepoint,
};
use super::super::ExecutionPlan;
use super::super::graph::{Instruction, InstructionKind};
use super::value::write_slot;

pub(super) fn write_instruction(
    output: &mut String,
    plan: &ExecutionPlan,
    instruction: &Instruction,
) {
    output.push_str("    ");
    write_slot(output, plan, instruction.output());
    output.push_str(" = ");
    match instruction.kind() {
        InstructionKind::Int(kind) => write_int(output, kind),
        InstructionKind::Float(kind) => write_float(output, kind),
        InstructionKind::String(kind) => write_string(output, kind),
        InstructionKind::BitArray(kind) => write_bit_array(output, kind),
        InstructionKind::UtfCodepoint(kind) => write_utf_codepoint(output, kind),
        InstructionKind::Custom(kind) => write_custom(output, kind),
        InstructionKind::Bool(kind) => write_bool(output, kind),
        InstructionKind::Nil(kind) => write_nil(output, kind),
        InstructionKind::Tuple(kind) => write_tuple(output, kind),
        InstructionKind::List(kind) => write_list(output, kind),
        InstructionKind::Function(kind) => write_function(output, kind),
    }
    output.push('\n');
}
