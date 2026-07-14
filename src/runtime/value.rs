mod bit_array;
mod capture;
mod function;
mod list;

use ecow::EcoString;
use num_bigint::BigInt;

use crate::plan::ValueType;

pub use self::bit_array::{BitArrayValue, BitArrayValueLengthError};
pub(crate) use self::capture::{CaptureListValue, CaptureValue};
pub use self::function::FunctionValue;
pub(crate) use self::function::{
    BitArrayFunctionValue, BoolFunctionValue, FloatFunctionValue, FunctionFunctionValue,
    FunctionValueKind, IntFunctionValue, ListFunctionValue, NilFunctionValue, StringFunctionValue,
    TupleFunctionValue, UtfCodepointFunctionValue,
};
pub use self::list::{ListValue, ListValueItemTypeMismatch};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(BigInt),
    Float(f64),
    String(EcoString),
    BitArray(BitArrayValue),
    UtfCodepoint(char),
    Bool(bool),
    Nil,
    Tuple(Vec<Value>),
    List(ListValue),
    Function(FunctionValue),
}

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::Float(_) => ValueType::Float,
            Self::String(_) => ValueType::String,
            Self::BitArray(_) => ValueType::BitArray,
            Self::UtfCodepoint(_) => ValueType::UtfCodepoint,
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
    use super::{BitArrayValue, ListValue, Value, ValueType};

    #[test]
    fn value_type_preserves_tuple_element_families() {
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
            crate::plan::execution::RuntimeFunctionId::Int(crate::plan::execution::IntFunctionId(
                0,
            )),
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
