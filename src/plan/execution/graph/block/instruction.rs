mod bit_array;
mod bool;
mod custom;
mod float;
mod function;
mod int;
mod list;
mod nil;
mod string;
mod tuple;
mod utf_codepoint;

pub(crate) use bit_array::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment,
};
pub(crate) use bool::BoolInstruction;
pub(crate) use custom::CustomInstruction;
pub(crate) use float::FloatInstruction;
pub(crate) use function::{
    FunctionCapture, FunctionInstruction, FunctionInstructionKind, FunctionTarget,
};
pub(crate) use int::IntInstruction;
pub(crate) use list::{ListInstruction, ParameterListInstruction, TypedListInstruction};
pub(crate) use nil::NilInstruction;
pub(crate) use string::StringInstruction;
pub(crate) use tuple::TupleInstruction;
pub(crate) use utf_codepoint::UtfCodepointInstruction;

use crate::plan::execution::ParamSlot;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::ExplainFunctionId;
use crate::plan::execution::graph::{ExplainLocal, ParamLocal, write_locals};

pub(crate) struct Instruction {
    output: ParamSlot,
    kind: InstructionKind,
}

pub(crate) enum InstructionKind {
    Int(IntInstruction),
    Float(FloatInstruction),
    String(StringInstruction),
    BitArray(BitArrayInstruction),
    UtfCodepoint(UtfCodepointInstruction),
    Custom(CustomInstruction),
    Bool(BoolInstruction),
    Nil(NilInstruction),
    Tuple(TupleInstruction),
    List(ListInstruction),
    Function(FunctionInstruction),
}

impl Instruction {
    pub(in crate::plan::execution) fn new(output: ParamSlot, kind: InstructionKind) -> Self {
        Self { output, kind }
    }

    pub(crate) fn output(&self) -> &ParamSlot {
        &self.output
    }

    pub(crate) fn kind(&self) -> &InstructionKind {
        &self.kind
    }
}

impl Explain for Instruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("    ");
        context.write(self.output());
        context.push_str(" = ");
        context.write(self.kind());
        context.push('\n');
    }
}

impl Explain for InstructionKind {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Int(instruction) => context.write(instruction),
            Self::Float(instruction) => context.write(instruction),
            Self::String(instruction) => context.write(instruction),
            Self::BitArray(instruction) => context.write(instruction),
            Self::UtfCodepoint(instruction) => context.write(instruction),
            Self::Custom(instruction) => context.write(instruction),
            Self::Bool(instruction) => context.write(instruction),
            Self::Nil(instruction) => context.write(instruction),
            Self::Tuple(instruction) => context.write(instruction),
            Self::List(instruction) => context.write(instruction),
            Self::Function(instruction) => context.write(instruction),
        }
    }
}

pub(super) fn write_binary<Value: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    left: &Value,
    right: &Value,
) {
    output.push_str(opcode);
    output.push(' ');
    left.write_local(output);
    output.push(' ');
    right.write_local(output);
}

pub(super) fn write_call<Function: ExplainFunctionId>(
    output: &mut String,
    opcode: &str,
    function: &Function,
    args: &[ParamLocal],
) {
    output.push_str(opcode);
    output.push(' ');
    function.label().write(output);
    write_args(output, args);
}

pub(super) fn write_function_call<Function: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    function: &Function,
    args: &[ParamLocal],
) {
    output.push_str(opcode);
    output.push(' ');
    function.write_local(output);
    write_args(output, args);
}

pub(super) fn write_args(output: &mut String, args: &[ParamLocal]) {
    output.push_str(" args=");
    write_locals(output, args);
}

pub(super) fn write_constant<Value>(
    output: &mut String,
    family: &str,
    id: crate::plan::execution::ConstantId<Value>,
) {
    output.push_str("constant.");
    output.push_str(family);
    output.push('#');
    output.push_str(&id.index().to_string());
}

pub(super) fn write_length<Value: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    value: &Value,
    length: usize,
) {
    output.push_str(opcode);
    output.push(' ');
    value.write_local(output);
    output.push_str(" length=");
    output.push_str(&length.to_string());
}

pub(super) fn write_literal(output: &mut String, opcode: &str, value: &str) {
    output.push_str(opcode);
    output.push(' ');
    output.push_str(value);
}

pub(super) fn write_projection<Source: ExplainLocal>(
    output: &mut String,
    opcode: &str,
    source: &Source,
    index: usize,
) {
    output.push_str(opcode);
    output.push(' ');
    source.write_local(output);
    output.push_str(" index=");
    output.push_str(&index.to_string());
}

pub(super) fn write_unary<Value: ExplainLocal>(output: &mut String, opcode: &str, value: &Value) {
    output.push_str(opcode);
    output.push(' ');
    value.write_local(output);
}
