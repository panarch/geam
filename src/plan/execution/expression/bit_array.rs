use super::{
    BitArrayFunctionExpr, BitArrayListExpr, BoolExpr, CallArg, FloatExpr, IntExpr, PanicExpr,
    StringExpr, TupleExpr,
};
use crate::plan::execution::{BitArrayFunctionId, BitArrayLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Endianness {
    Big,
    Little,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StringEncoding {
    Utf8,
    Utf16(Endianness),
    Utf32(Endianness),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FloatBitSize {
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

pub(crate) enum BitArraySegment {
    Int {
        value: IntExpr,
        bit_size: usize,
        endianness: Endianness,
    },
    Float {
        value: FloatExpr,
        bit_size: FloatBitSize,
        endianness: Endianness,
    },
    String {
        value: StringExpr,
        encoding: StringEncoding,
    },
    Bits(BitArrayExpr),
}

pub struct BitArrayExpr {
    kind: BitArrayExprKind,
}

pub(crate) enum BitArrayExprKind {
    Value(Vec<BitArraySegment>),
    LocalGet {
        local: BitArrayLocalId,
    },
    Call {
        function: BitArrayFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<BitArrayFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    ListIndex {
        list: Box<BitArrayListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<BitArrayExpr>,
        false_: Box<BitArrayExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, BitArrayExpr)>,
        fallback: Box<BitArrayExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, BitArrayExpr)>,
        fallback: Box<BitArrayExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, BitArrayExpr)>,
        fallback: Box<BitArrayExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<BitArrayExpr>,
    },
}

impl BitArrayExpr {
    pub(in crate::plan::execution) fn from_kind(kind: BitArrayExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &BitArrayExprKind {
        &self.kind
    }
}
