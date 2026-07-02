use super::FunctionReturn;
use crate::plan::{
    BoolFunctionExpr, BoolReturn, FloatFunctionExpr, FloatReturn, FunctionExpr, FunctionExprKind,
    FunctionFunctionExpr, IntFunctionExpr, IntReturn, NilFunctionExpr, NilReturn, ReturnBody,
    StringFunctionExpr, StringReturn, TupleFunctionExpr, TupleReturn,
};
use crate::planner::dsl::expression::{
    Bool, BoolFunction, Float, FloatFunction, Function, FunctionFunction, Int, IntFunction, Nil,
    NilFunction, String, StringFunction, Tuple, TupleFunction,
};

impl From<Int> for FunctionReturn {
    fn from(value: Int) -> Self {
        Self::Int(ReturnBody::expr(value.into()))
    }
}

impl From<String> for FunctionReturn {
    fn from(value: String) -> Self {
        Self::String(ReturnBody::expr(value.into()))
    }
}

impl From<Float> for FunctionReturn {
    fn from(value: Float) -> Self {
        Self::Float(ReturnBody::expr(value.into()))
    }
}

impl From<Bool> for FunctionReturn {
    fn from(value: Bool) -> Self {
        Self::Bool(ReturnBody::expr(value.into()))
    }
}

impl From<Nil> for FunctionReturn {
    fn from(value: Nil) -> Self {
        Self::Nil(ReturnBody::expr(value.into()))
    }
}

impl From<Tuple> for FunctionReturn {
    fn from(value: Tuple) -> Self {
        let expression = crate::plan::TupleExpr::from(value);
        Self::Tuple {
            type_: expression.type_().to_vec(),
            body: TupleReturn::expr(expression),
        }
    }
}

