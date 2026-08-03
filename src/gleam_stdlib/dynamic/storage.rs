use super::schema::DynamicSchema;
use crate::gleam_stdlib::GleamStdlibHostProfile;
use crate::host::HostStoredValueFamily;
use crate::{
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalStorage,
    HostExternalStore, HostStoredDynamic,
};
use ecow::EcoString;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Default)]
pub(in crate::gleam_stdlib) struct Stores {
    values: HostExternalStore<DynamicPayload>,
}

pub(in crate::gleam_stdlib) enum DynamicPayload {
    Stored {
        representation: DynamicRepresentation,
        value: HostStoredDynamic,
    },
    Array {
        value: HostStoredDynamic,
        elements: Box<[HostStoredDynamic]>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::gleam_stdlib) enum DynamicRepresentation {
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
    pub(super) fn from_value(value: &HostStoredDynamic) -> Self {
        Self::from_family(value.value_family(), value.value_type())
    }

    pub(super) fn name(self) -> &'static str {
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

    fn from_family(family: HostStoredValueFamily, type_: &crate::ValueType) -> Self {
        if let crate::ValueType::External(type_) = type_
            && type_.type_name().package() == "gleam_stdlib"
            && type_.type_name().module() == "gleam/dict"
            && type_.type_name().name() == "Dict"
        {
            return Self::Dict;
        }

        match family {
            HostStoredValueFamily::Int => Self::Int,
            HostStoredValueFamily::Float => Self::Float,
            HostStoredValueFamily::String => Self::String,
            HostStoredValueFamily::BitArray => Self::BitArray,
            HostStoredValueFamily::UtfCodepoint => Self::UtfCodepoint,
            HostStoredValueFamily::Bool => Self::Bool,
            HostStoredValueFamily::Nil => Self::Nil,
            HostStoredValueFamily::List => Self::List,
            HostStoredValueFamily::Tuple => Self::Array,
            HostStoredValueFamily::Custom => Self::Custom,
            HostStoredValueFamily::External => Self::External,
            HostStoredValueFamily::Function => Self::Function,
        }
    }
}

impl DynamicPayload {
    pub(super) fn representation(&self) -> DynamicRepresentation {
        match self {
            Self::Stored { representation, .. } => *representation,
            Self::Array { .. } => DynamicRepresentation::Array,
        }
    }

    pub(super) fn value(&self) -> &HostStoredDynamic {
        match self {
            Self::Stored { value, .. } | Self::Array { value, .. } => value,
        }
    }
}

impl<Profile> HostExternalStorage<DynamicSchema> for Profile
where
    Profile: GleamStdlibHostProfile,
{
    type Payload = DynamicPayload;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &Profile::gleam_stdlib_stores(stores).dynamic.values
    }

    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.representation() == right.representation()
            && context.dynamic_values_equal(left.value(), right.value())
    }

    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.representation().hash(&mut hasher);
        context.dynamic_value_hash(value.value()).hash(&mut hasher);
        hasher.finish()
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        match value {
            DynamicPayload::Stored { value, .. } => context.inspect_dynamic_value(value),
            DynamicPayload::Array { elements, .. } => {
                let mut output = String::from("#(");
                let mut separator = "";
                for item in elements {
                    output.push_str(separator);
                    output.push_str(&context.inspect_dynamic_value(item));
                    separator = ", ";
                }
                output.push(')');
                output.into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DynamicRepresentation;
    use crate::host::HostStoredValueFamily;
    use crate::{
        CustomType, CustomTypeName, ExternalType, ExternalTypeName, FunctionType, ValueType,
    };

    #[test]
    fn classifies_every_runtime_value_family_and_nominal_dictionary() {
        let dict = ExternalType::new(
            ExternalTypeName::new("gleam_stdlib".into(), "gleam/dict".into(), "Dict".into()),
            vec![ValueType::Int, ValueType::String],
        );
        let resource = ExternalType::new(
            ExternalTypeName::new("domain".into(), "domain/resource".into(), "Resource".into()),
            Vec::new(),
        );
        let custom = CustomType::new(
            CustomTypeName::new("domain".into(), "domain/item".into(), "Item".into()),
            Vec::new(),
        );
        let cases = [
            (HostStoredValueFamily::Int, ValueType::Int, "Int"),
            (HostStoredValueFamily::Float, ValueType::Float, "Float"),
            (HostStoredValueFamily::String, ValueType::String, "String"),
            (
                HostStoredValueFamily::BitArray,
                ValueType::BitArray,
                "BitArray",
            ),
            (
                HostStoredValueFamily::UtfCodepoint,
                ValueType::UtfCodepoint,
                "UtfCodepoint",
            ),
            (HostStoredValueFamily::Bool, ValueType::Bool, "Bool"),
            (HostStoredValueFamily::Nil, ValueType::Nil, "Nil"),
            (
                HostStoredValueFamily::Tuple,
                ValueType::Tuple(vec![ValueType::Int]),
                "Array",
            ),
            (
                HostStoredValueFamily::List,
                ValueType::List(Box::new(ValueType::Int)),
                "List",
            ),
            (
                HostStoredValueFamily::Function,
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Bool,
                ))),
                "Function",
            ),
            (
                HostStoredValueFamily::Custom,
                ValueType::Custom(custom),
                "Custom",
            ),
            (
                HostStoredValueFamily::External,
                ValueType::External(dict),
                "Dict",
            ),
            (
                HostStoredValueFamily::External,
                ValueType::External(resource),
                "External",
            ),
        ];

        for (family, type_, expected) in cases {
            assert_eq!(
                DynamicRepresentation::from_family(family, &type_).name(),
                expected,
            );
        }
    }
}
