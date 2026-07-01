use super::{
    BoolFunction, Function, FunctionFunction, IntFunction, IntoParamLocal, IntoValueType,
    NilFunction, StringFunction,
};
use crate::plan::{
    BoolFunctionExpr, BoolFunctionId, BoolFunctionLocalId, BoolFunctionValue, CaptureArg,
    FunctionExpr, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionLocalId,
    FunctionFunctionValue, FunctionType, FunctionValue, IntFunctionExpr, IntFunctionId,
    IntFunctionLocalId, IntFunctionValue, NilFunctionExpr, NilFunctionId, NilFunctionLocalId,
    NilFunctionValue, RuntimeFunctionId, StringFunctionExpr, StringFunctionId,
    StringFunctionLocalId, StringFunctionValue, ValueType,
};
use ecow::EcoString;

pub(crate) fn function_ref(
    runtime_id: RuntimeFunctionId,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> Function {
    Function(FunctionExpr::value(FunctionValue::new(
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
    IntFunction(IntFunctionExpr::value(IntFunctionValue::new(
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
    let type_ = FunctionType::from_params(&params, ValueType::Int);

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
    StringFunction(StringFunctionExpr::value(StringFunctionValue::new(
        StringFunctionId(runtime_id),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn bool_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> BoolFunction {
    BoolFunction(BoolFunctionExpr::value(BoolFunctionValue::new(
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
    NilFunction(NilFunctionExpr::value(NilFunctionValue::new(
        NilFunctionId(runtime_id),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
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
    FunctionFunction(FunctionFunctionExpr::value(FunctionFunctionValue::new(
        runtime_id,
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
        return_type,
    )))
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
    let type_ =
        FunctionType::from_params(&params, ValueType::Function(Box::new(return_type.clone())));

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
    function_type(params, ValueType::Int)
}

fn string_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    function_type(params, ValueType::String)
}

fn bool_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    function_type(params, ValueType::Bool)
}

fn nil_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    function_type(params, ValueType::Nil)
}

fn function_type(
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
        bool_function_ref, function_function_closure, function_function_ref, function_ref,
        int_function_closure, int_function_ref, local_bool_function, local_function_function,
        local_int_function, local_nil_function, local_string_function, nil_function_ref,
        string_function_ref,
    };
    use crate::plan::{
        BoolFunctionExprKind, Expr, ExprKind, FunctionExpr, FunctionExprKind, FunctionFunctionId,
        FunctionType, IntFunctionExprKind, IntFunctionFunctionId, NilFunctionExprKind, ParamLocal,
        RuntimeFunctionId, StringFunctionExprKind, ValueType,
    };
    use crate::planner::dsl::expression::{Function, int};

    #[test]
    fn function_ref_helpers_build_function_values() {
        assert!(matches!(
            Expr::from(function_ref(
                RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            ))
            .kind(),
            ExprKind::Function(_),
        ));
        assert!(matches!(
            Expr::from(string_function_ref(
                0,
                [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
            ))
            .kind(),
            ExprKind::Function(_),
        ));
        assert!(matches!(
            Expr::from(bool_function_ref(
                0,
                [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))],
            ))
            .kind(),
            ExprKind::Function(_),
        ));
        assert!(matches!(
            Expr::from(nil_function_ref(
                0,
                [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))],
            ))
            .kind(),
            ExprKind::Function(_),
        ));
        assert!(matches!(
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            ))
            .kind(),
            FunctionExprKind::Int(_),
        ));
        assert!(matches!(
            FunctionExpr::from(Function::from(int_function_ref(
                0,
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            )))
            .kind(),
            FunctionExprKind::Int(_),
        ));
    }

    #[test]
    fn function_local_helpers_build_local_get_shapes() {
        assert!(matches!(
            local_int_function(
                0,
                "f",
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]
            )
            .0
            .kind(),
            IntFunctionExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_string_function(
                0,
                "f",
                [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
            )
            .0
            .kind(),
            StringFunctionExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_bool_function(
                0,
                "f",
                [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))],
            )
            .0
            .kind(),
            BoolFunctionExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_nil_function(
                0,
                "f",
                [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))],
            )
            .0
            .kind(),
            NilFunctionExprKind::LocalGet { .. },
        ));
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
            .0
            .type_(),
            &FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
            ),
        );
    }

    #[test]
    fn function_closure_helpers_build_function_values() {
        assert_eq!(
            int_function_closure(0, [ParamLocal::int(crate::plan::IntLocalId(0))], [])
                .0
                .type_(),
            &FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        assert_eq!(
            function_function_closure(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                [crate::planner::dsl::expression::capture_int(0, int(1))],
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            )
            .0
            .type_(),
            &FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
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
