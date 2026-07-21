use super::{
    BitArrayFunction, BoolFunction, FloatFunction, Function, FunctionFunction, IntFunction,
    IntoParamLocal, IntoValueType, ListFunction, NilFunction, StringFunction, TupleFunction,
    UtfCodepointFunction,
};
use crate::plan::{
    BitArrayFunctionExpr, BitArrayFunctionLocalId, BitArrayFunctionReference, BoolFunctionExpr,
    BoolFunctionLocalId, BoolFunctionReference, CaptureArg, FloatFunctionExpr,
    FloatFunctionLocalId, FloatFunctionReference, FunctionExpr, FunctionFunctionExpr,
    FunctionFunctionId, FunctionFunctionLocalId, FunctionFunctionReference, FunctionInstantiation,
    FunctionReference, FunctionShape, FunctionType, GenericFunctionExpr, GenericFunctionReference,
    GenericFunctionType, IntFunctionExpr, IntFunctionLocalId, IntFunctionReference,
    ListFunctionExpr, ListFunctionLocal, ListFunctionReference, NilFunctionExpr,
    NilFunctionLocalId, NilFunctionReference, ParamLocal, RuntimeFunctionId, StringFunctionExpr,
    StringFunctionLocalId, StringFunctionReference, TupleFunctionExpr, TupleFunctionLocalId,
    TupleFunctionReference, TypeParameterId, UtfCodepointFunctionExpr, UtfCodepointFunctionLocalId,
    UtfCodepointFunctionReference, ValueShape, ValueType, monomorphic_function_instantiation,
};

use ecow::EcoString;

fn function_type(params: &[ParamLocal], return_: ValueType) -> FunctionType {
    FunctionType::new(params.iter().map(ParamLocal::value_type).collect(), return_)
}

fn instantiation(
    template: usize,
    params: &[ParamLocal],
    return_: ValueShape,
) -> FunctionInstantiation {
    monomorphic_function_instantiation(
        template,
        FunctionShape::new(
            params.iter().map(ParamLocal::value_shape).collect(),
            return_,
        ),
    )
}

fn function_function_template(id: &FunctionFunctionId) -> usize {
    match id {
        FunctionFunctionId::Int(id) => id.0,
        FunctionFunctionId::Float(id) => id.0,
        FunctionFunctionId::String(id) => id.0,
        FunctionFunctionId::BitArray(id) => id.0,
        FunctionFunctionId::UtfCodepoint(id) => id.0,
        FunctionFunctionId::Custom(id) => id.index(),
        FunctionFunctionId::Bool(id) => id.0,
        FunctionFunctionId::Nil(id) => id.0,
        FunctionFunctionId::Tuple(id) => id.0,
        FunctionFunctionId::List(id) => match id {
            crate::plan::ListFunctionFunctionId::Generic { id, .. } => id.0,
            crate::plan::ListFunctionFunctionId::Int { id, .. } => id.0,
            crate::plan::ListFunctionFunctionId::String { id, .. } => id.0,
            crate::plan::ListFunctionFunctionId::BitArray { id, .. } => id.0,
            crate::plan::ListFunctionFunctionId::UtfCodepoint { id, .. } => id.0,
            crate::plan::ListFunctionFunctionId::Custom { id, .. } => id.0,
            crate::plan::ListFunctionFunctionId::Float { id, .. } => id.0,
            crate::plan::ListFunctionFunctionId::Bool { id, .. } => id.0,
            crate::plan::ListFunctionFunctionId::Nil { id, .. } => id.0,
            crate::plan::ListFunctionFunctionId::Tuple { id, .. } => id.0,
            crate::plan::ListFunctionFunctionId::List { id, .. } => id.0,
            crate::plan::ListFunctionFunctionId::Function { id, .. } => id.0,
        },
        FunctionFunctionId::Function(id) => id.index(),
    }
}

