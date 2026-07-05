use super::{
    Bool, Float, Function, FunctionFunction, Int, IntFunction, List, ListFunction, Nil, String,
    TupleFunction,
};
use crate::plan::{
    BoolExpr, BoolFunctionExpr, FloatExpr, FloatFunctionExpr, FunctionExpr, FunctionExprKind,
    FunctionFunctionExpr, IntExpr, IntFunctionExpr, ListExpr, ListFunctionExpr, NilExpr,
    NilFunctionExpr, Step, StringExpr, StringFunctionExpr, TupleFunctionExpr,
};

pub(crate) fn block_int(steps: impl IntoIterator<Item = Step>, return_: Int) -> Int {
    Int(IntExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_string(steps: impl IntoIterator<Item = Step>, return_: String) -> String {
    String(StringExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_float(steps: impl IntoIterator<Item = Step>, return_: Float) -> Float {
    Float(FloatExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_bool(steps: impl IntoIterator<Item = Step>, return_: Bool) -> Bool {
    Bool(BoolExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_nil(steps: impl IntoIterator<Item = Step>, return_: Nil) -> Nil {
    Nil(NilExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_list(steps: impl IntoIterator<Item = Step>, return_: List) -> List {
    List(ListExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_function(steps: Vec<Step>, return_: Function) -> Function {
    Function(match FunctionExpr::from(return_).into_kind() {
        FunctionExprKind::Int(return_) => FunctionExpr::int(IntFunctionExpr::block(steps, return_)),
        FunctionExprKind::String(return_) => {
            FunctionExpr::string(StringFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Float(return_) => {
            FunctionExpr::float(FloatFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Bool(return_) => {
            FunctionExpr::bool(BoolFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Nil(return_) => FunctionExpr::nil(NilFunctionExpr::block(steps, return_)),
        FunctionExprKind::Tuple(return_) => {
            FunctionExpr::tuple(TupleFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::List(return_) => {
            FunctionExpr::list(ListFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Function(return_) => {
            FunctionExpr::function(FunctionFunctionExpr::block(steps, return_))
        }
    })
}

pub(crate) fn block_tuple_function(
    steps: impl IntoIterator<Item = Step>,
    return_: TupleFunction,
) -> TupleFunction {
    TupleFunction(TupleFunctionExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_list_function(
    steps: impl IntoIterator<Item = Step>,
    return_: ListFunction,
) -> ListFunction {
    ListFunction(ListFunctionExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_function_function(
    steps: impl IntoIterator<Item = Step>,
    return_: FunctionFunction,
) -> FunctionFunction {
    FunctionFunction(FunctionFunctionExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_int_function(
    steps: impl IntoIterator<Item = Step>,
    return_: IntFunction,
) -> IntFunction {
    IntFunction(IntFunctionExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        block_bool, block_float, block_function, block_function_function, block_int,
        block_int_function, block_list, block_list_function, block_nil, block_string,
        block_tuple_function,
    };
    use crate::plan::{
        BoolExpr, BoolFunctionId, BoolFunctionValue, FloatExpr, FloatFunctionId,
        FloatFunctionValue, FunctionExpr, FunctionFunctionExpr, FunctionFunctionId,
        FunctionFunctionValue, FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId,
        IntFunctionId, IntFunctionValue, ListExpr, ListFunctionExpr, ListFunctionValue, NilExpr,
        NilFunctionId, NilFunctionValue, ParamLocal, RuntimeFunctionId, StringExpr,
        StringFunctionId, StringFunctionValue, TupleFunctionExpr, TupleFunctionValue, ValueType,
    };
    use crate::planner::dsl::expression::{
        Function, bool_, float, function_function_ref, function_ref, int, int_function_ref,
        let_bool_step, let_int_step, let_nil_step, let_string_step, list, list_function_ref,
        local_bool, local_int, local_nil, local_string, nil, string, tuple_function_ref,
    };

    #[test]
    fn primitive_block_helpers_build_block_shapes() {
        assert_eq!(
            block_int([let_int_step(0, "x", int(1))], local_int(0, "x")).0,
            IntExpr::block(vec![let_int_step(0, "x", int(1))], local_int(0, "x").into()),
        );
        assert_eq!(
            block_string([let_string_step(0, "x", string("a"))], local_string(0, "x")).0,
            StringExpr::block(
                vec![let_string_step(0, "x", string("a"))],
                local_string(0, "x").into(),
            ),
        );
        assert_eq!(
            block_float([], float(1.0)).0,
            FloatExpr::block(Vec::new(), float(1.0).into()),
        );
        assert_eq!(
            block_bool([let_bool_step(0, "x", bool_(true))], local_bool(0, "x")).0,
            BoolExpr::block(
                vec![let_bool_step(0, "x", bool_(true))],
                local_bool(0, "x").into(),
            ),
        );
        assert_eq!(
            block_nil([let_nil_step(0, "x", nil())], local_nil(0, "x")).0,
            NilExpr::block(vec![let_nil_step(0, "x", nil())], local_nil(0, "x").into()),
        );
        assert_eq!(
            block_list([], list([int(1)], ValueType::Int)).0,
            ListExpr::block(Vec::new(), list([int(1)], ValueType::Int).into()),
        );
    }

    #[test]
    fn function_block_helpers_preserve_return_family() {
        assert_eq!(
            FunctionExpr::from(block_function(
                vec![],
                function_ref(
                    RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                    [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                ),
            )),
            FunctionExpr::int(IntFunctionExpr::block(
                Vec::new(),
                IntFunctionExpr::value(IntFunctionValue::new(
                    IntFunctionId(0),
                    vec![ParamLocal::int(crate::plan::IntLocalId(0))],
                )),
            )),
        );
        assert_eq!(
            FunctionExpr::from(block_function(
                vec![],
                function_ref(
                    RuntimeFunctionId::String(crate::plan::StringFunctionId(0)),
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                ),
            )),
            FunctionExpr::string(crate::plan::StringFunctionExpr::block(
                Vec::new(),
                crate::plan::StringFunctionExpr::value(StringFunctionValue::new(
                    StringFunctionId(0),
                    vec![ParamLocal::string(crate::plan::StringLocalId(0))],
                )),
            )),
        );
        assert_eq!(
            FunctionExpr::from(block_function(
                vec![],
                function_ref(
                    RuntimeFunctionId::Float(crate::plan::FloatFunctionId(0)),
                    [crate::plan::LocalId::Float(crate::plan::FloatLocalId(0))],
                ),
            )),
            FunctionExpr::float(crate::plan::FloatFunctionExpr::block(
                Vec::new(),
                crate::plan::FloatFunctionExpr::value(FloatFunctionValue::new(
                    FloatFunctionId(0),
                    vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
                )),
            )),
        );
        assert_eq!(
            FunctionExpr::from(block_function(
                vec![],
                function_ref(
                    RuntimeFunctionId::Bool(crate::plan::BoolFunctionId(0)),
                    [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))],
                ),
            )),
            FunctionExpr::bool(crate::plan::BoolFunctionExpr::block(
                Vec::new(),
                crate::plan::BoolFunctionExpr::value(BoolFunctionValue::new(
                    BoolFunctionId(0),
                    vec![ParamLocal::bool(crate::plan::BoolLocalId(0))],
                )),
            )),
        );
        assert_eq!(
            FunctionExpr::from(block_function(
                vec![],
                function_ref(
                    RuntimeFunctionId::Nil(crate::plan::NilFunctionId(0)),
                    [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))],
                ),
            )),
            FunctionExpr::nil(crate::plan::NilFunctionExpr::block(
                Vec::new(),
                crate::plan::NilFunctionExpr::value(NilFunctionValue::new(
                    NilFunctionId(0),
                    vec![ParamLocal::nil(crate::plan::NilLocalId(0))],
                )),
            )),
        );
        assert_eq!(
            FunctionExpr::from(block_function(
                vec![],
                Function::from(tuple_function_ref(
                    0,
                    [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                    [ValueType::Int, ValueType::String],
                )),
            )),
            FunctionExpr::tuple(TupleFunctionExpr::block(
                Vec::new(),
                TupleFunctionExpr::value(TupleFunctionValue::new(
                    crate::plan::TupleFunctionId(0),
                    vec![ParamLocal::int(crate::plan::IntLocalId(0))],
                    vec![ValueType::Int, ValueType::String],
                )),
            )),
        );
        assert_eq!(
            FunctionExpr::from(block_function(
                vec![],
                Function::from(list_function_ref(
                    0,
                    Vec::<ParamLocal>::new(),
                    ValueType::Int
                )),
            )),
            FunctionExpr::list(ListFunctionExpr::block(
                Vec::new(),
                ListFunctionExpr::value(ListFunctionValue::new(
                    crate::plan::ListFunctionId(0),
                    Vec::new(),
                    ValueType::Int,
                )),
            )),
        );
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert_eq!(
            FunctionExpr::from(block_function(
                vec![],
                Function::from(function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type.clone(),
                )),
            )),
            FunctionExpr::function(FunctionFunctionExpr::block(
                Vec::new(),
                FunctionFunctionExpr::value(FunctionFunctionValue::new(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    returned_function_type.clone(),
                )),
            )),
        );
        assert_eq!(
            block_int_function(
                [],
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]),
            )
            .0,
            IntFunctionExpr::block(
                Vec::new(),
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]).into(),
            ),
        );
        assert_eq!(
            block_tuple_function(
                [],
                tuple_function_ref(
                    0,
                    [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                    [ValueType::Int, ValueType::String],
                ),
            )
            .0,
            TupleFunctionExpr::block(
                Vec::new(),
                tuple_function_ref(
                    0,
                    [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                    [ValueType::Int, ValueType::String],
                )
                .into(),
            ),
        );
        assert_eq!(
            block_list_function(
                [],
                list_function_ref(0, Vec::<ParamLocal>::new(), ValueType::Int)
            )
            .0,
            ListFunctionExpr::block(
                Vec::new(),
                list_function_ref(0, Vec::<ParamLocal>::new(), ValueType::Int).into(),
            ),
        );
        assert_eq!(
            block_function_function(
                [],
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type.clone(),
                ),
            )
            .0,
            FunctionFunctionExpr::block(
                Vec::new(),
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type,
                )
                .into(),
            ),
        );
    }
}
