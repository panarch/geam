mod capture;
mod function;
mod list;

use ecow::EcoString;
use num_bigint::BigInt;

use super::ParamLocal;

pub(crate) use self::capture::{CaptureValue, CaptureValueKind};
pub use self::function::FunctionValue;
pub(crate) use self::function::{
    BoolFunctionValue, FloatFunctionValue, FunctionFunctionValue, FunctionValueKind,
    IntFunctionValue, ListFunctionValue, NilFunctionValue, StringFunctionValue, TupleFunctionValue,
};
pub use self::list::ListValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    Int,
    Float,
    String,
    Bool,
    Nil,
    Tuple(Vec<ValueType>),
    List(Box<ValueType>),
    Function(Box<FunctionType>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    arguments: Vec<ValueType>,
    return_: Box<ValueType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(BigInt),
    Float(f64),
    String(EcoString),
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
            Self::Bool(_) => ValueType::Bool,
            Self::Nil => ValueType::Nil,
            Self::Tuple(values) => ValueType::Tuple(values.iter().map(Self::value_type).collect()),
            Self::List(value) => ValueType::List(Box::new(value.item_type())),
            Self::Function(value) => ValueType::Function(Box::new(value.type_())),
        }
    }
}

impl FunctionType {
    pub(crate) fn new(arguments: Vec<ValueType>, return_: ValueType) -> Self {
        Self {
            arguments,
            return_: Box::new(return_),
        }
    }

    pub(crate) fn from_params(params: &[ParamLocal], return_: ValueType) -> Self {
        Self::new(params.iter().map(ParamLocal::value_type).collect(), return_)
    }

    pub fn return_(&self) -> &ValueType {
        &self.return_
    }

    pub fn argument_types(&self) -> &[ValueType] {
        &self.arguments
    }
}

#[cfg(test)]
mod tests {
    use super::{ListValue, Value, ValueType};

    #[test]
    fn value_type_preserves_tuple_element_families() {
        assert_eq!(Value::Float(1.0).value_type(), ValueType::Float);
        assert_eq!(Value::String("one".into()).value_type(), ValueType::String);
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
    }
}