fn runtime_function_template(id: &RuntimeFunctionId) -> (usize, ValueShape) {
    match id {
        RuntimeFunctionId::Int(id) => (id.0, ValueShape::Int),
        RuntimeFunctionId::Float(id) => (id.0, ValueShape::Float),
        RuntimeFunctionId::String(id) => (id.0, ValueShape::String),
        RuntimeFunctionId::BitArray(id) => (id.0, ValueShape::BitArray),
        RuntimeFunctionId::UtfCodepoint(id) => (id.0, ValueShape::UtfCodepoint),
        RuntimeFunctionId::Custom(id) => {
            (id.index(), ValueShape::Custom(id.return_shape().clone()))
        }
        RuntimeFunctionId::Bool(id) => (id.0, ValueShape::Bool),
        RuntimeFunctionId::Nil(id) => (id.0, ValueShape::Nil),
        RuntimeFunctionId::Tuple { id, return_type } => (
            id.0,
            ValueShape::Tuple(
                return_type
                    .iter()
                    .cloned()
                    .map(ValueShape::from_value_type)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
        ),
        RuntimeFunctionId::List(id) => (
            match id {
                crate::plan::ListFunctionId::Generic { id, .. } => id.0,
                crate::plan::ListFunctionId::Int(id) => id.0,
                crate::plan::ListFunctionId::String(id) => id.0,
                crate::plan::ListFunctionId::BitArray(id) => id.0,
                crate::plan::ListFunctionId::UtfCodepoint(id) => id.0,
                crate::plan::ListFunctionId::Custom { id, .. } => id.0,
                crate::plan::ListFunctionId::Float(id) => id.0,
                crate::plan::ListFunctionId::Bool(id) => id.0,
                crate::plan::ListFunctionId::Nil(id) => id.0,
                crate::plan::ListFunctionId::Tuple { id, .. } => id.0,
                crate::plan::ListFunctionId::List { id, .. } => id.0,
                crate::plan::ListFunctionId::Function { id, .. } => id.0,
            },
            ValueShape::List(Box::new(ValueShape::from_value_type(id.item_type()))),
        ),
        RuntimeFunctionId::Function { id, return_type } => (
            function_function_template(id),
            ValueShape::Function(Box::new(FunctionShape::from_function_type(
                return_type.clone(),
            ))),
        ),
    }
}

pub(crate) fn function_ref(
    target: RuntimeFunctionId,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> Function {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let (template, return_) = runtime_function_template(&target);
    Function(FunctionExpr::reference(FunctionReference::new(
        instantiation(template, &params, return_),
    )))
}

pub(crate) fn generic_function_ref(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    return_parameter: TypeParameterId,
) -> Function {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let type_ = GenericFunctionType::new(
        params.iter().map(ParamLocal::value_shape).collect(),
        return_parameter,
    );

    Function(FunctionExpr::generic(GenericFunctionExpr::reference(
        GenericFunctionReference::new(instantiation(
            template,
            &params,
            ValueShape::Parameter(return_parameter),
        )),
        type_,
    )))
}

pub(crate) fn int_function_ref(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> IntFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    IntFunction(IntFunctionExpr::reference(IntFunctionReference::new(
        instantiation(template, &params, ValueShape::Int),
    )))
}

pub(crate) fn int_function_closure(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    captures: impl IntoIterator<Item = CaptureArg>,
) -> IntFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let type_ = function_type(&params, ValueType::Int);

    IntFunction(IntFunctionExpr::closure(
        instantiation(template, &params, ValueShape::Int),
        captures.into_iter().collect(),
        type_,
    ))
}

pub(crate) fn string_function_ref(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> StringFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    StringFunction(StringFunctionExpr::reference(StringFunctionReference::new(
        instantiation(template, &params, ValueShape::String),
    )))
}

pub(crate) fn bit_array_function_ref(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> BitArrayFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    BitArrayFunction(BitArrayFunctionExpr::reference(
        BitArrayFunctionReference::new(instantiation(template, &params, ValueShape::BitArray)),
    ))
}

pub(crate) fn bit_array_function_closure(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    captures: impl IntoIterator<Item = CaptureArg>,
) -> BitArrayFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let type_ = function_type(&params, ValueType::BitArray);

    BitArrayFunction(BitArrayFunctionExpr::closure(
        instantiation(template, &params, ValueShape::BitArray),
        captures.into_iter().collect(),
        type_,
    ))
}

