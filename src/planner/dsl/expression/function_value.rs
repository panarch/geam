use super::{
    BitArrayFunction, BoolFunction, FloatFunction, Function, FunctionFunction, IntFunction,
    IntoParamLocal, IntoValueType, ListFunction, NilFunction, StringFunction, TupleFunction,
};
use crate::plan::{
    BitArrayFunctionExpr, BitArrayFunctionId, BitArrayFunctionLocalId, BitArrayFunctionReference,
    BoolFunctionExpr, BoolFunctionId, BoolFunctionLocalId, BoolFunctionReference, CaptureArg,
    FloatFunctionExpr, FloatFunctionId, FloatFunctionLocalId, FloatFunctionReference, FunctionExpr,
    FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionLocalId, FunctionFunctionReference,
    FunctionReference, FunctionType, IntFunctionExpr, IntFunctionId, IntFunctionLocalId,
    IntFunctionReference, ListFunctionExpr, ListFunctionId, ListFunctionLocal,
    ListFunctionReference, NilFunctionExpr, NilFunctionId, NilFunctionLocalId,
    NilFunctionReference, ParamLocal, RuntimeFunctionId, StringFunctionExpr, StringFunctionId,
    StringFunctionLocalId, StringFunctionReference, TupleFunctionExpr, TupleFunctionId,
    TupleFunctionLocalId, TupleFunctionReference, ValueType,
};

use ecow::EcoString;

fn function_type(params: &[ParamLocal], return_: ValueType) -> FunctionType {
    FunctionType::new(params.iter().map(ParamLocal::value_type).collect(), return_)
}

