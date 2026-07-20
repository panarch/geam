use super::{
    BitArrayFunctionExpr, BitArrayListExpr, BoolExpr, CustomFieldAccess, DirectCall, FloatExpr,
    FunctionCall, IntExpr, PanicExpr, StringExpr, TupleExpr, UtfCodepointExpr,
};
use crate::plan::PanicSite;
use crate::plan::execution::{BitArrayFunctionId, BitArrayLocalId, ConstantId, Step};
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

pub(crate) struct BitArrayEvaluatedSize {
    value: IntExpr,
    unit: u8,
}

pub(crate) enum BitArrayBitsSize {
    Fixed(usize),
    Evaluated(BitArrayEvaluatedSize),
}

impl BitArrayEvaluatedSize {
    pub(in crate::plan::execution) fn new(value: IntExpr, unit: u8) -> Self {
        Self { value, unit }
    }

    pub(crate) fn value(&self) -> &IntExpr {
        &self.value
    }

    pub(crate) fn unit(&self) -> u8 {
        self.unit
    }
}

pub(crate) enum BitArraySegment {
    Int {
        value: IntExpr,
        bit_size: usize,
        endianness: Endianness,
    },
    EvaluatedInt {
        value: IntExpr,
        size: BitArrayEvaluatedSize,
        endianness: Endianness,
        site: PanicSite,
    },
    Float {
        value: FloatExpr,
        bit_size: FloatBitSize,
        endianness: Endianness,
    },
    EvaluatedFloat {
        value: FloatExpr,
        size: BitArrayEvaluatedSize,
        endianness: Endianness,
        site: PanicSite,
    },
    String {
        value: StringExpr,
        encoding: StringEncoding,
    },
    UtfCodepoint {
        value: UtfCodepointExpr,
        encoding: StringEncoding,
    },
    Bits(BitArrayExpr),
    SizedBits {
        value: BitArrayExpr,
        size: BitArrayBitsSize,
        site: PanicSite,
    },
}

pub struct BitArrayExpr {
    kind: BitArrayExprKind,
}

pub(crate) enum BitArrayExprKind {
    Value(Vec<BitArraySegment>),
    Constant(ConstantId<BitArrayExpr>),
    LocalGet {
        local: BitArrayLocalId,
    },
    Call(DirectCall<BitArrayFunctionId>),
    FunctionCall(FunctionCall<BitArrayFunctionExpr>),
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
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

    pub(in crate::plan::execution) fn into_kind(self) -> BitArrayExprKind {
        self.kind
    }

    pub(crate) fn kind(&self) -> &BitArrayExprKind {
        &self.kind
    }
}