pub(crate) fn utf_codepoint_function_ref(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> UtfCodepointFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    UtfCodepointFunction(UtfCodepointFunctionExpr::reference(
        UtfCodepointFunctionReference::new(instantiation(
            template,
            &params,
            ValueShape::UtfCodepoint,
        )),
    ))
}

pub(crate) fn utf_codepoint_function_closure(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    captures: impl IntoIterator<Item = CaptureArg>,
) -> UtfCodepointFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let type_ = function_type(&params, ValueType::UtfCodepoint);

    UtfCodepointFunction(UtfCodepointFunctionExpr::closure(
        instantiation(template, &params, ValueShape::UtfCodepoint),
        captures.into_iter().collect(),
        type_,
    ))
}

pub(crate) fn float_function_ref(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> FloatFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    FloatFunction(FloatFunctionExpr::reference(FloatFunctionReference::new(
        instantiation(template, &params, ValueShape::Float),
    )))
}

pub(crate) fn float_function_closure(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    captures: impl IntoIterator<Item = CaptureArg>,
) -> FloatFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let type_ = function_type(&params, ValueType::Float);

    FloatFunction(FloatFunctionExpr::closure(
        instantiation(template, &params, ValueShape::Float),
        captures.into_iter().collect(),
        type_,
    ))
}

pub(crate) fn bool_function_ref(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> BoolFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    BoolFunction(BoolFunctionExpr::reference(BoolFunctionReference::new(
        instantiation(template, &params, ValueShape::Bool),
    )))
}

pub(crate) fn nil_function_ref(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> NilFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    NilFunction(NilFunctionExpr::reference(NilFunctionReference::new(
        instantiation(template, &params, ValueShape::Nil),
    )))
}

pub(crate) fn tuple_function_ref(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    return_type: impl IntoIterator<Item = impl IntoValueType>,
) -> TupleFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let return_type = return_type
        .into_iter()
        .map(IntoValueType::into_value_type)
        .collect::<Vec<_>>();
    TupleFunction(TupleFunctionExpr::reference(TupleFunctionReference::new(
        instantiation(
            template,
            &params,
            ValueShape::from_value_type(ValueType::Tuple(return_type)),
        ),
    )))
}

pub(crate) fn list_function_ref(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    item_type: impl IntoValueType,
) -> ListFunction {
    let item_type = item_type.into_value_type();
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    ListFunction(ListFunctionExpr::reference(
        ListFunctionReference::new(instantiation(
            template,
            &params,
            ValueShape::List(Box::new(ValueShape::from_value_type(item_type.clone()))),
        )),
        item_type,
    ))
}

pub(crate) fn tuple_function_closure(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    captures: impl IntoIterator<Item = CaptureArg>,
    return_type: impl IntoIterator<Item = impl IntoValueType>,
) -> TupleFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let return_type = return_type
        .into_iter()
        .map(IntoValueType::into_value_type)
        .collect::<Vec<_>>();
    let type_ = function_type(&params, ValueType::Tuple(return_type.clone()));

    TupleFunction(TupleFunctionExpr::closure(
        instantiation(
            template,
            &params,
            ValueShape::from_value_type(ValueType::Tuple(return_type.clone())),
        ),
        captures.into_iter().collect(),
        type_,
        return_type,
    ))
}

pub(crate) fn list_function_closure(
    template: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    captures: impl IntoIterator<Item = CaptureArg>,
    item_type: impl IntoValueType,
) -> ListFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let item_type = item_type.into_value_type();

    ListFunction(ListFunctionExpr::closure(
        instantiation(
            template,
            &params,
            ValueShape::List(Box::new(ValueShape::from_value_type(item_type.clone()))),
        ),
        captures.into_iter().collect(),
        item_type,
    ))
}

