use crate::plan::execution::{CustomConstructorId, Endianness, StringEncoding};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Signedness {
    Signed,
    Unsigned,
}

pub(crate) enum MatchPattern {
    Bind(MatchPatternBinding),
    Discard,
    Int(BigInt),
    Float(f64),
    String(EcoString),
    Bool(bool),
    Nil,
    Tuple(Box<[MatchPattern]>),
    List(MatchPatternList),
    BitArray(BitArrayPattern),
    Custom {
        constructor: CustomConstructorId,
        fields: Box<[MatchPattern]>,
    },
    StringPrefix {
        prefix: EcoString,
        left: Option<MatchPatternBinding>,
        right: Option<MatchPatternBinding>,
    },
    Alias {
        pattern: Box<MatchPattern>,
        binding: MatchPatternBinding,
    },
}

pub(crate) struct MatchPatternBinding {
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct MatchIntBindingId(usize);

pub(crate) struct MatchPatternList {
    elements: Box<[MatchPattern]>,
    tail: Option<MatchPatternListTail>,
}

pub(crate) enum MatchPatternListTail {
    Ignore,
    Bind(MatchPatternBinding),
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
    Local(crate::plan::execution::IntLocalId),
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

impl MatchPatternBinding {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self { index }
    }

    pub(crate) fn int_id(&self) -> MatchIntBindingId {
        MatchIntBindingId(self.index)
    }

    pub(in crate::plan::execution) fn index(&self) -> usize {
        self.index
    }
}

impl MatchIntBindingId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(in crate::plan::execution) fn index(self) -> usize {
        self.0
    }
}

impl MatchPatternList {
    pub(in crate::plan::execution) fn new(
        elements: Vec<MatchPattern>,
        tail: Option<MatchPatternListTail>,
    ) -> Self {
        Self {
            elements: elements.into_boxed_slice(),
            tail,
        }
    }

    pub(crate) fn elements(&self) -> &[MatchPattern] {
        &self.elements
    }

    pub(crate) fn tail(&self) -> Option<&MatchPatternListTail> {
        self.tail.as_ref()
    }
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
