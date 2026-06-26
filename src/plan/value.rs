use ecow::EcoString;
use num_bigint::BigInt;

use super::{
    BoolFunctionId, IntFunctionId, LocalId, NilFunctionId, RuntimeFunctionId, StringFunctionId,
};

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
    kind: FunctionValueKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FunctionValueKind {
    Int(IntFunctionValue),
    String(StringFunctionValue),
    Bool(BoolFunctionValue),
    Nil(NilFunctionValue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IntFunctionValue {
    runtime_id: IntFunctionId,
    params: Vec<LocalId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringFunctionValue {
    runtime_id: StringFunctionId,
    params: Vec<LocalId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BoolFunctionValue {
    runtime_id: BoolFunctionId,
    params: Vec<LocalId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NilFunctionValue {
    runtime_id: NilFunctionId,
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
        let kind = match runtime_id {
            RuntimeFunctionId::Int(runtime_id) => {
                FunctionValueKind::Int(IntFunctionValue::new(runtime_id, params))
            }
            RuntimeFunctionId::String(runtime_id) => {
                FunctionValueKind::String(StringFunctionValue::new(runtime_id, params))
            }
            RuntimeFunctionId::Bool(runtime_id) => {
                FunctionValueKind::Bool(BoolFunctionValue::new(runtime_id, params))
            }
            RuntimeFunctionId::Nil(runtime_id) => {
                FunctionValueKind::Nil(NilFunctionValue::new(runtime_id, params))
            }
        };

        Self { kind }
    }

    pub fn type_(&self) -> FunctionType {
        match &self.kind {
            FunctionValueKind::Int(value) => value.type_(),
            FunctionValueKind::String(value) => value.type_(),
            FunctionValueKind::Bool(value) => value.type_(),
            FunctionValueKind::Nil(value) => value.type_(),
        }
    }

    pub(crate) fn kind(&self) -> &FunctionValueKind {
        &self.kind
    }
}

impl IntFunctionValue {
    pub(crate) fn new(runtime_id: IntFunctionId, params: Vec<LocalId>) -> Self {
        Self { runtime_id, params }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        function_type(&self.params, ValueType::Int)
    }

    pub(crate) fn runtime_id(&self) -> IntFunctionId {
        self.runtime_id
    }
}

impl StringFunctionValue {
    pub(crate) fn new(runtime_id: StringFunctionId, params: Vec<LocalId>) -> Self {
        Self { runtime_id, params }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        function_type(&self.params, ValueType::String)
    }

    pub(crate) fn runtime_id(&self) -> StringFunctionId {
        self.runtime_id
    }
}

impl BoolFunctionValue {
    pub(crate) fn new(runtime_id: BoolFunctionId, params: Vec<LocalId>) -> Self {
        Self { runtime_id, params }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        function_type(&self.params, ValueType::Bool)
    }

    pub(crate) fn runtime_id(&self) -> BoolFunctionId {
        self.runtime_id
    }
}

impl NilFunctionValue {
    pub(crate) fn new(runtime_id: NilFunctionId, params: Vec<LocalId>) -> Self {
        Self { runtime_id, params }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        function_type(&self.params, ValueType::Nil)
    }

    pub(crate) fn runtime_id(&self) -> NilFunctionId {
        self.runtime_id
    }
}

fn function_type(params: &[LocalId], return_: ValueType) -> FunctionType {
    FunctionType::new(
        params
            .iter()
            .map(FunctionArgumentType::from_local)
            .collect(),
        return_,
    )
}

impl From<IntFunctionValue> for FunctionValue {
    fn from(value: IntFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Int(value),
        }
    }
}

impl From<StringFunctionValue> for FunctionValue {
    fn from(value: StringFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::String(value),
        }
    }
}

impl From<BoolFunctionValue> for FunctionValue {
    fn from(value: BoolFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Bool(value),
        }
    }
}

impl From<NilFunctionValue> for FunctionValue {
    fn from(value: NilFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Nil(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoolFunctionValue, FunctionArgumentType, FunctionType, FunctionValue, IntFunctionValue,
        NilFunctionValue, StringFunctionValue, ValueType,
    };
    use crate::plan::{
        BoolFunctionId, BoolLocalId, IntFunctionId, IntLocalId, LocalId, NilFunctionId, NilLocalId,
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
    }

    #[test]
    fn function_value_type_uses_runtime_id_for_return_type() {
        let value = FunctionValue::new(RuntimeFunctionId::Nil(NilFunctionId(0)), Vec::new());

        assert_eq!(value.type_(), FunctionType::new(Vec::new(), ValueType::Nil));
    }

    #[test]
    fn function_value_conversions_preserve_return_family() {
        let int: FunctionValue = IntFunctionValue::new(IntFunctionId(0), Vec::new()).into();
        let string: FunctionValue =
            StringFunctionValue::new(StringFunctionId(0), Vec::new()).into();
        let bool: FunctionValue = BoolFunctionValue::new(BoolFunctionId(0), Vec::new()).into();
        let nil: FunctionValue = NilFunctionValue::new(NilFunctionId(0), Vec::new()).into();

        assert_eq!(int.type_().return_(), &ValueType::Int);
        assert_eq!(string.type_().return_(), &ValueType::String);
        assert_eq!(bool.type_().return_(), &ValueType::Bool);
        assert_eq!(nil.type_().return_(), &ValueType::Nil);
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
