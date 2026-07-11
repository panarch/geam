use super::CaptureValue;
use crate::plan::FunctionType;

#[cfg(test)]
use crate::plan::execution::RuntimeFunctionId;
use crate::plan::execution::{
    BoolFunctionId, FloatFunctionId, FunctionFunctionId, FunctionReturnFamily, IntFunctionId,
    ListFunctionId, NilFunctionId, ParamLocal, StringFunctionId, TupleFunctionId,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionValue {
    kind: FunctionValueKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionValueKind {
    Int(IntFunctionValue),
    Float(FloatFunctionValue),
    String(StringFunctionValue),
    Bool(BoolFunctionValue),
    Nil(NilFunctionValue),
    Tuple(TupleFunctionValue),
    List(ListFunctionValue),
    Function(FunctionFunctionValue),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct IntFunctionValue {
    runtime_id: IntFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FloatFunctionValue {
    runtime_id: FloatFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StringFunctionValue {
    runtime_id: StringFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BoolFunctionValue {
    runtime_id: BoolFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NilFunctionValue {
    runtime_id: NilFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TupleFunctionValue {
    runtime_id: TupleFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ListFunctionValue {
    runtime_id: ListFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FunctionFunctionValue {
    runtime_id: FunctionFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

impl FunctionValue {
    #[cfg(test)]
    pub(crate) fn new(
        runtime_id: RuntimeFunctionId,
        params: Vec<ParamLocal>,
        type_: FunctionType,
    ) -> Self {
        let kind = match runtime_id {
            RuntimeFunctionId::Int(runtime_id) => FunctionValueKind::Int(
                IntFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
            ),
            RuntimeFunctionId::Float(runtime_id) => FunctionValueKind::Float(
                FloatFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
            ),
            RuntimeFunctionId::String(runtime_id) => FunctionValueKind::String(
                StringFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
            ),
            RuntimeFunctionId::Bool(runtime_id) => FunctionValueKind::Bool(
                BoolFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
            ),
            RuntimeFunctionId::Nil(runtime_id) => FunctionValueKind::Nil(
                NilFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
            ),
            RuntimeFunctionId::Tuple { id, return_type } => {
                let _ = return_type;
                FunctionValueKind::Tuple(TupleFunctionValue::from_evaluated(
                    id,
                    params,
                    Vec::new(),
                    type_,
                ))
            }
            RuntimeFunctionId::List(id) => FunctionValueKind::List(
                ListFunctionValue::new_with_captures(id, params, Vec::new(), type_),
            ),
            RuntimeFunctionId::Function { id, return_type } => {
                let _ = return_type;
                FunctionValueKind::Function(FunctionFunctionValue::from_evaluated(
                    id,
                    params,
                    Vec::new(),
                    type_,
                ))
            }
        };

        Self { kind }
    }

    pub fn type_(&self) -> FunctionType {
        match &self.kind {
            FunctionValueKind::Int(value) => value.type_(),
            FunctionValueKind::Float(value) => value.type_(),
            FunctionValueKind::String(value) => value.type_(),
            FunctionValueKind::Bool(value) => value.type_(),
            FunctionValueKind::Nil(value) => value.type_(),
            FunctionValueKind::Tuple(value) => value.type_(),
            FunctionValueKind::List(value) => value.type_(),
            FunctionValueKind::Function(value) => value.type_(),
        }
    }

    pub(crate) fn kind(&self) -> &FunctionValueKind {
        &self.kind
    }
}

impl FunctionValueKind {
    pub(crate) fn family(&self) -> FunctionReturnFamily {
        match self {
            Self::Int(_) => FunctionReturnFamily::Int,
            Self::Float(_) => FunctionReturnFamily::Float,
            Self::String(_) => FunctionReturnFamily::String,
            Self::Bool(_) => FunctionReturnFamily::Bool,
            Self::Nil(_) => FunctionReturnFamily::Nil,
            Self::Tuple(_) => FunctionReturnFamily::Tuple,
            Self::List(_) => FunctionReturnFamily::List,
            Self::Function(_) => FunctionReturnFamily::Function,
        }
    }
}

impl IntFunctionValue {
    #[cfg(test)]
    pub(crate) fn new(
        runtime_id: IntFunctionId,
        params: Vec<ParamLocal>,
        type_: FunctionType,
    ) -> Self {
        Self::new_with_captures(runtime_id, params, Vec::new(), type_)
    }

    pub(crate) fn new_with_captures(
        runtime_id: IntFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }

    pub(crate) fn runtime_id(&self) -> IntFunctionId {
        self.runtime_id
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }
}

impl FloatFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: FloatFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }

    pub(crate) fn runtime_id(&self) -> FloatFunctionId {
        self.runtime_id
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }
}

impl StringFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: StringFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }

    pub(crate) fn runtime_id(&self) -> StringFunctionId {
        self.runtime_id
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }
}

impl BoolFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: BoolFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }

    pub(crate) fn runtime_id(&self) -> BoolFunctionId {
        self.runtime_id
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }
}

impl NilFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: NilFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }

    pub(crate) fn runtime_id(&self) -> NilFunctionId {
        self.runtime_id
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }
}

impl TupleFunctionValue {
    pub(crate) fn from_evaluated(
        runtime_id: TupleFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }

    pub(crate) fn runtime_id(&self) -> TupleFunctionId {
        self.runtime_id
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }
}

impl ListFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: ListFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }

    pub(crate) fn runtime_id(&self) -> ListFunctionId {
        self.runtime_id.clone()
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }
}

impl FunctionFunctionValue {
    #[cfg(test)]
    pub(crate) fn new(
        runtime_id: FunctionFunctionId,
        params: Vec<ParamLocal>,
        type_: FunctionType,
    ) -> Self {
        Self::from_evaluated(runtime_id, params, Vec::new(), type_)
    }

    pub(crate) fn from_evaluated(
        runtime_id: FunctionFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }

    pub(crate) fn runtime_id(&self) -> FunctionFunctionId {
        self.runtime_id.clone()
    }

    pub(crate) fn captures(&self) -> &[CaptureValue] {
        &self.captures
    }
}

impl From<IntFunctionValue> for FunctionValue {
    fn from(value: IntFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Int(value),
        }
    }
}

