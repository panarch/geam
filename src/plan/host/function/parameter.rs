use crate::plan::{
    BitArrayLocalId, BoolLocalId, FloatLocalId, IntLocalId, NilLocalId, StringLocalId,
    UtfCodepointLocalId, ValueShape,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostParameter {
    Int(IntLocalId),
    Float(FloatLocalId),
    String(StringLocalId),
    BitArray(BitArrayLocalId),
    UtfCodepoint(UtfCodepointLocalId),
    Bool(BoolLocalId),
    Nil(NilLocalId),
}

impl HostParameter {
    pub(crate) fn shape(&self) -> ValueShape {
        match self {
            Self::Int(_) => ValueShape::Int,
            Self::Float(_) => ValueShape::Float,
            Self::String(_) => ValueShape::String,
            Self::BitArray(_) => ValueShape::BitArray,
            Self::UtfCodepoint(_) => ValueShape::UtfCodepoint,
            Self::Bool(_) => ValueShape::Bool,
            Self::Nil(_) => ValueShape::Nil,
        }
    }
}
