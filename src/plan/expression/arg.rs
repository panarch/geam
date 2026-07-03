use super::{
    BoolExpr, BoolFunctionExpr, Expr, ExprKind, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr,
    IntExpr, IntFunctionExpr, ListExpr, ListFunctionExpr, NilExpr, NilFunctionExpr, StringExpr,
    StringFunctionExpr, TupleExpr, TupleFunctionExpr,
};
use crate::plan::{
    BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocalId,
    IntFunctionLocalId, IntLocalId, ListFunctionLocalId, ListLocalId, NilFunctionLocalId,
    NilLocalId, ParamLocal, StringFunctionLocalId, StringLocalId, TupleFunctionLocalId,
    TupleLocalId,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    kind: CallArgKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CallArgKind {
    Int {
        local: IntLocalId,
        value: IntExpr,
    },
    String {
        local: StringLocalId,
        value: StringExpr,
    },
    Float {
        local: FloatLocalId,
        value: FloatExpr,
    },
    Bool {
        local: BoolLocalId,
        value: BoolExpr,
    },
    Nil {
        local: NilLocalId,
        value: NilExpr,
    },
    Tuple {
        local: TupleLocalId,
        value: TupleExpr,
    },
    List {
        local: ListLocalId,
        value: ListExpr,
    },
    IntFunction {
        local: IntFunctionLocalId,
        value: IntFunctionExpr,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: StringFunctionExpr,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: FloatFunctionExpr,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: BoolFunctionExpr,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: NilFunctionExpr,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: TupleFunctionExpr,
    },
    ListFunction {
        local: ListFunctionLocalId,
        value: ListFunctionExpr,
    },
    FunctionFunction {
        local: FunctionFunctionLocalId,
        value: FunctionFunctionExpr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CaptureArg {
    kind: CaptureArgKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CaptureArgKind {
    Int {
        local: IntLocalId,
        value: IntExpr,
    },
    String {
        local: StringLocalId,
        value: StringExpr,
    },
    Float {
        local: FloatLocalId,
        value: FloatExpr,
    },
    Bool {
        local: BoolLocalId,
        value: BoolExpr,
    },
    Nil {
        local: NilLocalId,
        value: NilExpr,
    },
    Tuple {
        local: TupleLocalId,
        value: TupleExpr,
    },
    List {
        local: ListLocalId,
        value: ListExpr,
    },
    IntFunction {
        local: IntFunctionLocalId,
        value: IntFunctionExpr,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: StringFunctionExpr,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: FloatFunctionExpr,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: BoolFunctionExpr,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: NilFunctionExpr,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: TupleFunctionExpr,
    },
    ListFunction {
        local: ListFunctionLocalId,
        value: ListFunctionExpr,
    },
    FunctionFunction {
        local: FunctionFunctionLocalId,
        value: FunctionFunctionExpr,
    },
}

impl Expr {
    pub(crate) fn into_call_arg(self, local: &ParamLocal) -> Option<CallArg> {
        match (local, self.kind) {
            (ParamLocal::Int(local), ExprKind::Int(value)) => Some(CallArg::int(*local, value)),
            (ParamLocal::String(local), ExprKind::String(value)) => {
                Some(CallArg::string(*local, value))
            }
            (ParamLocal::Float(local), ExprKind::Float(value)) => {
                Some(CallArg::float(*local, value))
            }
            (ParamLocal::Bool(local), ExprKind::Bool(value)) => Some(CallArg::bool(*local, value)),
            (ParamLocal::Nil(local), ExprKind::Nil(value)) => Some(CallArg::nil(*local, value)),
            (
                ParamLocal::Tuple {
                    local,
                    type_: expected,
                },
                ExprKind::Tuple(value),
            ) if value.type_() == expected => Some(CallArg::tuple(*local, value)),
            (
                ParamLocal::List {
                    local,
                    element_type: expected,
                },
                ExprKind::List(value),
            ) if value.element_type() == expected => Some(CallArg::list(*local, value)),
            (
                ParamLocal::IntFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_int()
                .map(|value| CallArg::int_function(*local, value)),
            (
                ParamLocal::StringFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_string()
                .map(|value| CallArg::string_function(*local, value)),
            (
                ParamLocal::FloatFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_float()
                .map(|value| CallArg::float_function(*local, value)),
            (
                ParamLocal::BoolFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_bool()
                .map(|value| CallArg::bool_function(*local, value)),
            (
                ParamLocal::NilFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_nil()
                .map(|value| CallArg::nil_function(*local, value)),
            (
                ParamLocal::TupleFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_tuple()
                .map(|value| CallArg::tuple_function(*local, value)),
            (
                ParamLocal::ListFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_list()
                .map(|value| CallArg::list_function(*local, value)),
            (
                ParamLocal::FunctionFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_function()
                .map(|value| CallArg::function_function(*local, value)),
            _ => None,
        }
    }
}

impl CallArg {
    pub(crate) fn int(local: IntLocalId, value: IntExpr) -> Self {
        Self {
            kind: CallArgKind::Int { local, value },
        }
    }

    pub(crate) fn string(local: StringLocalId, value: StringExpr) -> Self {
        Self {
            kind: CallArgKind::String { local, value },
        }
    }

    pub(crate) fn float(local: FloatLocalId, value: FloatExpr) -> Self {
        Self {
            kind: CallArgKind::Float { local, value },
        }
    }

    pub(crate) fn bool(local: BoolLocalId, value: BoolExpr) -> Self {
        Self {
            kind: CallArgKind::Bool { local, value },
        }
    }

    pub(crate) fn nil(local: NilLocalId, value: NilExpr) -> Self {
        Self {
            kind: CallArgKind::Nil { local, value },
        }
    }

    pub(crate) fn tuple(local: TupleLocalId, value: TupleExpr) -> Self {
        Self {
            kind: CallArgKind::Tuple { local, value },
        }
    }

    pub(crate) fn list(local: ListLocalId, value: ListExpr) -> Self {
        Self {
            kind: CallArgKind::List { local, value },
        }
    }

    pub(crate) fn int_function(local: IntFunctionLocalId, value: IntFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::IntFunction { local, value },
        }
    }

    pub(crate) fn string_function(local: StringFunctionLocalId, value: StringFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::StringFunction { local, value },
        }
    }

    pub(crate) fn float_function(local: FloatFunctionLocalId, value: FloatFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::FloatFunction { local, value },
        }
    }

    pub(crate) fn bool_function(local: BoolFunctionLocalId, value: BoolFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::BoolFunction { local, value },
        }
    }

    pub(crate) fn nil_function(local: NilFunctionLocalId, value: NilFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::NilFunction { local, value },
        }
    }

    pub(crate) fn tuple_function(local: TupleFunctionLocalId, value: TupleFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::TupleFunction { local, value },
        }
    }

    pub(crate) fn list_function(local: ListFunctionLocalId, value: ListFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::ListFunction { local, value },
        }
    }

    pub(crate) fn function_function(
        local: FunctionFunctionLocalId,
        value: FunctionFunctionExpr,
    ) -> Self {
        Self {
            kind: CallArgKind::FunctionFunction { local, value },
        }
    }

    pub(crate) fn kind(&self) -> &CallArgKind {
        &self.kind
    }
}

impl CaptureArg {
    pub(crate) fn int(local: IntLocalId, value: IntExpr) -> Self {
        Self {
            kind: CaptureArgKind::Int { local, value },
        }
    }

    pub(crate) fn string(local: StringLocalId, value: StringExpr) -> Self {
        Self {
            kind: CaptureArgKind::String { local, value },
        }
    }

    pub(crate) fn float(local: FloatLocalId, value: FloatExpr) -> Self {
        Self {
            kind: CaptureArgKind::Float { local, value },
        }
    }

    pub(crate) fn bool(local: BoolLocalId, value: BoolExpr) -> Self {
        Self {
            kind: CaptureArgKind::Bool { local, value },
        }
    }

    pub(crate) fn nil(local: NilLocalId, value: NilExpr) -> Self {
        Self {
            kind: CaptureArgKind::Nil { local, value },
        }
    }

    pub(crate) fn tuple(local: TupleLocalId, value: TupleExpr) -> Self {
        Self {
            kind: CaptureArgKind::Tuple { local, value },
        }
    }

    pub(crate) fn list(local: ListLocalId, value: ListExpr) -> Self {
        Self {
            kind: CaptureArgKind::List { local, value },
        }
    }

    pub(crate) fn int_function(local: IntFunctionLocalId, value: IntFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::IntFunction { local, value },
        }
    }

    pub(crate) fn string_function(local: StringFunctionLocalId, value: StringFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::StringFunction { local, value },
        }
    }

    pub(crate) fn float_function(local: FloatFunctionLocalId, value: FloatFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::FloatFunction { local, value },
        }
    }

    pub(crate) fn bool_function(local: BoolFunctionLocalId, value: BoolFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::BoolFunction { local, value },
        }
    }

    pub(crate) fn nil_function(local: NilFunctionLocalId, value: NilFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::NilFunction { local, value },
        }
    }

    pub(crate) fn tuple_function(local: TupleFunctionLocalId, value: TupleFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::TupleFunction { local, value },
        }
    }

    pub(crate) fn list_function(local: ListFunctionLocalId, value: ListFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::ListFunction { local, value },
        }
    }

    pub(crate) fn function_function(
        local: FunctionFunctionLocalId,
        value: FunctionFunctionExpr,
    ) -> Self {
        Self {
            kind: CaptureArgKind::FunctionFunction { local, value },
        }
    }

    pub(crate) fn kind(&self) -> &CaptureArgKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::CallArg;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionId, BoolFunctionLocalId, BoolFunctionValue,
        BoolLocalId, Expr, FloatExpr, FloatFunctionExpr, FloatFunctionId, FloatFunctionLocalId,
        FloatFunctionValue, FloatLocalId, FunctionExpr, FunctionFunctionExpr, FunctionFunctionId,
        FunctionFunctionLocalId, FunctionFunctionValue, FunctionType, FunctionValue, IntExpr,
        IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId,
        IntFunctionValue, IntLocalId, ListExpr, ListFunctionExpr, ListFunctionId,
        ListFunctionLocalId, ListFunctionValue, ListLocalId, NilExpr, NilFunctionExpr,
        NilFunctionId, NilFunctionLocalId, NilFunctionValue, NilLocalId, ParamLocal,
        RuntimeFunctionId, StringExpr, StringFunctionExpr, StringFunctionId, StringFunctionLocalId,
        StringFunctionValue, StringLocalId, TupleExpr, TupleFunctionExpr, TupleFunctionId,
        TupleFunctionLocalId, TupleFunctionValue, TupleLocalId, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn into_call_arg_preserves_param_family() {
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1)))
                .into_call_arg(&ParamLocal::int(IntLocalId(0))),
            Some(CallArg::int(IntLocalId(0), IntExpr::value(BigInt::from(1)),)),
        );
        assert_eq!(
            Expr::string(StringExpr::value("geam".into()))
                .into_call_arg(&ParamLocal::string(StringLocalId(0))),
            Some(CallArg::string(
                StringLocalId(0),
                StringExpr::value("geam".into()),
            )),
        );
        assert_eq!(
            Expr::float(FloatExpr::value(1.5)).into_call_arg(&ParamLocal::float(FloatLocalId(0))),
            Some(CallArg::float(FloatLocalId(0), FloatExpr::value(1.5))),
        );
        assert_eq!(
            Expr::bool(BoolExpr::value(true)).into_call_arg(&ParamLocal::bool(BoolLocalId(0))),
            Some(CallArg::bool(BoolLocalId(0), BoolExpr::value(true))),
        );
        assert_eq!(
            Expr::nil(NilExpr::value()).into_call_arg(&ParamLocal::nil(NilLocalId(0))),
            Some(CallArg::nil(NilLocalId(0), NilExpr::value())),
        );
        assert_eq!(
            tuple_expr().into_call_arg(&ParamLocal::tuple(TupleLocalId(0), vec![ValueType::Int])),
            Some(CallArg::tuple(
                TupleLocalId(0),
                TupleExpr::value(
                    vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                    vec![ValueType::Int],
                ),
            )),
        );
        assert_eq!(
            Expr::list(list_expr())
                .into_call_arg(&ParamLocal::list(ListLocalId(0), ValueType::Int)),
            Some(CallArg::list(ListLocalId(0), list_expr())),
        );
        assert_eq!(
            Expr::function(FunctionExpr::value(function_value())).into_call_arg(
                &ParamLocal::int_function(
                    IntFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                )
            ),
            Some(CallArg::int_function(
                IntFunctionLocalId(0),
                int_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::string(string_function_expr())).into_call_arg(
                &ParamLocal::string_function(
                    StringFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::String], ValueType::String),
                )
            ),
            Some(CallArg::string_function(
                StringFunctionLocalId(0),
                string_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::float(float_function_expr())).into_call_arg(
                &ParamLocal::float_function(
                    FloatFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Float], ValueType::Float),
                )
            ),
            Some(CallArg::float_function(
                FloatFunctionLocalId(0),
                float_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::bool(bool_function_expr())).into_call_arg(
                &ParamLocal::bool_function(
                    BoolFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
                )
            ),
            Some(CallArg::bool_function(
                BoolFunctionLocalId(0),
                bool_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::nil(nil_function_expr())).into_call_arg(
                &ParamLocal::nil_function(
                    NilFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
                )
            ),
            Some(CallArg::nil_function(
                NilFunctionLocalId(0),
                nil_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::tuple(tuple_function_expr())).into_call_arg(
                &ParamLocal::tuple_function(TupleFunctionLocalId(0), tuple_function_type())
            ),
            Some(CallArg::tuple_function(
                TupleFunctionLocalId(0),
                tuple_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::list(list_function_expr())).into_call_arg(
                &ParamLocal::list_function(ListFunctionLocalId(0), list_function_type())
            ),
            Some(CallArg::list_function(
                ListFunctionLocalId(0),
                list_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::function(function_function_expr())).into_call_arg(
                &ParamLocal::function_function(
                    FunctionFunctionLocalId(0),
                    function_function_type()
                )
            ),
            Some(CallArg::function_function(
                FunctionFunctionLocalId(0),
                function_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::string(malformed_string_function_expr(
                function_type(),
            )))
            .into_call_arg(&ParamLocal::int_function(
                IntFunctionLocalId(0),
                function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::bool(malformed_bool_function_expr(
                string_function_type(),
            )))
            .into_call_arg(&ParamLocal::string_function(
                StringFunctionLocalId(0),
                string_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::nil(malformed_nil_function_expr(
                bool_function_type(),
            )))
            .into_call_arg(&ParamLocal::bool_function(
                BoolFunctionLocalId(0),
                bool_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::int(malformed_int_function_expr(
                function_function_type(),
            )))
            .into_call_arg(&ParamLocal::function_function(
                FunctionFunctionLocalId(0),
                function_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::int(malformed_int_function_expr(
                nil_function_type(),
            )))
            .into_call_arg(&ParamLocal::nil_function(
                NilFunctionLocalId(0),
                nil_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::string(malformed_string_function_expr(
                tuple_function_type(),
            )))
            .into_call_arg(&ParamLocal::tuple_function(
                TupleFunctionLocalId(0),
                tuple_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::value(function_value()))
                .into_call_arg(&ParamLocal::int(IntLocalId(0))),
            None,
        );
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1)))
                .into_call_arg(&ParamLocal::bool(BoolLocalId(0))),
            None,
        );
    }

    fn function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(IntLocalId(0))],
        )
    }

    fn int_function_expr() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    fn string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            vec![ParamLocal::string(StringLocalId(0))],
        ))
    }

    fn float_function_expr() -> FloatFunctionExpr {
        FloatFunctionExpr::value(FloatFunctionValue::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(FloatLocalId(0))],
        ))
    }

    fn bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            vec![ParamLocal::bool(BoolLocalId(0))],
        ))
    }

    fn nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(0),
            vec![ParamLocal::nil(NilLocalId(0))],
        ))
    }

    fn tuple_expr() -> Expr {
        Expr::tuple(TupleExpr::value(
            vec![Expr::int(IntExpr::value(BigInt::from(1)))],
            vec![ValueType::Int],
        ))
    }

    fn list_expr() -> ListExpr {
        ListExpr::value(
            vec![Expr::int(IntExpr::value(BigInt::from(1)))],
            ValueType::Int,
        )
    }

    fn tuple_function_expr() -> TupleFunctionExpr {
        TupleFunctionExpr::value(TupleFunctionValue::new(
            TupleFunctionId(0),
            vec![ParamLocal::tuple(TupleLocalId(0), vec![ValueType::Int])],
            vec![ValueType::Int],
        ))
    }

    fn list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::value(ListFunctionValue::new(
            ListFunctionId(0),
            vec![ParamLocal::list(ListLocalId(0), ValueType::Int)],
            ValueType::Int,
        ))
    }

    fn function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            function_type(),
        ))
    }

    fn malformed_int_function_expr(type_: FunctionType) -> IntFunctionExpr {
        IntFunctionExpr::local_get(IntFunctionLocalId(0), "f".into(), type_)
    }

    fn malformed_string_function_expr(type_: FunctionType) -> StringFunctionExpr {
        StringFunctionExpr::local_get(StringFunctionLocalId(0), "f".into(), type_)
    }

    fn malformed_bool_function_expr(type_: FunctionType) -> BoolFunctionExpr {
        BoolFunctionExpr::local_get(BoolFunctionLocalId(0), "f".into(), type_)
    }

    fn malformed_nil_function_expr(type_: FunctionType) -> NilFunctionExpr {
        NilFunctionExpr::local_get(NilFunctionLocalId(0), "f".into(), type_)
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn string_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::String], ValueType::String)
    }

    fn bool_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Bool], ValueType::Bool)
    }

    fn nil_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Nil], ValueType::Nil)
    }

    fn tuple_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Tuple(vec![ValueType::Int])],
            ValueType::Tuple(vec![ValueType::Int]),
        )
    }

    fn list_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::List(Box::new(ValueType::Int))],
            ValueType::List(Box::new(ValueType::Int)),
        )
    }

    fn function_function_type() -> FunctionType {
        FunctionType::new(Vec::new(), ValueType::Function(Box::new(function_type())))
    }
}
