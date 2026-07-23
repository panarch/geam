use super::{MatchIntBindingId, MatchPatternBinding};
use crate::plan::execution::{Endianness, IntLocalId, StringEncoding};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Signedness {
    Signed,
    Unsigned,
}

pub(crate) struct BitArrayPattern {
    segments: Box<[BitArrayPatternSegment]>,
}

pub(crate) enum BitArrayPatternSegment {
    Int {
        pattern: BitArrayPatternValue<BigInt>,
        size: BitArrayPatternSize,
        endianness: Endianness,
        signedness: Signedness,
    },
    Float {
        pattern: BitArrayPatternValue<f64>,
        size: BitArrayPatternSize,
        endianness: Endianness,
    },
    Bits {
        pattern: BitArrayBindingPattern,
        size: Option<BitArrayPatternSize>,
        unit: u8,
    },
    String {
        pattern: BitArrayStringPattern,
        encoding: StringEncoding,
    },
    UtfCodepoint {
        pattern: BitArrayBindingPattern,
        encoding: StringEncoding,
    },
}

pub(crate) struct BitArrayPatternSize {
    value: BitArrayPatternSizeExpr,
    unit: u8,
}

pub(crate) enum BitArrayPatternSizeExpr {
    Value(BigInt),
    Local(IntLocalId),
    Binding(MatchIntBindingId),
    Add { left: Box<Self>, right: Box<Self> },
    Subtract { left: Box<Self>, right: Box<Self> },
    Multiply { left: Box<Self>, right: Box<Self> },
    Divide { left: Box<Self>, right: Box<Self> },
    Remainder { left: Box<Self>, right: Box<Self> },
}

pub(crate) enum BitArrayPatternValue<Value> {
    Literal(Value),
    Bind(MatchPatternBinding),
    Discard,
    Alias {
        pattern: Box<Self>,
        binding: MatchPatternBinding,
    },
}

pub(crate) enum BitArrayStringPattern {
    Literal(EcoString),
    Discard,
}

pub(crate) enum BitArrayBindingPattern {
    Bind(MatchPatternBinding),
    Discard,
    Alias {
        pattern: Box<Self>,
        binding: MatchPatternBinding,
    },
}

impl BitArrayPattern {
    pub(in crate::plan::execution) fn new(segments: Vec<BitArrayPatternSegment>) -> Self {
        Self {
            segments: segments.into_boxed_slice(),
        }
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
