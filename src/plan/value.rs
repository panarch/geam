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
    type_: FunctionType,
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
    pub(crate) fn new(
        type_: FunctionType,
        runtime_id: RuntimeFunctionId,
        params: Vec<LocalId>,
    ) -> Self {
        Self {
            type_,
            runtime_id,
            params,
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn runtime_id(&self) -> RuntimeFunctionId {
        self.runtime_id
    }

    pub(crate) fn params(&self) -> &[LocalId] {
        &self.params
    }
}
