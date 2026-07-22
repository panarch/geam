mod bit_array;
mod function;
mod list;
mod value;

pub(crate) use bit_array::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment,
};
pub(crate) use function::{
    FunctionCapture, FunctionInstruction, FunctionInstructionKind, FunctionTarget,
};
pub(crate) use list::{ListInstruction, ParameterListInstruction, TypedListInstruction};
pub(crate) use value::{
    BoolInstruction, CustomInstruction, FloatInstruction, IntInstruction, NilInstruction,
    StringInstruction, TupleInstruction, UtfCodepointInstruction,
};

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
