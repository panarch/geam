use crate::plan::ValueShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostReturnFamily {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
}

impl HostReturnFamily {
    pub(crate) fn shape(self) -> ValueShape {
        match self {
            Self::Int => ValueShape::Int,
            Self::Float => ValueShape::Float,
            Self::String => ValueShape::String,
            Self::BitArray => ValueShape::BitArray,
            Self::UtfCodepoint => ValueShape::UtfCodepoint,
            Self::Bool => ValueShape::Bool,
            Self::Nil => ValueShape::Nil,
        }
    }
}
