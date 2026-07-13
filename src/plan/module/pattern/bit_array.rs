use crate::plan::{BitArrayLocalId, Endianness, FloatLocalId, IntLocalId, StringEncoding};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BitArrayPattern {
    segments: Vec<BitArrayPatternSegment>,
}

#[derive(Debug, Clone, PartialEq)]
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
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BitArrayPatternSize {
    value: BitArrayPatternSizeExpr,
    unit: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BitArrayPatternSizeExpr {
    Value(BigInt),
    LocalGet { local: IntLocalId, name: EcoString },
    Add { left: Box<Self>, right: Box<Self> },
    Subtract { left: Box<Self>, right: Box<Self> },
    Multiply { left: Box<Self>, right: Box<Self> },
    Divide { left: Box<Self>, right: Box<Self> },
    Remainder { left: Box<Self>, right: Box<Self> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Signedness {
    Signed,
    Unsigned,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BitArrayPatternValue<Value, Local> {
    Literal(Value),
    Bind(PatternBinding<Local>),
    Discard,
    Alias {
        pattern: Box<Self>,
        binding: PatternBinding<Local>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BitArrayStringPattern {
    Literal(EcoString),
    Discard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BitArrayBindingPattern<Local> {
    Bind(PatternBinding<Local>),
    Discard,
    Alias {
        pattern: Box<Self>,
        binding: PatternBinding<Local>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PatternBinding<Local> {
    local: Local,
    name: EcoString,
}

impl BitArrayPattern {
    pub(crate) fn new(segments: Vec<BitArrayPatternSegment>) -> Self {
        Self { segments }
    }

    pub(crate) fn segments(&self) -> &[BitArrayPatternSegment] {
        &self.segments
    }

    pub(crate) fn into_segments(self) -> Vec<BitArrayPatternSegment> {
        self.segments
    }
}

impl BitArrayPatternSize {
    pub(crate) fn new(value: BitArrayPatternSizeExpr, unit: u8) -> Self {
        Self { value, unit }
    }

    pub(crate) fn value(&self) -> &BitArrayPatternSizeExpr {
        &self.value
    }

    pub(crate) fn into_parts(self) -> (BitArrayPatternSizeExpr, u8) {
        (self.value, self.unit)
    }
}

impl BitArrayPatternSizeExpr {
    pub(crate) fn value(value: BigInt) -> Self {
        Self::Value(value)
    }

    pub(crate) fn local_get(local: IntLocalId, name: EcoString) -> Self {
        Self::LocalGet { local, name }
    }

    pub(crate) fn add(left: Self, right: Self) -> Self {
        Self::Add {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub(crate) fn subtract(left: Self, right: Self) -> Self {
        Self::Subtract {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub(crate) fn multiply(left: Self, right: Self) -> Self {
        Self::Multiply {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub(crate) fn divide(left: Self, right: Self) -> Self {
        Self::Divide {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub(crate) fn remainder(left: Self, right: Self) -> Self {
        Self::Remainder {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl<Local> PatternBinding<Local> {
    pub(crate) fn new(local: Local, name: EcoString) -> Self {
        Self { local, name }
    }

    pub(crate) fn local(&self) -> &Local {
        &self.local
    }

    pub(crate) fn into_parts(self) -> (Local, EcoString) {
        (self.local, self.name)
    }
}
