use crate::plan::execution::{
    BitArrayLocalId, Endianness, FloatLocalId, IntLocalId, StringEncoding, UtfCodepointLocalId,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) struct BitArrayPattern {
    segments: Vec<BitArrayPatternSegment>,
}

pub(crate) enum BitArrayPatternSegment {
    Int {
        pattern: BitArrayPatternValue<BigInt, IntLocalId>,
        size: BitArrayPatternSize,
        endianness: Endianness,
        signedness: Signedness,
    },
    Float {
        pattern: BitArrayPatternValue<f64, FloatLocalId>,
        size: BitArrayPatternSize,
        endianness: Endianness,
    },
    Bits {
        pattern: BitArrayBindingPattern<BitArrayLocalId>,
        size: Option<BitArrayPatternSize>,
        unit: u8,
    },
    String {
        pattern: BitArrayStringPattern,
        encoding: StringEncoding,
    },
    UtfCodepoint {
        pattern: BitArrayBindingPattern<UtfCodepointLocalId>,
        encoding: StringEncoding,
    },
}

pub(crate) struct BitArrayPatternSize {
    value: BitArrayPatternSizeExpr,
    unit: u8,
}

pub(crate) enum BitArrayPatternSizeExpr {
    Value(BigInt),
    LocalGet(IntLocalId),
    Add { left: Box<Self>, right: Box<Self> },
    Subtract { left: Box<Self>, right: Box<Self> },
    Multiply { left: Box<Self>, right: Box<Self> },
    Divide { left: Box<Self>, right: Box<Self> },
    Remainder { left: Box<Self>, right: Box<Self> },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Signedness {
    Signed,
    Unsigned,
}

pub(crate) enum BitArrayPatternValue<Value, Local> {
    Literal(Value),
    Bind(PatternBinding<Local>),
    Discard,
    Alias {
        pattern: Box<Self>,
        binding: PatternBinding<Local>,
    },
}

pub(crate) enum BitArrayStringPattern {
    Literal(EcoString),
    Discard,
}

pub(crate) enum BitArrayBindingPattern<Local> {
    Bind(PatternBinding<Local>),
    Discard,
    Alias {
        pattern: Box<Self>,
        binding: PatternBinding<Local>,
    },
}

pub(crate) struct PatternBinding<Local> {
    local: Local,
}

impl BitArrayPattern {
    pub(in crate::plan::execution) fn new(segments: Vec<BitArrayPatternSegment>) -> Self {
        Self { segments }
    }

    pub(crate) fn segments(&self) -> &[BitArrayPatternSegment] {
        &self.segments
    }
}

impl BitArrayPatternSize {
    pub(in crate::plan::execution) fn new(value: BitArrayPatternSizeExpr, unit: u8) -> Self {
        Self { value, unit }
    }

    pub(crate) fn value(&self) -> &BitArrayPatternSizeExpr {
        &self.value
    }

    pub(crate) fn unit(&self) -> u8 {
        self.unit
    }
}

impl<Local> PatternBinding<Local> {
    pub(in crate::plan::execution) fn new(local: Local) -> Self {
        Self { local }
    }

    pub(crate) fn local(&self) -> &Local {
        &self.local
    }
}
