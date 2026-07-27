use crate::plan::{BoolLocalId, IntLocalId, ValueShape};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostParameter {
    Int(IntLocalId),
    Bool(BoolLocalId),
}

impl HostParameter {
    pub(crate) fn shape(&self) -> ValueShape {
        match self {
            Self::Int(_) => ValueShape::Int,
            Self::Bool(_) => ValueShape::Bool,
        }
    }
}
