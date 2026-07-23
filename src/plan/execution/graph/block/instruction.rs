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
