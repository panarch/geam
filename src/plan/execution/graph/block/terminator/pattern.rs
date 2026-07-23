mod bit_array;
mod list;

pub(crate) use bit_array::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, Signedness,
};
pub(crate) use list::{MatchPatternList, MatchPatternListTail};

use crate::plan::execution::CustomConstructorId;
use ecow::EcoString;
use num_bigint::BigInt;

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
