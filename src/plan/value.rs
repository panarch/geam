use ecow::EcoString;
use num_bigint::BigInt;

use super::{LocalId, RuntimeFunctionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    Int,
    String,
    Bool,
    Nil,
    Function(Box<FunctionType>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    arguments: Vec<FunctionArgumentType>,
    return_: Box<ValueType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionArgumentType {
    Int,
    String,
    Bool,
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionValue {
    runtime_id: RuntimeFunctionId,
    params: Vec<LocalId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(BigInt),
    String(EcoString),
    Bool(bool),
    Nil,
    Function(FunctionValue),
}

impl FunctionType {
    pub(crate) fn new(arguments: Vec<FunctionArgumentType>, return_: ValueType) -> Self {
        Self {
            arguments,
            return_: Box::new(return_),
        }
    }

    pub fn return_(&self) -> &ValueType {
        &self.return_
    }

    pub(crate) fn argument_types(&self) -> &[FunctionArgumentType] {
        &self.arguments
    }
}

impl FunctionArgumentType {
    pub(crate) fn from_value_type(type_: &ValueType) -> Option<Self> {
        match type_ {
            ValueType::Int => Some(Self::Int),
            ValueType::String => Some(Self::String),
            ValueType::Bool => Some(Self::Bool),
            ValueType::Nil => Some(Self::Nil),
            ValueType::Function(_) => None,
        }
    }

    pub(crate) fn from_local(local: &LocalId) -> Self {
        match local {
            LocalId::Int(_) => Self::Int,
            LocalId::String(_) => Self::String,
            LocalId::Bool(_) => Self::Bool,
            LocalId::Nil(_) => Self::Nil,
        }
    }
}

impl FunctionValue {
    pub(crate) fn new(runtime_id: RuntimeFunctionId, params: Vec<LocalId>) -> Self {
        Self { runtime_id, params }
    }

    pub fn type_(&self) -> FunctionType {
        FunctionType::new(
            self.params
                .iter()
                .map(FunctionArgumentType::from_local)
                .collect(),
            self.runtime_id.value_type(),
        )
    }

    pub(crate) fn runtime_id(&self) -> RuntimeFunctionId {
        self.runtime_id
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionArgumentType, FunctionType, FunctionValue, ValueType};
    use crate::plan::{
        BoolLocalId, IntFunctionId, IntLocalId, LocalId, NilFunctionId, NilLocalId,
        RuntimeFunctionId, StringFunctionId, StringLocalId,
    };

    #[test]
    fn function_value_accepts_matching_shape() {
        let value = FunctionValue::new(
            RuntimeFunctionId::String(StringFunctionId(0)),
            vec![LocalId::Int(IntLocalId(0))],
        );
        let type_ = value.type_();

        assert_eq!(
            type_,
            FunctionType::new(vec![FunctionArgumentType::Int], ValueType::String),
        );
        assert_eq!(type_.return_(), &ValueType::String);
        assert_eq!(
            value.runtime_id(),
            RuntimeFunctionId::String(StringFunctionId(0))
        );
    }

    #[test]
    fn function_value_type_uses_runtime_id_for_return_type() {
        let value = FunctionValue::new(RuntimeFunctionId::Nil(NilFunctionId(0)), Vec::new());

        assert_eq!(value.type_(), FunctionType::new(Vec::new(), ValueType::Nil));
    }

    #[test]
    fn function_argument_type_rejects_function_shape() {
        assert_eq!(
            FunctionArgumentType::from_value_type(&ValueType::Function(Box::new(
                FunctionType::new(Vec::new(), ValueType::Int),
            ))),
            None,
        );
    }

    #[test]
    fn function_argument_type_accepts_primitive_shapes() {
        assert_eq!(
            FunctionArgumentType::from_value_type(&ValueType::Int),
            Some(FunctionArgumentType::Int),
        );
        assert_eq!(
            FunctionArgumentType::from_value_type(&ValueType::String),
            Some(FunctionArgumentType::String),
        );
        assert_eq!(
            FunctionArgumentType::from_value_type(&ValueType::Bool),
            Some(FunctionArgumentType::Bool),
        );
        assert_eq!(
            FunctionArgumentType::from_value_type(&ValueType::Nil),
            Some(FunctionArgumentType::Nil),
        );
    }

    #[test]
    fn function_value_type_uses_all_parameter_shapes() {
        let value = FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![
                LocalId::Int(IntLocalId(0)),
                LocalId::String(StringLocalId(0)),
                LocalId::Bool(BoolLocalId(0)),
                LocalId::Nil(NilLocalId(0)),
            ],
        );

        assert_eq!(
            value.type_(),
            FunctionType::new(
                vec![
                    FunctionArgumentType::Int,
                    FunctionArgumentType::String,
                    FunctionArgumentType::Bool,
                    FunctionArgumentType::Nil,
                ],
                ValueType::Int,
            ),
        );
    }
}
