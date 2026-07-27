use crate::plan::ValueShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostReturnFamily {
    Int,
    Bool,
}

impl HostReturnFamily {
    pub(crate) fn shape(self) -> ValueShape {
        match self {
            Self::Int => ValueShape::Int,
            Self::Bool => ValueShape::Bool,
        }
    }
}
