use crate::plan::{
    BoolExpr, CaptureArg, FloatExpr, FunctionFunctionExpr, FunctionListExpr, FunctionType, IntExpr,
    PanicExpr, ParamLocal, Step, StringExpr, StringFunctionFunctionId, StringFunctionId,
    StringFunctionLocalId, StringFunctionValue, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct StringFunctionExpr {
    type_: FunctionType,
    kind: StringFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StringFunctionExprKind {
    Value(StringFunctionValue),
    Closure {
        runtime_id: StringFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: StringFunctionLocalId,
        name: EcoString,
    },
    Call {
        function: StringFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    },
    FunctionCall {
        function: Box<FunctionFunctionExpr>,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
        type_: FunctionType,
    },
    ListIndex {
        list: Box<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<StringFunctionExpr>,
        false_: Box<StringFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, StringFunctionExpr)>,
        fallback: Box<StringFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, StringFunctionExpr)>,
        fallback: Box<StringFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, StringFunctionExpr)>,
        fallback: Box<StringFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<StringFunctionExpr>,
    },
}

impl StringFunctionExpr {
    pub(crate) fn value(value: StringFunctionValue) -> Self {
        Self {
            type_: value.type_(),
            kind: StringFunctionExprKind::Value(value),
        }
    }

    pub(crate) fn closure(
        runtime_id: StringFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: StringFunctionExprKind::Closure {
                runtime_id,
                params,
                captures,
            },
        }
    }

    pub(crate) fn local_get(
        local: StringFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: StringFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: StringFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: StringFunctionExprKind::Call {
                function,
                args,
                type_,
            },
        }
    }

    pub(crate) fn function_call(
        function: FunctionFunctionExpr,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: StringFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: FunctionType) -> Self {
        Self {
            type_: type_.clone(),
            kind: StringFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
                type_,
            },
        }
    }

    pub(crate) fn list_index(
        list: impl Into<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: StringFunctionExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
                type_,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: StringFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: StringFunctionExpr,
        false_: StringFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: StringFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: StringFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: StringFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: StringFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: StringFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: StringFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &StringFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{StringFunctionExpr, StringFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionValue,
        FunctionType, IntExpr, ParamLocal, Step, StringExpr, StringFunctionFunctionId,
        StringFunctionId, StringFunctionLocalId, StringFunctionValue, StringLocalId, ValueType,
    };

    #[test]
    fn string_function_expr_kind_accessors() {
        assert_eq!(
            function_value().kind(),
            &StringFunctionExprKind::Value(StringFunctionValue::new(
                StringFunctionId(0),
                vec![ParamLocal::string(StringLocalId(0))],
            )),
        );
        assert_eq!(
            StringFunctionExpr::closure(
                StringFunctionId(0),
                vec![ParamLocal::string(StringLocalId(0))],
                Vec::new(),
                function_type(),
            )
            .kind(),
            &StringFunctionExprKind::Closure {
                runtime_id: StringFunctionId(0),
                params: vec![ParamLocal::string(StringLocalId(0))],
                captures: Vec::new(),
            },
        );
        assert_eq!(
            StringFunctionExpr::local_get(StringFunctionLocalId(0), "f".into(), function_type())
                .kind(),
            &StringFunctionExprKind::LocalGet {
                local: StringFunctionLocalId(0),
                name: "f".into(),
            },
        );
        assert_eq!(
            StringFunctionExpr::call(StringFunctionFunctionId(0), Vec::new(), function_type())
                .kind(),
            &StringFunctionExprKind::Call {
                function: StringFunctionFunctionId(0),
                args: Vec::new(),
                type_: function_type(),
            },
        );
        assert_eq!(
            StringFunctionExpr::function_call(
                function_function_value(),
                Vec::new(),
                function_type(),
            )
            .kind(),
            &StringFunctionExprKind::FunctionCall {
                function: Box::new(function_function_value()),
                args: Vec::new(),
                type_: function_type(),
            },
        );
        assert_eq!(
            StringFunctionExpr::tuple_index(tuple_expr(), 0, function_type()).kind(),
            &StringFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
                type_: function_type(),
            },
        );
        assert_eq!(
            StringFunctionExpr::bool_case(
                BoolExpr::value(true),
                function_value(),
                function_value(),
            )
            .kind(),
            &StringFunctionExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(function_value()),
                false_: Box::new(function_value()),
            },
        );
        assert_eq!(
            StringFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                function_value(),
            )
            .kind(),
            &StringFunctionExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            StringFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), function_value())],
                function_value(),
            )
            .kind(),
            &StringFunctionExprKind::StringCase {
                subject: Box::new(StringExpr::value("one".into())),
                clauses: vec![("one".into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            StringFunctionExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, function_value())],
                function_value(),
            )
            .kind(),
            &StringFunctionExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            StringFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                function_value(),
            )
            .kind(),
            &StringFunctionExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(function_value()),
            },
        );
    }

    #[test]
    fn string_function_expr_type() {
        assert_eq!(function_value().type_(), &function_type());
    }

    fn function_value() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            vec![ParamLocal::string(StringLocalId(0))],
        ))
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::String], ValueType::String)
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::String(StringFunctionFunctionId(0)),
            Vec::new(),
            function_type(),
        ))
    }

    fn tuple_expr() -> crate::plan::TupleExpr {
        crate::plan::TupleExpr::value(
            vec![Expr::function(crate::plan::FunctionExpr::string(
                function_value(),
            ))],
            vec![ValueType::Function(Box::new(function_type()))],
        )
    }
}
