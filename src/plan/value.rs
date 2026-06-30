use ecow::EcoString;
use num_bigint::BigInt;

use super::{
    BoolFunctionId, BoolFunctionLocalId, BoolLocalId, FunctionFunctionId, FunctionFunctionLocalId,
    IntFunctionId, IntFunctionLocalId, IntLocalId, NilFunctionId, NilFunctionLocalId, NilLocalId,
    ParamLocal, RuntimeFunctionId, StringFunctionId, StringFunctionLocalId, StringLocalId,
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
    arguments: Vec<ValueType>,
    return_: Box<ValueType>,
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
    Function(FunctionFunctionValue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IntFunctionValue {
    runtime_id: IntFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringFunctionValue {
    runtime_id: StringFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BoolFunctionValue {
    runtime_id: BoolFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NilFunctionValue {
    runtime_id: NilFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FunctionFunctionValue {
    runtime_id: FunctionFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    return_type: FunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CaptureValue {
    kind: CaptureValueKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CaptureValueKind {
    Int {
        local: IntLocalId,
        value: BigInt,
    },
    String {
        local: StringLocalId,
        value: EcoString,
    },
    Bool {
        local: BoolLocalId,
        value: bool,
    },
    Nil {
        local: NilLocalId,
    },
    IntFunction {
        local: IntFunctionLocalId,
        value: IntFunctionValue,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: StringFunctionValue,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: BoolFunctionValue,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: NilFunctionValue,
    },
    FunctionFunction {
        local: FunctionFunctionLocalId,
        value: FunctionFunctionValue,
    },
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

impl FunctionValue {
    pub(crate) fn new(runtime_id: RuntimeFunctionId, params: Vec<ParamLocal>) -> Self {
        Self::new_with_captures(runtime_id, params, Vec::new())
    }

    pub(crate) fn new_with_captures(
        runtime_id: RuntimeFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
    ) -> Self {
        let kind = match runtime_id {
            RuntimeFunctionId::Int(runtime_id) => FunctionValueKind::Int(
                IntFunctionValue::new_with_captures(runtime_id, params, captures),
            ),
            RuntimeFunctionId::String(runtime_id) => FunctionValueKind::String(
                StringFunctionValue::new_with_captures(runtime_id, params, captures),
            ),
            RuntimeFunctionId::Bool(runtime_id) => FunctionValueKind::Bool(
                BoolFunctionValue::new_with_captures(runtime_id, params, captures),
            ),
            RuntimeFunctionId::Nil(runtime_id) => FunctionValueKind::Nil(
                NilFunctionValue::new_with_captures(runtime_id, params, captures),
            ),
            RuntimeFunctionId::Function { id, return_type } => FunctionValueKind::Function(
                FunctionFunctionValue::new_with_captures(id, params, captures, return_type),
            ),
        };

        Self { kind }
    }

    pub fn type_(&self) -> FunctionType {
        match &self.kind {
            FunctionValueKind::Int(value) => value.type_(),
            FunctionValueKind::String(value) => value.type_(),
            FunctionValueKind::Bool(value) => value.type_(),
            FunctionValueKind::Nil(value) => value.type_(),
            FunctionValueKind::Function(value) => value.type_(),
        }
    }

    pub(crate) fn kind(&self) -> &FunctionValueKind {
        &self.kind
    }

    #[cfg(test)]
    pub(crate) fn params(&self) -> &[ParamLocal] {
        match &self.kind {
            FunctionValueKind::Int(value) => value.params(),
            FunctionValueKind::String(value) => value.params(),
            FunctionValueKind::Bool(value) => value.params(),
            FunctionValueKind::Nil(value) => value.params(),
            FunctionValueKind::Function(value) => value.params(),
        }
    }
}

impl IntFunctionValue {
    #[cfg(test)]
    pub(crate) fn new(runtime_id: IntFunctionId, params: Vec<ParamLocal>) -> Self {
        Self::new_with_captures(runtime_id, params, Vec::new())
    }

    pub(crate) fn new_with_captures(
        runtime_id: IntFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        FunctionType::from_params(&self.params, ValueType::Int)
    }

    pub(crate) fn runtime_id(&self) -> IntFunctionId {
        self.runtime_id
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }

    #[cfg(test)]
    pub(crate) fn params(&self) -> &[ParamLocal] {
        &self.params
    }
}

impl StringFunctionValue {
    #[cfg(test)]
    pub(crate) fn new(runtime_id: StringFunctionId, params: Vec<ParamLocal>) -> Self {
        Self::new_with_captures(runtime_id, params, Vec::new())
    }

    pub(crate) fn new_with_captures(
        runtime_id: StringFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        FunctionType::from_params(&self.params, ValueType::String)
    }

    pub(crate) fn runtime_id(&self) -> StringFunctionId {
        self.runtime_id
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }

    #[cfg(test)]
    pub(crate) fn params(&self) -> &[ParamLocal] {
        &self.params
    }
}

impl BoolFunctionValue {
    #[cfg(test)]
    pub(crate) fn new(runtime_id: BoolFunctionId, params: Vec<ParamLocal>) -> Self {
        Self::new_with_captures(runtime_id, params, Vec::new())
    }

    pub(crate) fn new_with_captures(
        runtime_id: BoolFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        FunctionType::from_params(&self.params, ValueType::Bool)
    }

    pub(crate) fn runtime_id(&self) -> BoolFunctionId {
        self.runtime_id
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }

    #[cfg(test)]
    pub(crate) fn params(&self) -> &[ParamLocal] {
        &self.params
    }
}

impl NilFunctionValue {
    #[cfg(test)]
    pub(crate) fn new(runtime_id: NilFunctionId, params: Vec<ParamLocal>) -> Self {
        Self::new_with_captures(runtime_id, params, Vec::new())
    }

    pub(crate) fn new_with_captures(
        runtime_id: NilFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        FunctionType::from_params(&self.params, ValueType::Nil)
    }

    pub(crate) fn runtime_id(&self) -> NilFunctionId {
        self.runtime_id
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }

    #[cfg(test)]
    pub(crate) fn params(&self) -> &[ParamLocal] {
        &self.params
    }
}

impl FunctionFunctionValue {
    #[cfg(test)]
    pub(crate) fn new(
        runtime_id: FunctionFunctionId,
        params: Vec<ParamLocal>,
        return_type: FunctionType,
    ) -> Self {
        Self::new_with_captures(runtime_id, params, Vec::new(), return_type)
    }

    pub(crate) fn new_with_captures(
        runtime_id: FunctionFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        return_type: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            return_type,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        FunctionType::from_params(
            &self.params,
            ValueType::Function(Box::new(self.return_type.clone())),
        )
    }

    pub(crate) fn runtime_id(&self) -> FunctionFunctionId {
        self.runtime_id
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }

    #[cfg(test)]
    pub(crate) fn params(&self) -> &[ParamLocal] {
        &self.params
    }
}

impl CaptureValue {
    pub(crate) fn int(local: IntLocalId, value: BigInt) -> Self {
        Self {
            kind: CaptureValueKind::Int { local, value },
        }
    }

    pub(crate) fn string(local: StringLocalId, value: EcoString) -> Self {
        Self {
            kind: CaptureValueKind::String { local, value },
        }
    }

    pub(crate) fn bool(local: BoolLocalId, value: bool) -> Self {
        Self {
            kind: CaptureValueKind::Bool { local, value },
        }
    }

    pub(crate) fn nil(local: NilLocalId) -> Self {
        Self {
            kind: CaptureValueKind::Nil { local },
        }
    }

    pub(crate) fn int_function(local: IntFunctionLocalId, value: IntFunctionValue) -> Self {
        Self {
            kind: CaptureValueKind::IntFunction { local, value },
        }
    }

    pub(crate) fn string_function(
        local: StringFunctionLocalId,
        value: StringFunctionValue,
    ) -> Self {
        Self {
            kind: CaptureValueKind::StringFunction { local, value },
        }
    }

    pub(crate) fn bool_function(local: BoolFunctionLocalId, value: BoolFunctionValue) -> Self {
        Self {
            kind: CaptureValueKind::BoolFunction { local, value },
        }
    }

    pub(crate) fn nil_function(local: NilFunctionLocalId, value: NilFunctionValue) -> Self {
        Self {
            kind: CaptureValueKind::NilFunction { local, value },
        }
    }

    pub(crate) fn function_function(
        local: FunctionFunctionLocalId,
        value: FunctionFunctionValue,
    ) -> Self {
        Self {
            kind: CaptureValueKind::FunctionFunction { local, value },
        }
    }

    pub(crate) fn kind(&self) -> &CaptureValueKind {
        &self.kind
    }
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

impl From<FunctionFunctionValue> for FunctionValue {
    fn from(value: FunctionFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Function(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoolFunctionValue, FunctionFunctionValue, FunctionType, FunctionValue, IntFunctionValue,
        NilFunctionValue, StringFunctionValue, ValueType,
    };
    use crate::plan::{
        BoolFunctionId, BoolFunctionLocalId, BoolLocalId, FunctionFunctionId, FunctionValueKind,
        IntFunctionFunctionId, IntFunctionId, IntLocalId, NilFunctionId, NilLocalId, ParamLocal,
        RuntimeFunctionId, StringFunctionId, StringLocalId,
    };

    #[test]
    fn function_value_accepts_matching_shape() {
        let value = FunctionValue::new(
            RuntimeFunctionId::String(StringFunctionId(0)),
            vec![int_param(0)],
        );
        let type_ = value.type_();

        assert_eq!(
            type_,
            FunctionType::new(vec![ValueType::Int], ValueType::String),
        );
        assert_eq!(type_.argument_types(), &[ValueType::Int]);
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
        let function: FunctionValue = FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        )
        .into();

        assert_eq!(int.type_().return_(), &ValueType::Int);
        assert_eq!(string.type_().return_(), &ValueType::String);
        assert_eq!(bool.type_().return_(), &ValueType::Bool);
        assert_eq!(nil.type_().return_(), &ValueType::Nil);
        assert!(matches!(function.type_().return_(), ValueType::Function(_)));
    }

    #[test]
    fn function_value_type_uses_all_parameter_shapes() {
        let argument_function = FunctionType::new(vec![ValueType::String], ValueType::Bool);
        let value = FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![
                int_param(0),
                string_param(0),
                bool_param(0),
                nil_param(0),
                ParamLocal::bool_function(BoolFunctionLocalId(0), argument_function.clone()),
            ],
        );

        assert_eq!(
            value.type_(),
            FunctionType::new(
                vec![
                    ValueType::Int,
                    ValueType::String,
                    ValueType::Bool,
                    ValueType::Nil,
                    ValueType::Function(Box::new(argument_function)),
                ],
                ValueType::Int,
            ),
        );
    }

    #[test]
    fn function_value_type_uses_function_return_type_metadata() {
        let return_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let value = FunctionValue::new(
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                return_type: return_type.clone(),
            },
            vec![bool_param(0)],
        );

        assert_eq!(
            value.type_(),
            FunctionType::new(
                vec![ValueType::Bool],
                ValueType::Function(Box::new(return_type)),
            ),
        );
    }

    #[test]
    fn function_value_preserves_exact_parameter_slots() {
        let params = vec![int_param(2), bool_param(1)];
        let int = FunctionValue::new(RuntimeFunctionId::Int(IntFunctionId(0)), params.clone());
        let string = FunctionValue::new(
            RuntimeFunctionId::String(StringFunctionId(0)),
            params.clone(),
        );
        let bool = FunctionValue::new(RuntimeFunctionId::Bool(BoolFunctionId(0)), params.clone());
        let nil = FunctionValue::new(RuntimeFunctionId::Nil(NilFunctionId(0)), params.clone());
        let function = FunctionValue::new(
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Int),
            },
            params.clone(),
        );

        assert_eq!(int.params(), params);
        assert_eq!(string.params(), params);
        assert_eq!(bool.params(), params);
        assert_eq!(nil.params(), params);
        assert_eq!(function.params(), params);
        assert!(matches!(int.kind(), FunctionValueKind::Int(_)));
    }

    fn int_param(index: usize) -> ParamLocal {
        ParamLocal::int(IntLocalId(index))
    }

    fn string_param(index: usize) -> ParamLocal {
        ParamLocal::string(StringLocalId(index))
    }

    fn bool_param(index: usize) -> ParamLocal {
        ParamLocal::bool(BoolLocalId(index))
    }

    fn nil_param(index: usize) -> ParamLocal {
        ParamLocal::nil(NilLocalId(index))
    }
}