pub(crate) fn local_int_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> IntFunction {
    IntFunction(IntFunctionExpr::local_get(
        IntFunctionLocalId(local),
        name.into(),
        int_function_type(params),
    ))
}

pub(crate) fn function_function_ref(
    template: FunctionFunctionId,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    return_type: FunctionType,
) -> FunctionFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let function = instantiation(
        function_function_template(&template),
        &params,
        ValueShape::Function(Box::new(FunctionShape::from_function_type(
            return_type.clone(),
        ))),
    );
    FunctionFunction(FunctionFunctionExpr::reference(
        FunctionFunctionReference::new(function),
        return_type,
    ))
}

pub(crate) fn function_function_closure(
    template: FunctionFunctionId,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    captures: impl IntoIterator<Item = CaptureArg>,
    return_type: FunctionType,
) -> FunctionFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let type_ = crate::plan::FunctionFunctionType::new(
        params.iter().map(ParamLocal::value_type).collect(),
        return_type.clone(),
    );

    FunctionFunction(FunctionFunctionExpr::closure(
        instantiation(
            function_function_template(&template),
            &params,
            ValueShape::Function(Box::new(FunctionShape::from_function_type(return_type))),
        ),
        captures.into_iter().collect(),
        type_,
    ))
}

pub(crate) fn local_string_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> StringFunction {
    StringFunction(StringFunctionExpr::local_get(
        StringFunctionLocalId(local),
        name.into(),
        string_function_type(params),
    ))
}

pub(crate) fn local_bit_array_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> BitArrayFunction {
    BitArrayFunction(BitArrayFunctionExpr::local_get(
        BitArrayFunctionLocalId(local),
        name.into(),
        bit_array_function_type(params),
    ))
}

pub(crate) fn local_utf_codepoint_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> UtfCodepointFunction {
    UtfCodepointFunction(UtfCodepointFunctionExpr::local_get(
        UtfCodepointFunctionLocalId(local),
        name.into(),
        utf_codepoint_function_type(params),
    ))
}

pub(crate) fn local_float_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> FloatFunction {
    FloatFunction(FloatFunctionExpr::local_get(
        FloatFunctionLocalId(local),
        name.into(),
        float_function_type(params),
    ))
}

pub(crate) fn local_bool_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> BoolFunction {
    BoolFunction(BoolFunctionExpr::local_get(
        BoolFunctionLocalId(local),
        name.into(),
        bool_function_type(params),
    ))
}

pub(crate) fn local_nil_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> NilFunction {
    NilFunction(NilFunctionExpr::local_get(
        NilFunctionLocalId(local),
        name.into(),
        nil_function_type(params),
    ))
}

pub(crate) fn local_tuple_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
    return_type: impl IntoIterator<Item = impl IntoValueType>,
) -> TupleFunction {
    TupleFunction(TupleFunctionExpr::local_get(
        TupleFunctionLocalId(local),
        name.into(),
        dsl_function_type(
            params,
            ValueType::Tuple(
                return_type
                    .into_iter()
                    .map(IntoValueType::into_value_type)
                    .collect(),
            ),
        ),
    ))
}

pub(crate) fn local_list_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
    return_type: impl IntoValueType,
) -> ListFunction {
    let params = params
        .into_iter()
        .map(IntoValueType::into_value_type)
        .collect::<Vec<_>>();
    let item_type = return_type.into_value_type();
    let return_type = ValueType::List(Box::new(item_type.clone()));
    ListFunction(ListFunctionExpr::local_get(
        ListFunctionLocal::from_item_type(local, FunctionType::new(params, return_type), item_type),
        name.into(),
    ))
}

pub(crate) fn local_function_function(
    local: usize,
    name: impl Into<EcoString>,
    type_: crate::plan::FunctionFunctionType,
) -> FunctionFunction {
    FunctionFunction(FunctionFunctionExpr::local_get(
        crate::plan::FunctionFunctionLocal::new(FunctionFunctionLocalId(local), type_),
        name.into(),
    ))
}