impl From<FloatFunctionValue> for FunctionValue {
    fn from(value: FloatFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Float(value),
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

impl From<TupleFunctionValue> for FunctionValue {
    fn from(value: TupleFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Tuple(value),
        }
    }
}

impl From<ListFunctionValue> for FunctionValue {
    fn from(value: ListFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::List(value),
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
    use super::FunctionValue;
    use crate::plan::execution::FunctionReturnFamily;
    use crate::plan::{FunctionType, ValueType};

    #[test]
    fn function_value_preserves_every_lowered_return_family() {
        let cases = [
            (
                "pub fn main() -> Int { 1 }",
                ValueType::Int,
                FunctionReturnFamily::Int,
            ),
            (
                "pub fn main() -> Float { 1.0 }",
                ValueType::Float,
                FunctionReturnFamily::Float,
            ),
            (
                "pub fn main() -> String { \"one\" }",
                ValueType::String,
                FunctionReturnFamily::String,
            ),
            (
                "pub fn main() -> Bool { True }",
                ValueType::Bool,
                FunctionReturnFamily::Bool,
            ),
            (
                "pub fn main() -> Nil { Nil }",
                ValueType::Nil,
                FunctionReturnFamily::Nil,
            ),
            (
                "pub fn main() -> #(Int) { #(1) }",
                ValueType::Tuple(vec![ValueType::Int]),
                FunctionReturnFamily::Tuple,
            ),
            (
                "pub fn main() -> List(Int) { [] }",
                ValueType::List(Box::new(ValueType::Int)),
                FunctionReturnFamily::List,
            ),
            (
                "pub fn main() -> fn() -> Int { fn() { 1 } }",
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
                FunctionReturnFamily::Function,
            ),
        ];

        for (source, return_type, family) in cases {
            let plan = crate::runtime::plan_src(source);
            let value = FunctionValue::new(
                plan.main_runtime(),
                Vec::new(),
                FunctionType::new(Vec::new(), return_type.clone()),
            );

            assert_eq!(value.type_(), FunctionType::new(Vec::new(), return_type));
            assert_eq!(value.kind().family(), family);
        }
    }
}