impl From<IntFunction> for FunctionReturn {
    fn from(value: IntFunction) -> Self {
        let expression = IntFunctionExpr::from(value);
        Self::IntFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<StringFunction> for FunctionReturn {
    fn from(value: StringFunction) -> Self {
        let expression = StringFunctionExpr::from(value);
        Self::StringFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<FloatFunction> for FunctionReturn {
    fn from(value: FloatFunction) -> Self {
        let expression = FloatFunctionExpr::from(value);
        Self::FloatFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<BoolFunction> for FunctionReturn {
    fn from(value: BoolFunction) -> Self {
        let expression = BoolFunctionExpr::from(value);
        Self::BoolFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<NilFunction> for FunctionReturn {
    fn from(value: NilFunction) -> Self {
        let expression = NilFunctionExpr::from(value);
        Self::NilFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<TupleFunction> for FunctionReturn {
    fn from(value: TupleFunction) -> Self {
        let expression = TupleFunctionExpr::from(value);
        Self::TupleFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<FunctionFunction> for FunctionReturn {
    fn from(value: FunctionFunction) -> Self {
        let expression = FunctionFunctionExpr::from(value);
        Self::FunctionFunction {
            type_: expression.type_().clone(),
            body: ReturnBody::expr(expression),
        }
    }
}

impl From<Function> for FunctionReturn {
    fn from(value: Function) -> Self {
        match FunctionExpr::from(value).into_kind() {
            FunctionExprKind::Int(expression) => Self::IntFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::String(expression) => Self::StringFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Float(expression) => Self::FloatFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Bool(expression) => Self::BoolFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Nil(expression) => Self::NilFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Tuple(expression) => Self::TupleFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
            FunctionExprKind::Function(expression) => Self::FunctionFunction {
                type_: expression.type_().clone(),
                body: ReturnBody::expr(expression),
            },
        }
    }
}

impl From<IntReturn> for FunctionReturn {
    fn from(value: IntReturn) -> Self {
        Self::Int(value)
    }
}

impl From<StringReturn> for FunctionReturn {
    fn from(value: StringReturn) -> Self {
        Self::String(value)
    }
}

impl From<FloatReturn> for FunctionReturn {
    fn from(value: FloatReturn) -> Self {
        Self::Float(value)
    }
}

impl From<BoolReturn> for FunctionReturn {
    fn from(value: BoolReturn) -> Self {
        Self::Bool(value)
    }
}

impl From<NilReturn> for FunctionReturn {
    fn from(value: NilReturn) -> Self {
        Self::Nil(value)
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionReturn;
    use crate::plan::{
        BoolFunctionId, BoolReturn, Expr, FunctionFunctionId, FunctionType, IntFunctionFunctionId,
        IntFunctionId, IntReturn, NilFunctionId, NilReturn, ParamLocal, ReturnBodyKind,
        RuntimeFunctionId, StringFunctionId, StringReturn, TupleFunctionId, ValueType,
    };
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, function_function_ref, function_ref, int, int_function_ref, nil,
        nil_function_ref, string, string_function_ref, tuple, tuple_function_ref,
    };

    #[test]
    fn value_conversions_build_function_return_families() {
        assert!(matches!(
            FunctionReturn::from(int(1)),
            FunctionReturn::Int(_),
        ));
        assert!(matches!(
            FunctionReturn::from(string("value")),
            FunctionReturn::String(_),
        ));
        assert!(matches!(
            FunctionReturn::from(bool_(true)),
            FunctionReturn::Bool(_),
        ));
        assert!(matches!(
            FunctionReturn::from(nil()),
            FunctionReturn::Nil(_),
        ));
        assert!(matches!(
            FunctionReturn::from(tuple([Expr::from(int(1))])),
            FunctionReturn::Tuple { .. },
        ));
    }

    #[test]
    fn function_value_conversions_preserve_return_family() {
        assert!(matches!(
            FunctionReturn::from(int_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionReturn::IntFunction { .. },
        ));
        assert!(matches!(
            FunctionReturn::from(string_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionReturn::StringFunction { .. },
        ));
        assert!(matches!(
            FunctionReturn::from(bool_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionReturn::BoolFunction { .. },
        ));
        assert!(matches!(
            FunctionReturn::from(nil_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionReturn::NilFunction { .. },
        ));
        assert!(matches!(
            FunctionReturn::from(tuple_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                [ValueType::Int],
            )),
            FunctionReturn::TupleFunction { .. },
        ));
        assert!(matches!(
            FunctionReturn::from(function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            )),
            FunctionReturn::FunctionFunction { .. },
        ));
    }

    #[test]
    fn erased_function_value_conversion_preserves_return_family() {
        assert!(matches!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::Int(IntFunctionId(0)),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::IntFunction { .. },
        ));
        assert!(matches!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::String(StringFunctionId(0)),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::StringFunction { .. },
        ));
        assert!(matches!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::Bool(BoolFunctionId(0)),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::BoolFunction { .. },
        ));
        assert!(matches!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::Nil(NilFunctionId(0)),
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::NilFunction { .. },
        ));
        assert!(matches!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::Tuple {
                    id: TupleFunctionId(0),
                    return_type: vec![ValueType::Int],
                },
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::TupleFunction { .. },
        ));
        assert!(matches!(
            FunctionReturn::from(function_ref(
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                },
                Vec::<ParamLocal>::new(),
            )),
            FunctionReturn::FunctionFunction { .. },
        ));
    }

    #[test]
    fn return_body_conversions_keep_existing_body_family() {
        assert!(matches!(
            FunctionReturn::from(IntReturn::expr(int(1).into())),
            FunctionReturn::Int(body) if matches!(body.kind(), ReturnBodyKind::Expr(_)),
        ));
        assert!(matches!(
            FunctionReturn::from(StringReturn::expr(string("value").into())),
            FunctionReturn::String(body) if matches!(body.kind(), ReturnBodyKind::Expr(_)),
        ));
        assert!(matches!(
            FunctionReturn::from(BoolReturn::expr(bool_(true).into())),
            FunctionReturn::Bool(body) if matches!(body.kind(), ReturnBodyKind::Expr(_)),
        ));
        assert!(matches!(
            FunctionReturn::from(NilReturn::expr(nil().into())),
            FunctionReturn::Nil(body) if matches!(body.kind(), ReturnBodyKind::Expr(_)),
        ));
    }
}
