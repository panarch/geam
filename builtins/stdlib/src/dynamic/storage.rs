use geam_core::provider::advanced::{NativeKind, NativeValue, StoredDynamic};

pub(super) struct DynamicValue {
    representation: DynamicRepresentation,
    view: NativeValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynamicRepresentation {
    Bool,
    String,
    Float,
    Int,
    BitArray,
    Atom,
    List,
    Array,
    Dict,
    Nil,
    Function,
    External,
}

impl DynamicRepresentation {
    pub(super) fn from_value(value: &NativeValue) -> Self {
        match value.kind() {
            NativeKind::Int => Self::Int,
            NativeKind::Float => Self::Float,
            NativeKind::Binary => {
                if value.bit_len().is_some_and(|len| len.is_multiple_of(8)) {
                    Self::String
                } else {
                    Self::BitArray
                }
            }
            NativeKind::Symbol => match value.as_symbol().as_deref() {
                Some("true" | "false") => Self::Bool,
                Some("nil" | "null" | "undefined") => Self::Nil,
                _ => Self::Atom,
            },
            NativeKind::List => Self::List,
            NativeKind::Tuple => Self::Array,
            NativeKind::Map => Self::Dict,
            NativeKind::External => Self::External,
            NativeKind::Function => Self::Function,
        }
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Bool => "Bool",
            Self::String => "String",
            Self::Float => "Float",
            Self::Int => "Int",
            Self::BitArray => "BitArray",
            Self::Atom => "Atom",
            Self::List => "List",
            Self::Array => "Array",
            Self::Dict => "Dict",
            Self::Nil => "Nil",
            Self::Function => "Function",
            Self::External => "External",
        }
    }
}

impl DynamicValue {
    pub(super) fn stored(value: StoredDynamic<super::function::provider::DynamicPayload>) -> Self {
        Self::native(value.native_view())
    }

    pub(super) fn native(view: NativeValue) -> Self {
        Self {
            representation: DynamicRepresentation::from_value(&view),
            view,
        }
    }

    pub(super) fn representation(&self) -> DynamicRepresentation {
        self.representation
    }

    pub(super) fn view(&self) -> &NativeValue {
        &self.view
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
            (DynamicRepresentation::Atom, "Atom"),
            (DynamicRepresentation::List, "List"),
            (DynamicRepresentation::Array, "Array"),
            (DynamicRepresentation::Dict, "Dict"),
            (DynamicRepresentation::Nil, "Nil"),
            (DynamicRepresentation::Function, "Function"),
            (DynamicRepresentation::External, "External"),
        ];

        for (representation, expected) in cases {
            assert_eq!(representation.name(), expected);
        }
    }
}
