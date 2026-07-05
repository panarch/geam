use super::{CaptureValue, FunctionType, ValueType};
use crate::plan::{
    BoolFunctionId, FloatFunctionId, FunctionFunctionId, IntFunctionId, ListFunctionId,
    NilFunctionId, ParamLocal, RuntimeFunctionId, StringFunctionId, TupleFunctionId,
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
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FloatFunctionValue {
    runtime_id: FloatFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StringFunctionValue {
    runtime_id: StringFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BoolFunctionValue {
    runtime_id: BoolFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NilFunctionValue {
    runtime_id: NilFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TupleFunctionValue {
    runtime_id: TupleFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    return_type: Vec<ValueType>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ListFunctionValue {
    runtime_id: ListFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    return_type: ValueType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FunctionFunctionValue {
    runtime_id: FunctionFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    return_type: FunctionType,
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
            RuntimeFunctionId::Float(runtime_id) => FunctionValueKind::Float(
                FloatFunctionValue::new_with_captures(runtime_id, params, captures),
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
            RuntimeFunctionId::Tuple { id, return_type } => FunctionValueKind::Tuple(
                TupleFunctionValue::new_with_captures(id, params, captures, return_type),
            ),
            RuntimeFunctionId::List { id, return_type } => FunctionValueKind::List(
                ListFunctionValue::new_with_captures(id, params, captures, *return_type),
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

    #[cfg(test)]
    pub(crate) fn params(&self) -> &[ParamLocal] {
        match &self.kind {
            FunctionValueKind::Int(value) => value.params(),
            FunctionValueKind::Float(value) => value.params(),
            FunctionValueKind::String(value) => value.params(),
            FunctionValueKind::Bool(value) => value.params(),
            FunctionValueKind::Nil(value) => value.params(),
            FunctionValueKind::Tuple(value) => value.params(),
            FunctionValueKind::List(value) => value.params(),
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

impl FloatFunctionValue {
    #[cfg(test)]
    pub(crate) fn new(runtime_id: FloatFunctionId, params: Vec<ParamLocal>) -> Self {
        Self::new_with_captures(runtime_id, params, Vec::new())
    }

    pub(crate) fn new_with_captures(
        runtime_id: FloatFunctionId,
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
        FunctionType::from_params(&self.params, ValueType::Float)
    }

    pub(crate) fn runtime_id(&self) -> FloatFunctionId {
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

impl TupleFunctionValue {
    #[cfg(test)]
    pub(crate) fn new(
        runtime_id: TupleFunctionId,
        params: Vec<ParamLocal>,
        return_type: Vec<ValueType>,
    ) -> Self {
        Self::new_with_captures(runtime_id, params, Vec::new(), return_type)
    }

    pub(crate) fn new_with_captures(
        runtime_id: TupleFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        return_type: Vec<ValueType>,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            return_type,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        FunctionType::from_params(&self.params, ValueType::Tuple(self.return_type.clone()))
    }

    pub(crate) fn runtime_id(&self) -> TupleFunctionId {
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

impl ListFunctionValue {
    #[cfg(test)]
    pub(crate) fn new(
        runtime_id: ListFunctionId,
        params: Vec<ParamLocal>,
        return_type: ValueType,
    ) -> Self {
        Self::new_with_captures(runtime_id, params, Vec::new(), return_type)
    }

    pub(crate) fn new_with_captures(
        runtime_id: ListFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        return_type: ValueType,
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
            ValueType::List(Box::new(self.return_type.clone())),
        )
    }

    pub(crate) fn runtime_id(&self) -> ListFunctionId {
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
    use super::{
        BoolFunctionValue, FloatFunctionValue, FunctionFunctionValue, FunctionValue,
        FunctionValueKind, IntFunctionValue, ListFunctionValue, NilFunctionValue,
        StringFunctionValue, TupleFunctionValue,
    };
    use crate::plan::{
        BoolFunctionId, BoolFunctionLocalId, BoolLocalId, FloatFunctionId, FloatFunctionLocalId,
        FloatLocalId, FunctionFunctionId, FunctionType, IntFunctionFunctionId, IntFunctionId,
        IntLocalId, ListFunctionId, ListFunctionLocalId, ListLocalId, NilFunctionId, NilLocalId,
        ParamLocal, RuntimeFunctionId, StringFunctionId, StringLocalId, TupleFunctionId,
        TupleFunctionLocalId, TupleLocalId, ValueType,
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
        let float: FunctionValue = FloatFunctionValue::new(FloatFunctionId(0), Vec::new()).into();
        let string: FunctionValue =
            StringFunctionValue::new(StringFunctionId(0), Vec::new()).into();
        let bool: FunctionValue = BoolFunctionValue::new(BoolFunctionId(0), Vec::new()).into();
        let nil: FunctionValue = NilFunctionValue::new(NilFunctionId(0), Vec::new()).into();
        let tuple: FunctionValue =
            TupleFunctionValue::new(TupleFunctionId(0), Vec::new(), vec![ValueType::Int]).into();
        let list: FunctionValue =
            ListFunctionValue::new(ListFunctionId(0), Vec::new(), ValueType::Int).into();
        let function_return_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let function: FunctionValue = FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            function_return_type.clone(),
        )
        .into();

        assert_eq!(int.type_().return_(), &ValueType::Int);
        assert_eq!(float.type_().return_(), &ValueType::Float);
        assert_eq!(string.type_().return_(), &ValueType::String);
        assert_eq!(bool.type_().return_(), &ValueType::Bool);
        assert_eq!(nil.type_().return_(), &ValueType::Nil);
        assert_eq!(
            tuple.type_().return_(),
            &ValueType::Tuple(vec![ValueType::Int])
        );
        assert_eq!(
            list.type_().return_(),
            &ValueType::List(Box::new(ValueType::Int)),
        );
        assert_eq!(
            function.type_().return_(),
            &ValueType::Function(Box::new(function_return_type)),
        );
    }

    #[test]
    fn function_value_type_uses_all_parameter_shapes() {
        let argument_function = FunctionType::new(vec![ValueType::String], ValueType::Bool);
        let value = FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![
                int_param(0),
                float_param(0),
                string_param(0),
                bool_param(0),
                nil_param(0),
                tuple_param(0),
                list_param(0),
                ParamLocal::float_function(FloatFunctionLocalId(0), argument_function.clone()),
                ParamLocal::tuple_function(TupleFunctionLocalId(0), argument_function.clone()),
                ParamLocal::list_function(ListFunctionLocalId(0), argument_function.clone()),
                ParamLocal::bool_function(BoolFunctionLocalId(0), argument_function.clone()),
            ],
        );

        assert_eq!(
            value.type_(),
            FunctionType::new(
                vec![
                    ValueType::Int,
                    ValueType::Float,
                    ValueType::String,
                    ValueType::Bool,
                    ValueType::Nil,
                    ValueType::Tuple(vec![ValueType::Int]),
                    ValueType::List(Box::new(ValueType::Int)),
                    ValueType::Function(Box::new(argument_function.clone())),
                    ValueType::Function(Box::new(argument_function.clone())),
                    ValueType::Function(Box::new(argument_function.clone())),
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
        let float =
            FunctionValue::new(RuntimeFunctionId::Float(FloatFunctionId(0)), params.clone());
        let string = FunctionValue::new(
            RuntimeFunctionId::String(StringFunctionId(0)),
            params.clone(),
        );
        let bool = FunctionValue::new(RuntimeFunctionId::Bool(BoolFunctionId(0)), params.clone());
        let nil = FunctionValue::new(RuntimeFunctionId::Nil(NilFunctionId(0)), params.clone());
        let tuple = FunctionValue::new(
            RuntimeFunctionId::Tuple {
                id: TupleFunctionId(0),
                return_type: vec![ValueType::Int],
            },
            params.clone(),
        );
        let list = FunctionValue::new(
            RuntimeFunctionId::List {
                id: ListFunctionId(0),
                return_type: Box::new(ValueType::Int),
            },
            params.clone(),
        );
        let function = FunctionValue::new(
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Int),
            },
            params.clone(),
        );

        assert_eq!(int.params(), params);
        assert_eq!(float.params(), params);
        assert_eq!(string.params(), params);
        assert_eq!(bool.params(), params);
        assert_eq!(nil.params(), params);
        assert_eq!(tuple.params(), params);
        assert_eq!(list.params(), params);
        assert_eq!(function.params(), params);
        assert_eq!(
            int.kind(),
            &FunctionValueKind::Int(IntFunctionValue::new(IntFunctionId(0), params)),
        );
    }

    fn int_param(index: usize) -> ParamLocal {
        ParamLocal::int(IntLocalId(index))
    }

    fn string_param(index: usize) -> ParamLocal {
        ParamLocal::string(StringLocalId(index))
    }

    fn float_param(index: usize) -> ParamLocal {
        ParamLocal::float(FloatLocalId(index))
    }

    fn bool_param(index: usize) -> ParamLocal {
        ParamLocal::bool(BoolLocalId(index))
    }

    fn nil_param(index: usize) -> ParamLocal {
        ParamLocal::nil(NilLocalId(index))
    }

    fn tuple_param(index: usize) -> ParamLocal {
        ParamLocal::tuple(TupleLocalId(index), vec![ValueType::Int])
    }

    fn list_param(index: usize) -> ParamLocal {
        ParamLocal::list(ListLocalId(index), ValueType::Int)
    }
}
