use super::schema::DynamicSchema;
use crate::{GleamStdlibHostProfile, stdlib_stores};
use crate::{
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalStorage,
    HostExternalStore, HostStoredDynamic,
};
use ecow::EcoString;
use geam_core::provider_support::{HostStoredValueFamily, stored_value_family, stored_value_type};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Default)]
pub(crate) struct Stores {
    values: HostExternalStore<DynamicPayload>,
}

pub enum DynamicPayload {
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
    pub(super) fn from_value(value: &HostStoredDynamic) -> Self {
        Self::from_family(stored_value_family(value), stored_value_type(value))
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

pub struct DynamicExternalStorage;

impl<Profile> HostExternalStorage<Profile, DynamicSchema> for DynamicExternalStorage
where
    Profile: GleamStdlibHostProfile,
{
    type Payload = DynamicPayload;

    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &stdlib_stores::<Profile>(stores).dynamic.values
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
    use crate::GleamStdlibProfile;
    use crate::ValueType;
    use geam_core::provider_support::HostStoredValueFamily;

    #[test]
    fn classifies_every_runtime_value_family_and_nominal_dictionary() {
        let dict_provider = crate::dict::host_provider::<GleamStdlibProfile>()
            .expect("official dict provider should register");
        let dict = dict_provider
            .functions()
            .find(|function| function.name() == "new")
            .expect("dict provider should register new")
            .type_()
            .return_()
            .clone();
        let string_tree_provider = crate::string_tree::host_provider::<GleamStdlibProfile>()
            .expect("official string tree provider should register");
        let resource = string_tree_provider
            .functions()
            .find(|function| function.name() == "from_string")
            .expect("string tree provider should register from_string")
            .type_()
            .return_()
            .clone();
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
            (HostStoredValueFamily::Tuple, ValueType::Nil, "Array"),
            (HostStoredValueFamily::List, ValueType::Nil, "List"),
            (HostStoredValueFamily::Function, ValueType::Nil, "Function"),
            (HostStoredValueFamily::Custom, ValueType::Nil, "Custom"),
            (HostStoredValueFamily::External, dict, "Dict"),
            (HostStoredValueFamily::External, resource, "External"),
        ];

        for (family, type_, expected) in cases {
            assert_eq!(
                DynamicRepresentation::from_family(family, &type_).name(),
                expected,
            );
        }
    }
}
