use super::super::super::{Endianness, FloatBitSize, StringEncoding};
use crate::plan::PanicSite;
use crate::plan::execution::{
    BitArrayFunctionId, BitArrayListLocalId, ConstantId, CustomLocal, FloatLocalId, IntLocalId,
    ParamLocal, StringLocalId, TupleLocalId, UtfCodepointLocalId,
};

pub(crate) struct BitArrayEvaluatedSize {
    value: IntLocalId,
    unit: u8,
}

pub(crate) enum BitArrayBitsSize {
    Fixed(usize),
    Evaluated(BitArrayEvaluatedSize),
}

pub(crate) enum BitArraySegment {
    Int {
        value: IntLocalId,
        bit_size: usize,
        endianness: Endianness,
    },
    EvaluatedInt {
        value: IntLocalId,
        size: BitArrayEvaluatedSize,
        endianness: Endianness,
        site: PanicSite,
    },
    Float {
        value: FloatLocalId,
        bit_size: FloatBitSize,
        endianness: Endianness,
    },
    EvaluatedFloat {
        value: FloatLocalId,
        size: BitArrayEvaluatedSize,
        endianness: Endianness,
        site: PanicSite,
    },
    String {
        value: StringLocalId,
        encoding: StringEncoding,
    },
    UtfCodepoint {
        value: UtfCodepointLocalId,
        encoding: StringEncoding,
    },
    Bits(crate::plan::execution::BitArrayLocalId),
    SizedBits {
        value: crate::plan::execution::BitArrayLocalId,
        size: BitArrayBitsSize,
        site: PanicSite,
    },
}

pub(crate) enum BitArrayInstruction {
    Value(Box<[BitArraySegment]>),
    Constant(ConstantId<crate::plan::execution::BitArrayLocalId>),
    Call {
        function: BitArrayFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: crate::plan::execution::BitArrayFunctionLocalId,
        args: Box<[ParamLocal]>,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: CustomLocal,
        index: usize,
    },
    ListIndex {
        list: BitArrayListLocalId,
        index: usize,
    },
}

impl BitArrayEvaluatedSize {
    pub(in crate::plan::execution) fn new(value: IntLocalId, unit: u8) -> Self {
        Self { value, unit }
    }

    pub(crate) fn value(&self) -> IntLocalId {
        self.value
    }

    pub(crate) fn unit(&self) -> u8 {
        self.unit
    }
}
