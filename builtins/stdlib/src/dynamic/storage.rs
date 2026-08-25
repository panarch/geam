use crate::dict::DictDeclaration;
use geam_core::provider::advanced::{DynamicKind, StoredDynamic};

pub(super) enum DynamicValue {
    Stored {
        representation: DynamicRepresentation,
        value: StoredDynamic<super::function::provider::DynamicPayload>,
    },
    Array {
        value: StoredDynamic<super::function::provider::DynamicPayload>,
        elements: Box<[StoredDynamic<super::function::provider::DynamicPayload>]>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynamicRepresentation {
    Bool,
    String,
    Float,
    Int,
    BitArray,
    UtfCodepoint,
    List,
    Array,
    Dict,
    Nil,
    Function,
    Custom,
    External,
}

impl DynamicRepresentation {
    pub(super) fn from_value(
        value: &StoredDynamic<super::function::provider::DynamicPayload>,
    ) -> Self {
        if value.is_external::<DictDeclaration<(), ()>>() {
            return Self::Dict;
        }

        match value.kind() {
            DynamicKind::Int => Self::Int,
            DynamicKind::Float => Self::Float,
            DynamicKind::String => Self::String,
            DynamicKind::BitArray => Self::BitArray,
            DynamicKind::UtfCodepoint => Self::UtfCodepoint,
            DynamicKind::Bool => Self::Bool,
            DynamicKind::Nil => Self::Nil,
            DynamicKind::List => Self::List,
            DynamicKind::Tuple => Self::Array,
            DynamicKind::Custom => Self::Custom,
            DynamicKind::External => Self::External,
            DynamicKind::Function => Self::Function,
        }
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Bool => "Bool",
            Self::String => "String",
            Self::Float => "Float",
            Self::Int => "Int",
            Self::BitArray => "BitArray",
            Self::UtfCodepoint => "UtfCodepoint",
            Self::List => "List",
            Self::Array => "Array",
            Self::Dict => "Dict",
            Self::Nil => "Nil",
            Self::Function => "Function",
            Self::Custom => "Custom",
            Self::External => "External",
        }
    }
}

impl DynamicValue {
    pub(super) fn stored(value: StoredDynamic<super::function::provider::DynamicPayload>) -> Self {
        Self::Stored {
            representation: DynamicRepresentation::from_value(&value),
            value,
        }
    }

    pub(super) fn representation(&self) -> DynamicRepresentation {
        match self {
            Self::Stored { representation, .. } => *representation,
            Self::Array { .. } => DynamicRepresentation::Array,
        }
    }

    pub(super) fn value(&self) -> &StoredDynamic<super::function::provider::DynamicPayload> {
        match self {
            Self::Stored { value, .. } | Self::Array { value, .. } => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DynamicRepresentation;

    #[test]
    fn representation_names_cover_every_source_classification() {
        let cases = [
            (DynamicRepresentation::Bool, "Bool"),
            (DynamicRepresentation::String, "String"),
            (DynamicRepresentation::Float, "Float"),
            (DynamicRepresentation::Int, "Int"),
            (DynamicRepresentation::BitArray, "BitArray"),
            (DynamicRepresentation::UtfCodepoint, "UtfCodepoint"),
            (DynamicRepresentation::List, "List"),
            (DynamicRepresentation::Array, "Array"),
            (DynamicRepresentation::Dict, "Dict"),
            (DynamicRepresentation::Nil, "Nil"),
            (DynamicRepresentation::Function, "Function"),
            (DynamicRepresentation::Custom, "Custom"),
            (DynamicRepresentation::External, "External"),
        ];

        for (representation, expected) in cases {
            assert_eq!(representation.name(), expected);
        }
    }
}
