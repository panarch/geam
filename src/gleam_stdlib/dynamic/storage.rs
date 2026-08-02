use super::schema::DynamicSchema;
use crate::gleam_stdlib::GleamStdlibHostProfile;
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

pub(super) enum DynamicPayload {
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
pub(super) enum DynamicRepresentation {
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
    Unknown,
}

impl DynamicRepresentation {
    pub(super) fn from_type(type_: &crate::ValueType) -> Self {
        match type_ {
            crate::ValueType::Parameter(_) => Self::Unknown,
            crate::ValueType::Int => Self::Int,
            crate::ValueType::Float => Self::Float,
            crate::ValueType::String => Self::String,
            crate::ValueType::BitArray => Self::BitArray,
            crate::ValueType::UtfCodepoint => Self::UtfCodepoint,
            crate::ValueType::Bool => Self::Bool,
            crate::ValueType::Nil => Self::Nil,
            crate::ValueType::Tuple(_) => Self::Array,
            crate::ValueType::List(_) => Self::List,
            crate::ValueType::Function(_) => Self::Function,
            crate::ValueType::Custom(_) => Self::Custom,
            crate::ValueType::External(type_)
                if type_.type_name().package() == "gleam_stdlib"
                    && type_.type_name().module() == "gleam/dict"
                    && type_.type_name().name() == "Dict" =>
            {
                Self::Dict
            }
            crate::ValueType::External(_) => Self::External,
        }
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
            Self::Unknown => "Unknown",
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

    fn value(&self) -> &HostStoredDynamic {
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
    use crate::plan::TypeParameterId;
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
            (ValueType::Parameter(TypeParameterId(0)), "Unknown"),
            (ValueType::Int, "Int"),
            (ValueType::Float, "Float"),
            (ValueType::String, "String"),
            (ValueType::BitArray, "BitArray"),
            (ValueType::UtfCodepoint, "UtfCodepoint"),
            (ValueType::Bool, "Bool"),
            (ValueType::Nil, "Nil"),
            (ValueType::Tuple(vec![ValueType::Int]), "Array"),
            (ValueType::List(Box::new(ValueType::Int)), "List"),
            (
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Bool,
                ))),
                "Function",
            ),
            (ValueType::Custom(custom), "Custom"),
            (ValueType::External(dict), "Dict"),
            (ValueType::External(resource), "External"),
        ];

        for (type_, expected) in cases {
            assert_eq!(DynamicRepresentation::from_type(&type_).name(), expected);
        }
    }
}
