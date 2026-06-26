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
    arguments: Vec<ValueType>,
    return_: Box<ValueType>,
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
    pub(crate) fn new(arguments: Vec<ValueType>, return_: ValueType) -> Self {
        Self {
            arguments,
            return_: Box::new(return_),
        }
    }

    pub fn arguments(&self) -> &[ValueType] {
        &self.arguments
    }

    pub fn return_(&self) -> &ValueType {
        &self.return_
    }
}

impl FunctionValue {
    pub(crate) fn new(runtime_id: RuntimeFunctionId, params: Vec<LocalId>) -> Self {
        Self { runtime_id, params }
    }

    pub fn type_(&self) -> FunctionType {
        FunctionType::new(
            self.params.iter().map(|param| param.value_type()).collect(),
            self.runtime_id.value_type(),
        )
    }

    pub(crate) fn runtime_id(&self) -> RuntimeFunctionId {
        self.runtime_id
    }

    pub(crate) fn params(&self) -> &[LocalId] {
        &self.params
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionType, FunctionValue, ValueType};
    use crate::plan::{IntLocalId, LocalId, NilFunctionId, RuntimeFunctionId, StringFunctionId};

    #[test]
    fn function_value_accepts_matching_shape() {
        let value = FunctionValue::new(
            RuntimeFunctionId::String(StringFunctionId(0)),
            vec![LocalId::Int(IntLocalId(0))],
        );
        let type_ = value.type_();

        assert_eq!(type_.arguments(), &[ValueType::Int]);
        assert_eq!(type_.return_(), &ValueType::String);
        assert_eq!(
            value.runtime_id(),
            RuntimeFunctionId::String(StringFunctionId(0))
        );
        assert_eq!(value.params(), &[LocalId::Int(IntLocalId(0))]);
    }

    #[test]
    fn function_value_type_uses_runtime_id_for_return_type() {
        let value = FunctionValue::new(RuntimeFunctionId::Nil(NilFunctionId(0)), Vec::new());

        assert_eq!(value.type_(), FunctionType::new(Vec::new(), ValueType::Nil));
    }
}
