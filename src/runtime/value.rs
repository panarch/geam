mod bit_array;
mod capture;
mod custom;
mod external;
mod function;
mod inspection;
mod list;

use ecow::EcoString;
use num_bigint::BigInt;

use crate::plan::ValueType;

pub use self::bit_array::{BitArrayValue, BitArrayValueLengthError};
pub(crate) use self::capture::{CaptureListValue, CaptureValue};
pub use self::custom::{CustomFieldValue, CustomValue};
pub use self::external::{ExternalValue, ExternalValueIdentity};
pub use self::function::FunctionValue;
pub(crate) use self::function::{
    BitArrayFunctionValue, BoolFunctionValue, CustomFunctionValue, CustomFunctionValueTarget,
    ExternalFunctionValue, FloatFunctionValue, FunctionFunctionValue, FunctionValueKind,
    GenericFunctionValue, IntFunctionValue, ListFunctionValue, NeverFunctionValue,
    NilFunctionValue, StringFunctionValue, TupleFunctionValue, UtfCodepointFunctionValue,
};
pub use self::inspection::ValueInspection;
pub use self::list::{ListValue, ListValueItemTypeMismatch};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(BigInt),
    Float(f64),
    String(EcoString),
    BitArray(BitArrayValue),
    UtfCodepoint(char),
    Custom(CustomValue),
    External(ExternalValue),
    Bool(bool),
    Nil,
    Tuple(Vec<Value>),
    List(ListValue),
    Function(FunctionValue),
}

impl Value {
    pub fn inspect(&self) -> ValueInspection<'_> {
        ValueInspection::new(self)
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::Float(_) => ValueType::Float,
            Self::String(_) => ValueType::String,
            Self::BitArray(_) => ValueType::BitArray,
            Self::UtfCodepoint(_) => ValueType::UtfCodepoint,
            Self::Custom(value) => ValueType::Custom(value.type_().clone()),
            Self::External(value) => ValueType::External(value.type_().clone()),
            Self::Bool(_) => ValueType::Bool,
            Self::Nil => ValueType::Nil,
            Self::Tuple(values) => ValueType::Tuple(values.iter().map(Self::value_type).collect()),
            Self::List(value) => ValueType::List(Box::new(value.item_type())),
            Self::Function(value) => ValueType::Function(Box::new(value.type_())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BitArrayValue, CustomValue, ExternalValue, ListValue, Value, ValueType};
    use crate::host::HostExternalStore;
    use crate::plan::{CustomType, CustomTypeName, ExternalType, ExternalTypeName};

    #[test]
    fn value_type_preserves_tuple_element_families() {
        fn source_hash(
            context: &crate::host::HostExternalHashing<'_>,
            value: &crate::host::HostStoredValue<num_bigint::BigInt>,
        ) -> u64 {
            context.stored_value_hash(value)
        }

        fn inspect(
            context: &crate::host::HostExternalInspection<'_>,
            value: &crate::host::HostStoredValue<num_bigint::BigInt>,
        ) -> ecow::EcoString {
            context.inspect_stored_value(value)
        }

        assert_eq!(Value::Float(1.0).value_type(), ValueType::Float);
        assert_eq!(Value::String("one".into()).value_type(), ValueType::String);
        assert_eq!(
            Value::BitArray(BitArrayValue::from_bytes(vec![1])).value_type(),
            ValueType::BitArray,
        );
        assert_eq!(
            Value::UtfCodepoint('\u{10ffff}').value_type(),
            ValueType::UtfCodepoint,
        );
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        assert_eq!(
            Value::Custom(CustomValue::from_evaluated(
                custom_type.clone(),
                "Boxed".into(),
                0,
                Vec::new(),
            ))
            .value_type(),
            ValueType::Custom(custom_type),
        );
        let external_type = ExternalType::new(
            ExternalTypeName::new("application".into(), "main".into(), "Resource".into()),
            Vec::new(),
        );
        let store = HostExternalStore::default();
        let source_equal =
            |context: &crate::host::HostExternalEquality<'_>,
             left: &crate::host::HostStoredValue<num_bigint::BigInt>,
             right: &crate::host::HostStoredValue<num_bigint::BigInt>| {
                context.stored_values_equal(left, right)
            };
        let first = store.insert(
            crate::host::HostStoredValue::new(crate::runtime::StoredRuntimeValue::test_int(
                7.into(),
            )),
            source_equal,
            source_hash,
            inspect,
        );
        let equal = store.insert(
            crate::host::HostStoredValue::new(crate::runtime::StoredRuntimeValue::test_int(
                7.into(),
            )),
            source_equal,
            source_hash,
            inspect,
        );
        let stored_equal =
            |left: &crate::runtime::StoredRuntimeValue,
             right: &crate::runtime::StoredRuntimeValue| left.value() == right.value();
        let equality = crate::host::HostExternalEquality::new(&stored_equal);
        assert!(first.source_equal(&equality, &equal));
        let stored_hash = |_: &crate::runtime::StoredRuntimeValue| 7;
        let stored_inspect = |_: &crate::runtime::StoredRuntimeValue| "Resource(7)".into();
        assert_eq!(
            first.source_hash(&crate::host::HostExternalHashing::new(&stored_hash)),
            7,
        );
        assert_eq!(
            first.inspection(&crate::host::HostExternalInspection::new(&stored_inspect)),
            "Resource(7)",
        );
        assert_eq!(
            Value::External(ExternalValue::from_evaluated(
                external_type.clone(),
                first,
                "Resource(7)".into(),
            ))
            .value_type(),
            ValueType::External(external_type),
        );
        assert_eq!(Value::Bool(true).value_type(), ValueType::Bool);
        assert_eq!(Value::Nil.value_type(), ValueType::Nil);
        assert_eq!(
            Value::Tuple(vec![Value::Int(1.into()), Value::String("one".into())]).value_type(),
            ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
        );
        assert_eq!(
            Value::List(ListValue::int(vec![1.into()])).value_type(),
            ValueType::List(Box::new(ValueType::Int)),
        );
        let function = super::FunctionValue::new(
            crate::plan::execution::function::RuntimeFunctionId::Core(
                crate::plan::execution::function::CoreRuntimeFunctionId::Int(
                    crate::plan::execution::function::IntFunctionId(0),
                ),
            ),
            Vec::new(),
            crate::plan::FunctionType::new(Vec::new(), ValueType::Int),
        );
        assert_eq!(
            Value::Function(function).value_type(),
            ValueType::Function(Box::new(crate::plan::FunctionType::new(
                Vec::new(),
                ValueType::Int,
            ))),
        );
    }
}
