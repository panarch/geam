use super::{DraftInt, DraftValueRef};
use crate::plan::execution::{CustomConstructorId, Endianness, Signedness, StringEncoding};

pub(in crate::plan::execution::lowering) enum DraftMatchPattern {
    Bind(DraftMatchPatternBinding),
    Discard,
    Int(num_bigint::BigInt),
    Float(f64),
    String(ecow::EcoString),
    Bool(bool),
    Nil,
    Tuple(Vec<DraftMatchPattern>),
    List {
        elements: Vec<DraftMatchPattern>,
        tail: Option<DraftMatchListTail>,
    },
    BitArray(DraftBitArrayPattern),
    Custom {
        constructor: CustomConstructorId,
        fields: Vec<DraftMatchPattern>,
    },
    StringPrefix {
        prefix: ecow::EcoString,
        left: Option<DraftMatchPatternBinding>,
        right: Option<DraftMatchPatternBinding>,
    },
    Alias {
        pattern: Box<DraftMatchPattern>,
        binding: DraftMatchPatternBinding,
    },
}

#[derive(Clone)]
pub(in crate::plan::execution::lowering) struct DraftMatchPatternBinding {
    pub(super) value: DraftValueRef,
    pub(super) index: usize,
}

pub(in crate::plan::execution::lowering) enum DraftMatchListTail {
    Ignore,
    Bind(DraftMatchPatternBinding),
}

pub(in crate::plan::execution::lowering) struct DraftBitArrayPattern {
    pub(super) segments: Vec<DraftBitArrayPatternSegment>,
}

pub(in crate::plan::execution::lowering) enum DraftBitArrayPatternSegment {
    Int {
        pattern: DraftBitArrayPatternValue<num_bigint::BigInt>,
        size: DraftBitArrayPatternSize,
        endianness: Endianness,
        signedness: Signedness,
    },
    Float {
        pattern: DraftBitArrayPatternValue<f64>,
        size: DraftBitArrayPatternSize,
        endianness: Endianness,
    },
    Bits {
        pattern: DraftBitArrayBindingPattern,
        size: Option<DraftBitArrayPatternSize>,
        unit: u8,
    },
    String {
        pattern: DraftBitArrayStringPattern,
        encoding: StringEncoding,
    },
    UtfCodepoint {
        pattern: DraftBitArrayBindingPattern,
        encoding: StringEncoding,
    },
}

pub(in crate::plan::execution::lowering) enum DraftBitArrayStringPattern {
    Literal(ecow::EcoString),
    Discard,
}

pub(in crate::plan::execution::lowering) struct DraftBitArrayPatternSize {
    pub(super) value: DraftBitArrayPatternSizeExpr,
    pub(super) unit: u8,
}

pub(in crate::plan::execution::lowering) enum DraftBitArrayPatternSizeExpr {
    Value(num_bigint::BigInt),
    Local(DraftInt),
    Binding(usize),
    Add { left: Box<Self>, right: Box<Self> },
    Subtract { left: Box<Self>, right: Box<Self> },
    Multiply { left: Box<Self>, right: Box<Self> },
    Divide { left: Box<Self>, right: Box<Self> },
    Remainder { left: Box<Self>, right: Box<Self> },
}

pub(in crate::plan::execution::lowering) enum DraftBitArrayPatternValue<Value> {
    Literal(Value),
    Bind(DraftMatchPatternBinding),
    Discard,
    Alias {
        pattern: Box<Self>,
        binding: DraftMatchPatternBinding,
    },
}

pub(in crate::plan::execution::lowering) enum DraftBitArrayBindingPattern {
    Bind(DraftMatchPatternBinding),
    Discard,
    Alias {
        pattern: Box<Self>,
        binding: DraftMatchPatternBinding,
    },
}

impl DraftMatchPattern {
    pub(super) fn uses(&self, values: &mut Vec<DraftValueRef>) {
        match self {
            Self::Bind(_)
            | Self::Discard
            | Self::Int(_)
            | Self::Float(_)
            | Self::String(_)
            | Self::Bool(_)
            | Self::Nil => {}
            Self::Tuple(elements) => {
                for element in elements {
                    element.uses(values);
                }
            }
            Self::List { elements, .. } => {
                for element in elements {
                    element.uses(values);
                }
            }
            Self::BitArray(pattern) => pattern.uses(values),
            Self::Custom { fields, .. } => {
                for field in fields {
                    field.uses(values);
                }
            }
            Self::StringPrefix { .. } => {}
            Self::Alias { pattern, .. } => pattern.uses(values),
        }
    }
}

impl DraftBitArrayPattern {
    fn uses(&self, values: &mut Vec<DraftValueRef>) {
        for segment in &self.segments {
            match segment {
                DraftBitArrayPatternSegment::Int { size, .. }
                | DraftBitArrayPatternSegment::Float { size, .. } => size.value.uses(values),
                DraftBitArrayPatternSegment::Bits { size, .. } => {
                    if let Some(size) = size {
                        size.value.uses(values);
                    }
                }
                DraftBitArrayPatternSegment::String { .. }
                | DraftBitArrayPatternSegment::UtfCodepoint { .. } => {}
            }
        }
    }
}

impl DraftBitArrayPatternSizeExpr {
    fn uses(&self, values: &mut Vec<DraftValueRef>) {
        match self {
            Self::Value(_) | Self::Binding(_) => {}
            Self::Local(value) => values.push(value.erase()),
            Self::Add { left, right }
            | Self::Subtract { left, right }
            | Self::Multiply { left, right }
            | Self::Divide { left, right }
            | Self::Remainder { left, right } => {
                left.uses(values);
                right.uses(values);
            }
        }
    }
}