fn int_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    dsl_function_type(params, ValueType::Int)
}

fn string_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    dsl_function_type(params, ValueType::String)
}

fn bit_array_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    dsl_function_type(params, ValueType::BitArray)
}

fn utf_codepoint_function_type(
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> FunctionType {
    dsl_function_type(params, ValueType::UtfCodepoint)
}

fn float_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    dsl_function_type(params, ValueType::Float)
}

fn bool_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    dsl_function_type(params, ValueType::Bool)
}

fn nil_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    dsl_function_type(params, ValueType::Nil)
}

fn dsl_function_type(
    params: impl IntoIterator<Item = impl IntoValueType>,
    return_: ValueType,
) -> FunctionType {
    FunctionType::new(
        params
            .into_iter()
            .map(IntoValueType::into_value_type)
            .collect(),
        return_,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        bit_array_function_closure, bit_array_function_ref, bool_function_ref,
        float_function_closure, float_function_ref, function_function_closure,
        function_function_ref, function_ref, generic_function_ref, instantiation,
        int_function_closure, int_function_ref, list_function_closure, list_function_ref,
        local_bit_array_function, local_bool_function, local_float_function,
        local_function_function, local_int_function, local_list_function, local_nil_function,
        local_string_function, local_tuple_function, local_utf_codepoint_function,
        nil_function_ref, string_function_ref, tuple_function_closure, tuple_function_ref,
        utf_codepoint_function_closure, utf_codepoint_function_ref,
    };
    use crate::plan::{
        BitArrayFunctionExpr, BitArrayFunctionLocalId, BitArrayFunctionReference, BoolFunctionExpr,
        BoolFunctionLocalId, BoolFunctionReference, CustomType, CustomTypeName, FloatFunctionExpr,
        FloatFunctionLocalId, FloatFunctionReference, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionId, FunctionFunctionLocalId, FunctionFunctionReference, FunctionReference,
        FunctionShape, FunctionType, GenericFunctionExpr, GenericFunctionReference,
        GenericFunctionType, IntFunctionExpr, IntFunctionFunctionId, IntFunctionLocalId,
        IntFunctionReference, ListFunctionExpr, ListFunctionFunctionId, ListFunctionId,
        ListFunctionReference, NilFunctionExpr, NilFunctionLocalId, NilFunctionReference,
        ParamLocal, RuntimeFunctionId, StringFunctionExpr, StringFunctionLocalId,
        StringFunctionReference, TupleFunctionExpr, TupleFunctionLocalId, TupleFunctionReference,
        TypeParameterId, UtfCodepointFunctionExpr, UtfCodepointFunctionLocalId,
        UtfCodepointFunctionReference, ValueShape, ValueType,
    };
    use crate::planner::dsl::expression::Function;

    #[test]
    fn function_ref_helpers_build_function_values() {
        assert_eq!(
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            )),
            FunctionExpr::reference(FunctionReference::new(instantiation(
                0,
                &[ParamLocal::int(crate::plan::IntLocalId(0))],
                ValueShape::Int,
            ))),
        );
        assert_eq!(
            string_function_ref(
                1,
                [crate::plan::LocalId::String(crate::plan::StringLocalId(0))]
            )
            .0,
            StringFunctionExpr::reference(StringFunctionReference::new(instantiation(
                1,
                &[ParamLocal::string(crate::plan::StringLocalId(0))],
                ValueShape::String,
            ))),
        );
        assert_eq!(
            bit_array_function_ref(
                8,
                [crate::plan::LocalId::BitArray(
                    crate::plan::BitArrayLocalId(0)
                )],
            )
            .0,
            BitArrayFunctionExpr::reference(BitArrayFunctionReference::new(instantiation(
                8,
                &[ParamLocal::bit_array(crate::plan::BitArrayLocalId(0))],
                ValueShape::BitArray,
            ))),
        );
        assert_eq!(
            utf_codepoint_function_ref(
                9,
                [crate::plan::LocalId::UtfCodepoint(
                    crate::plan::UtfCodepointLocalId(0),
                )],
            )
            .0,
            UtfCodepointFunctionExpr::reference(UtfCodepointFunctionReference::new(instantiation(
                9,
                &[ParamLocal::utf_codepoint(crate::plan::UtfCodepointLocalId(
                    0
                ),)],
                ValueShape::UtfCodepoint,
            ))),
        );
        assert_eq!(
            float_function_ref(
                2,
                [crate::plan::LocalId::Float(crate::plan::FloatLocalId(0))]
            )
            .0,
            FloatFunctionExpr::reference(FloatFunctionReference::new(instantiation(
                2,
                &[ParamLocal::float(crate::plan::FloatLocalId(0))],
                ValueShape::Float,
            ))),
        );
        assert_eq!(
            bool_function_ref(3, [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))]).0,
            BoolFunctionExpr::reference(BoolFunctionReference::new(instantiation(
                3,
                &[ParamLocal::bool(crate::plan::BoolLocalId(0))],
                ValueShape::Bool,
            ))),
        );
        assert_eq!(
            nil_function_ref(4, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]).0,
            NilFunctionExpr::reference(NilFunctionReference::new(instantiation(
                4,
                &[ParamLocal::nil(crate::plan::NilLocalId(0))],
                ValueShape::Nil,
            ))),
        );
        assert_eq!(
            tuple_function_ref(
                5,
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                [ValueType::Int, ValueType::String],
            )
            .0,
            TupleFunctionExpr::reference(TupleFunctionReference::new(instantiation(
                5,
                &[ParamLocal::int(crate::plan::IntLocalId(0))],
                ValueShape::Tuple(vec![ValueShape::Int, ValueShape::String].into_boxed_slice(),),
            )),),
        );
        assert_eq!(
            list_function_ref(
                6,
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                ValueType::Int,
            )
            .0,
            ListFunctionExpr::reference(
                ListFunctionReference::new(instantiation(
                    6,
                    &[ParamLocal::int(crate::plan::IntLocalId(0))],
                    ValueShape::List(Box::new(ValueShape::Int)),
                )),
                ValueType::Int,
            ),
        );
        assert_eq!(
            FunctionExpr::from(Function::from(int_function_ref(
                7,
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            ))),
            FunctionExpr::int(IntFunctionExpr::reference(IntFunctionReference::new(
                instantiation(
                    7,
                    &[ParamLocal::int(crate::plan::IntLocalId(0))],
                    ValueShape::Int,
                )
            ))),
        );

        let parameter = TypeParameterId(0);
        assert_eq!(
            FunctionExpr::from(generic_function_ref(
                10,
                [ParamLocal::generic(crate::plan::GenericLocal::new(
                    crate::plan::GenericLocalId(0),
                    parameter,
                ))],
                parameter,
            )),
            FunctionExpr::generic(GenericFunctionExpr::reference(
                GenericFunctionReference::new(instantiation(
                    10,
                    &[ParamLocal::generic(crate::plan::GenericLocal::new(
                        crate::plan::GenericLocalId(0),
                        parameter,
                    ))],
                    ValueShape::Parameter(parameter),
                )),
                GenericFunctionType::new(vec![ValueShape::Parameter(parameter)], parameter),
            )),
        );
    }

    #[test]
    fn list_function_refs_preserve_every_item_family_template() {
        let custom = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let item_types = vec![
            ValueType::Parameter(TypeParameterId(0)),
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        ];

        for (template, item_type) in item_types.into_iter().enumerate() {
            assert_eq!(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::List(ListFunctionId::from_item_type(
                        template,
                        item_type.clone(),
                    )),
                    Vec::<ParamLocal>::new(),
                )),
                FunctionExpr::reference(FunctionReference::new(instantiation(
                    template,
                    &[],
                    ValueShape::List(Box::new(ValueShape::from_value_type(item_type))),
                ))),
            );
        }
    }

    #[test]
    fn function_returning_list_refs_preserve_every_item_family_template() {
        let custom = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let item_types = vec![
            ValueType::Parameter(TypeParameterId(0)),
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        ];

        for (template, item_type) in item_types.into_iter().enumerate() {
            let returned_type =
                FunctionType::new(Vec::new(), ValueType::List(Box::new(item_type.clone())));
            assert_eq!(
                function_function_ref(
                    FunctionFunctionId::List(ListFunctionFunctionId::from_item_type(
                        template,
                        returned_type.clone(),
                        item_type,
                    )),
                    Vec::<ParamLocal>::new(),
                    returned_type.clone(),
                )
                .0,
                FunctionFunctionExpr::reference(
                    FunctionFunctionReference::new(instantiation(
                        template,
                        &[],
                        ValueShape::Function(Box::new(FunctionShape::from_function_type(
                            returned_type.clone(),
                        ))),
                    )),
                    returned_type,
                ),
            );
        }
    }

    #[test]
    fn function_local_helpers_build_local_get_shapes() {
        assert_eq!(
            local_int_function(
                0,
                "f",
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]
            )
            .0,
            IntFunctionExpr::local_get(
                IntFunctionLocalId(0),
                "f".into(),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            ),
        );
        assert_eq!(
            local_string_function(
                0,
                "f",
                [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
            )
            .0,
            StringFunctionExpr::local_get(
                StringFunctionLocalId(0),
                "f".into(),
                FunctionType::new(vec![ValueType::String], ValueType::String),
            ),
        );
        assert_eq!(
            local_bit_array_function(
                1,
                "bits",
                [crate::plan::LocalId::BitArray(
                    crate::plan::BitArrayLocalId(0)
                )],
            )
            .0,
            BitArrayFunctionExpr::local_get(
                BitArrayFunctionLocalId(1),
                "bits".into(),
                FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray),
            ),
        );
        assert_eq!(
            local_utf_codepoint_function(
                2,
                "codepoint",
                [crate::plan::LocalId::UtfCodepoint(
                    crate::plan::UtfCodepointLocalId(0),
                )],
            )
            .0,
            UtfCodepointFunctionExpr::local_get(
                UtfCodepointFunctionLocalId(2),
                "codepoint".into(),
                FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint,),
            ),
        );
        assert_eq!(
            local_float_function(
                0,
                "f",
                [crate::plan::LocalId::Float(crate::plan::FloatLocalId(0))],
            )
            .0,
            FloatFunctionExpr::local_get(
                FloatFunctionLocalId(0),
                "f".into(),
                FunctionType::new(vec![ValueType::Float], ValueType::Float),
            ),
        );
        assert_eq!(
            local_bool_function(
                0,
                "f",
                [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))],
            )
            .0,
            BoolFunctionExpr::local_get(
                BoolFunctionLocalId(0),
                "f".into(),
                FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
            ),
        );
        assert_eq!(
            local_nil_function(
                0,
                "f",
                [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))],
            )
            .0,
            NilFunctionExpr::local_get(
                NilFunctionLocalId(0),
                "f".into(),
                FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
            ),
        );
        assert_eq!(
            local_tuple_function(
                0,
                "f",
                [ValueType::Int],
                [ValueType::Int, ValueType::String]
            )
            .0,
            TupleFunctionExpr::local_get(
                TupleFunctionLocalId(0),
                "f".into(),
                FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
                ),
            ),
        );
        assert_eq!(
            local_list_function(0, "f", [ValueType::Int], ValueType::String).0,
            ListFunctionExpr::local_get(
                crate::plan::ListFunctionLocal::from_item_type(
                    0,
                    FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::List(Box::new(ValueType::String)),
                    ),
                    ValueType::String,
                ),
                "f".into(),
            ),
        );
        assert_eq!(
            local_function_function(
                0,
                "f",
                crate::plan::FunctionFunctionType::new(
                    Vec::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
            )
            .0,
            FunctionFunctionExpr::local_get(
                crate::plan::FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(0),
                    crate::plan::FunctionFunctionType::new(
                        Vec::new(),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    ),
                ),
                "f".into(),
            ),
        );
    }

    #[test]
    fn function_closure_helpers_build_function_values() {
        assert_eq!(
            int_function_closure(0, [ParamLocal::int(crate::plan::IntLocalId(0))], []).0,
            IntFunctionExpr::closure(
                instantiation(
                    0,
                    &[ParamLocal::int(crate::plan::IntLocalId(0))],
                    ValueShape::Int,
                ),
                Vec::new(),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            ),
        );
        assert_eq!(
            bit_array_function_closure(
                1,
                [ParamLocal::bit_array(crate::plan::BitArrayLocalId(0))],
                [],
            )
            .0,
            BitArrayFunctionExpr::closure(
                instantiation(
                    1,
                    &[ParamLocal::bit_array(crate::plan::BitArrayLocalId(0))],
                    ValueShape::BitArray,
                ),
                Vec::new(),
                FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray),
            ),
        );
        assert_eq!(
            utf_codepoint_function_closure(
                2,
                [ParamLocal::utf_codepoint(crate::plan::UtfCodepointLocalId(
                    0
                ),)],
                [],
            )
            .0,
            UtfCodepointFunctionExpr::closure(
                instantiation(
                    2,
                    &[ParamLocal::utf_codepoint(crate::plan::UtfCodepointLocalId(
                        0
                    ),)],
                    ValueShape::UtfCodepoint,
                ),
                Vec::new(),
                FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint,),
            ),
        );
        assert_eq!(
            float_function_closure(
                0,
                [ParamLocal::float(crate::plan::FloatLocalId(0))],
                [crate::planner::dsl::expression::capture_float(1)],
            )
            .0,
            FloatFunctionExpr::closure(
                instantiation(
                    0,
                    &[ParamLocal::float(crate::plan::FloatLocalId(0))],
                    ValueShape::Float,
                ),
                vec![crate::planner::dsl::expression::capture_float(1)],
                FunctionType::new(vec![ValueType::Float], ValueType::Float),
            ),
        );
        assert_eq!(
            function_function_closure(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                [crate::planner::dsl::expression::capture_int(1)],
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            )
            .0,
            FunctionFunctionExpr::closure(
                instantiation(
                    0,
                    &[],
                    ValueShape::Function(Box::new(FunctionShape::from_function_type(
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    ))),
                ),
                vec![crate::planner::dsl::expression::capture_int(1)],
                crate::plan::FunctionFunctionType::new(
                    Vec::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
            ),
        );
        assert_eq!(
            tuple_function_closure(
                0,
                [ParamLocal::int(crate::plan::IntLocalId(0))],
                [crate::planner::dsl::expression::capture_int(1)],
                [ValueType::Int, ValueType::String],
            )
            .0,
            TupleFunctionExpr::closure(
                instantiation(
                    0,
                    &[ParamLocal::int(crate::plan::IntLocalId(0))],
                    ValueShape::Tuple(vec![ValueShape::Int, ValueShape::String].into_boxed_slice(),),
                ),
                vec![crate::planner::dsl::expression::capture_int(1)],
                FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
                ),
                vec![ValueType::Int, ValueType::String],
            ),
        );
        assert_eq!(
            list_function_closure(
                0,
                [ParamLocal::int(crate::plan::IntLocalId(0))],
                [crate::planner::dsl::expression::capture_int(1)],
                ValueType::String,
            )
            .0,
            ListFunctionExpr::closure(
                instantiation(
                    0,
                    &[ParamLocal::int(crate::plan::IntLocalId(0))],
                    ValueShape::List(Box::new(ValueShape::String)),
                ),
                vec![crate::planner::dsl::expression::capture_int(1)],
                ValueType::String,
            ),
        );
    }

    #[test]
    fn function_returning_function_ref_preserves_return_type() {
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);

        assert_eq!(
            FunctionExpr::from(function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                returned_function_type.clone(),
            ))
            .into_function()
            .expect("function-returning-function expression")
            .type_(),
            FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(returned_function_type)),
            ),
        );
    }
}