pub(crate) fn function_ref(
    runtime_id: RuntimeFunctionId,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> Function {
    Function(FunctionExpr::reference(FunctionReference::new(
        runtime_id,
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn int_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> IntFunction {
    IntFunction(IntFunctionExpr::reference(IntFunctionReference::new(
        IntFunctionId(runtime_id),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn int_function_closure(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    captures: impl IntoIterator<Item = CaptureArg>,
) -> IntFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let type_ = function_type(&params, ValueType::Int);

    IntFunction(IntFunctionExpr::closure(
        IntFunctionId(runtime_id),
        params,
        captures.into_iter().collect(),
        type_,
    ))
}

pub(crate) fn string_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> StringFunction {
    StringFunction(StringFunctionExpr::reference(StringFunctionReference::new(
        StringFunctionId(runtime_id),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn bit_array_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> BitArrayFunction {
    BitArrayFunction(BitArrayFunctionExpr::reference(
        BitArrayFunctionReference::new(
            BitArrayFunctionId(runtime_id),
            params
                .into_iter()
                .map(IntoParamLocal::into_param_local)
                .collect(),
        ),
    ))
}

pub(crate) fn bit_array_function_closure(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    captures: impl IntoIterator<Item = CaptureArg>,
) -> BitArrayFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let type_ = function_type(&params, ValueType::BitArray);

    BitArrayFunction(BitArrayFunctionExpr::closure(
        BitArrayFunctionId(runtime_id),
        params,
        captures.into_iter().collect(),
        type_,
    ))
}

pub(crate) fn float_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> FloatFunction {
    FloatFunction(FloatFunctionExpr::reference(FloatFunctionReference::new(
        FloatFunctionId(runtime_id),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn float_function_closure(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    captures: impl IntoIterator<Item = CaptureArg>,
) -> FloatFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let type_ = function_type(&params, ValueType::Float);

    FloatFunction(FloatFunctionExpr::closure(
        FloatFunctionId(runtime_id),
        params,
        captures.into_iter().collect(),
        type_,
    ))
}

pub(crate) fn bool_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> BoolFunction {
    BoolFunction(BoolFunctionExpr::reference(BoolFunctionReference::new(
        BoolFunctionId(runtime_id),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn nil_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> NilFunction {
    NilFunction(NilFunctionExpr::reference(NilFunctionReference::new(
        NilFunctionId(runtime_id),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn tuple_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    return_type: impl IntoIterator<Item = impl IntoValueType>,
) -> TupleFunction {
    TupleFunction(TupleFunctionExpr::reference(
        TupleFunctionReference::new(
            TupleFunctionId(runtime_id),
            params
                .into_iter()
                .map(IntoParamLocal::into_param_local)
                .collect(),
        ),
        return_type
            .into_iter()
            .map(IntoValueType::into_value_type)
            .collect(),
    ))
}

pub(crate) fn list_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    item_type: impl IntoValueType,
) -> ListFunction {
    let item_type = item_type.into_value_type();
    ListFunction(ListFunctionExpr::reference(ListFunctionReference::new(
        ListFunctionId::from_item_type(runtime_id, item_type),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn tuple_function_closure(
    runtime_id: usize,
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
        TupleFunctionId(runtime_id),
        params,
        captures.into_iter().collect(),
        type_,
        return_type,
    ))
}

pub(crate) fn list_function_closure(
    runtime_id: usize,
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
        ListFunctionId::from_item_type(runtime_id, item_type),
        params,
        captures.into_iter().collect(),
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
    runtime_id: FunctionFunctionId,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    return_type: FunctionType,
) -> FunctionFunction {
    FunctionFunction(FunctionFunctionExpr::reference(
        FunctionFunctionReference::new(
            runtime_id,
            params
                .into_iter()
                .map(IntoParamLocal::into_param_local)
                .collect(),
        ),
        return_type,
    ))
}

pub(crate) fn function_function_closure(
    runtime_id: FunctionFunctionId,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
    captures: impl IntoIterator<Item = CaptureArg>,
    return_type: FunctionType,
) -> FunctionFunction {
    let params = params
        .into_iter()
        .map(IntoParamLocal::into_param_local)
        .collect::<Vec<_>>();
    let type_ = function_type(&params, ValueType::Function(Box::new(return_type.clone())));

    FunctionFunction(FunctionFunctionExpr::closure(
        runtime_id,
        params,
        captures.into_iter().collect(),
        type_,
        return_type,
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
    type_: FunctionType,
) -> FunctionFunction {
    FunctionFunction(FunctionFunctionExpr::local_get(
        FunctionFunctionLocalId(local),
        name.into(),
        type_,
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
        function_function_ref, function_ref, int_function_closure, int_function_ref,
        list_function_closure, list_function_ref, local_bit_array_function, local_bool_function,
        local_float_function, local_function_function, local_int_function, local_list_function,
        local_nil_function, local_string_function, local_tuple_function, nil_function_ref,
        string_function_ref, tuple_function_closure, tuple_function_ref,
    };
    use crate::plan::{
        BitArrayFunctionExpr, BitArrayFunctionId, BitArrayFunctionLocalId,
        BitArrayFunctionReference, BoolFunctionExpr, BoolFunctionId, BoolFunctionLocalId,
        BoolFunctionReference, FloatFunctionExpr, FloatFunctionId, FloatFunctionLocalId,
        FloatFunctionReference, FunctionExpr, FunctionFunctionExpr, FunctionFunctionId,
        FunctionFunctionLocalId, FunctionReference, FunctionType, IntFunctionExpr,
        IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId, IntFunctionReference,
        ListFunctionExpr, ListFunctionId, ListFunctionReference, NilFunctionExpr, NilFunctionId,
        NilFunctionLocalId, NilFunctionReference, ParamLocal, RuntimeFunctionId,
        StringFunctionExpr, StringFunctionId, StringFunctionLocalId, StringFunctionReference,
        TupleFunctionExpr, TupleFunctionId, TupleFunctionLocalId, TupleFunctionReference,
        ValueType,
    };
    use crate::planner::dsl::expression::{Function, float, int};

    #[test]
    fn function_ref_helpers_build_function_values() {
        assert_eq!(
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            )),
            FunctionExpr::reference(FunctionReference::new(
                RuntimeFunctionId::Int(IntFunctionId(0)),
                vec![ParamLocal::int(crate::plan::IntLocalId(0))],
            )),
        );
        assert_eq!(
            string_function_ref(
                1,
                [crate::plan::LocalId::String(crate::plan::StringLocalId(0))]
            )
            .0,
            StringFunctionExpr::reference(StringFunctionReference::new(
                StringFunctionId(1),
                vec![ParamLocal::string(crate::plan::StringLocalId(0))],
            )),
        );
        assert_eq!(
            bit_array_function_ref(
                8,
                [crate::plan::LocalId::BitArray(
                    crate::plan::BitArrayLocalId(0)
                )],
            )
            .0,
            BitArrayFunctionExpr::reference(BitArrayFunctionReference::new(
                BitArrayFunctionId(8),
                vec![ParamLocal::bit_array(crate::plan::BitArrayLocalId(0))],
            )),
        );
        assert_eq!(
            float_function_ref(
                2,
                [crate::plan::LocalId::Float(crate::plan::FloatLocalId(0))]
            )
            .0,
            FloatFunctionExpr::reference(FloatFunctionReference::new(
                FloatFunctionId(2),
                vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
            )),
        );
        assert_eq!(
            bool_function_ref(3, [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))]).0,
            BoolFunctionExpr::reference(BoolFunctionReference::new(
                BoolFunctionId(3),
                vec![ParamLocal::bool(crate::plan::BoolLocalId(0))],
            )),
        );
        assert_eq!(
            nil_function_ref(4, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]).0,
            NilFunctionExpr::reference(NilFunctionReference::new(
                NilFunctionId(4),
                vec![ParamLocal::nil(crate::plan::NilLocalId(0))],
            )),
        );
        assert_eq!(
            tuple_function_ref(
                5,
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                [ValueType::Int, ValueType::String],
            )
            .0,
            TupleFunctionExpr::reference(
                TupleFunctionReference::new(
                    TupleFunctionId(5),
                    vec![ParamLocal::int(crate::plan::IntLocalId(0))],
                ),
                vec![ValueType::Int, ValueType::String],
            ),
        );
        assert_eq!(
            list_function_ref(
                6,
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                ValueType::Int,
            )
            .0,
            ListFunctionExpr::reference(ListFunctionReference::new(
                ListFunctionId::from_item_type(6, crate::plan::ValueType::Int),
                vec![ParamLocal::int(crate::plan::IntLocalId(0))]
            )),
        );
        assert_eq!(
            FunctionExpr::from(Function::from(int_function_ref(
                7,
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            ))),
            FunctionExpr::int(IntFunctionExpr::reference(IntFunctionReference::new(
                IntFunctionId(7),
                vec![ParamLocal::int(crate::plan::IntLocalId(0))],
            ))),
        );
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
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
            )
            .0,
            FunctionFunctionExpr::local_get(
                FunctionFunctionLocalId(0),
                "f".into(),
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
            ),
        );
    }

    #[test]
    fn function_closure_helpers_build_function_values() {
        assert_eq!(
            int_function_closure(0, [ParamLocal::int(crate::plan::IntLocalId(0))], []).0,
            IntFunctionExpr::closure(
                IntFunctionId(0),
                vec![ParamLocal::int(crate::plan::IntLocalId(0))],
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
                BitArrayFunctionId(1),
                vec![ParamLocal::bit_array(crate::plan::BitArrayLocalId(0))],
                Vec::new(),
                FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray),
            ),
        );
        assert_eq!(
            float_function_closure(
                0,
                [ParamLocal::float(crate::plan::FloatLocalId(0))],
                [crate::planner::dsl::expression::capture_float(
                    0,
                    float(1.0)
                )],
            )
            .0,
            FloatFunctionExpr::closure(
                FloatFunctionId(0),
                vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
                vec![crate::planner::dsl::expression::capture_float(
                    0,
                    float(1.0)
                )],
                FunctionType::new(vec![ValueType::Float], ValueType::Float),
            ),
        );
        assert_eq!(
            function_function_closure(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                [crate::planner::dsl::expression::capture_int(0, int(1))],
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            )
            .0,
            FunctionFunctionExpr::closure(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::new(),
                vec![crate::planner::dsl::expression::capture_int(0, int(1))],
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            ),
        );
        assert_eq!(
            tuple_function_closure(
                0,
                [ParamLocal::int(crate::plan::IntLocalId(0))],
                [crate::planner::dsl::expression::capture_int(0, int(1))],
                [ValueType::Int, ValueType::String],
            )
            .0,
            TupleFunctionExpr::closure(
                TupleFunctionId(0),
                vec![ParamLocal::int(crate::plan::IntLocalId(0))],
                vec![crate::planner::dsl::expression::capture_int(0, int(1))],
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
                [crate::planner::dsl::expression::capture_int(0, int(1))],
                ValueType::String,
            )
            .0,
            ListFunctionExpr::closure(
                ListFunctionId::from_item_type(0, ValueType::String),
                vec![ParamLocal::int(crate::plan::IntLocalId(0))],
                vec![crate::planner::dsl::expression::capture_int(0, int(1))],
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
            &FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(returned_function_type)),
            ),
        );
    }
}
