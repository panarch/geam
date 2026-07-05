use crate::plan::{
    BoolExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId, BoolFunctionValue,
    CaptureArg, FloatExpr, FunctionFunctionExpr, FunctionType, IntExpr, ParamLocal, Step,
    StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct BoolFunctionExpr {
    type_: FunctionType,
    kind: BoolFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BoolFunctionExprKind {
    Value(BoolFunctionValue),
    Closure {
        runtime_id: BoolFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: BoolFunctionLocalId,
        name: EcoString,
    },
    Call {
        function: BoolFunctionFunctionId,
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
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<BoolFunctionExpr>,
        false_: Box<BoolFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, BoolFunctionExpr)>,
        fallback: Box<BoolFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, BoolFunctionExpr)>,
        fallback: Box<BoolFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, BoolFunctionExpr)>,
        fallback: Box<BoolFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<BoolFunctionExpr>,
    },
}

impl BoolFunctionExpr {
    pub(crate) fn value(value: BoolFunctionValue) -> Self {
        Self {
            type_: value.type_(),
            kind: BoolFunctionExprKind::Value(value),
        }
    }

    pub(crate) fn closure(
        runtime_id: BoolFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: BoolFunctionExprKind::Closure {
                runtime_id,
                params,
                captures,
            },
        }
    }

    pub(crate) fn local_get(
        local: BoolFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: BoolFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: BoolFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: BoolFunctionExprKind::Call {
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
            kind: BoolFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: FunctionType) -> Self {
        Self {
            type_: type_.clone(),
            kind: BoolFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
                type_,
            },
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: BoolFunctionExpr,
        false_: BoolFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: BoolFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, BoolFunctionExpr)>,
        fallback: BoolFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: BoolFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, BoolFunctionExpr)>,
        fallback: BoolFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: BoolFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, BoolFunctionExpr)>,
        fallback: BoolFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: BoolFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: BoolFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: BoolFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &BoolFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{BoolFunctionExpr, BoolFunctionExprKind};
    use crate::plan::{
        BoolExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId, BoolFunctionValue,
        BoolLocalId, Expr, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionValue,
        FunctionType, IntExpr, ParamLocal, Step, StringExpr, ValueType,
    };

    #[test]
    fn bool_function_expr_kind_accessors() {
        assert_eq!(
            function_value().kind(),
            &BoolFunctionExprKind::Value(BoolFunctionValue::new(
                BoolFunctionId(0),
                vec![ParamLocal::bool(BoolLocalId(0))],
            )),
        );
        assert_eq!(
            BoolFunctionExpr::closure(
                BoolFunctionId(0),
                vec![ParamLocal::bool(BoolLocalId(0))],
                Vec::new(),
                function_type(),
            )
            .kind(),
            &BoolFunctionExprKind::Closure {
                runtime_id: BoolFunctionId(0),
                params: vec![ParamLocal::bool(BoolLocalId(0))],
                captures: Vec::new(),
            },
        );
        assert_eq!(
            BoolFunctionExpr::local_get(BoolFunctionLocalId(0), "f".into(), function_type()).kind(),
            &BoolFunctionExprKind::LocalGet {
                local: BoolFunctionLocalId(0),
                name: "f".into(),
            },
        );
        assert_eq!(
            BoolFunctionExpr::call(BoolFunctionFunctionId(0), Vec::new(), function_type()).kind(),
            &BoolFunctionExprKind::Call {
                function: BoolFunctionFunctionId(0),
                args: Vec::new(),
                type_: function_type(),
            },
        );
        assert_eq!(
            BoolFunctionExpr::function_call(function_function_value(), Vec::new(), function_type())
                .kind(),
            &BoolFunctionExprKind::FunctionCall {
                function: Box::new(function_function_value()),
                args: Vec::new(),
                type_: function_type(),
            },
        );
        assert_eq!(
            BoolFunctionExpr::tuple_index(tuple_expr(), 0, function_type()).kind(),
            &BoolFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
                type_: function_type(),
            },
        );
        assert_eq!(
            BoolFunctionExpr::bool_case(BoolExpr::value(true), function_value(), function_value(),)
                .kind(),
            &BoolFunctionExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(function_value()),
                false_: Box::new(function_value()),
            },
        );
        assert_eq!(
            BoolFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                function_value(),
            )
            .kind(),
            &BoolFunctionExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            BoolFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), function_value())],
                function_value(),
            )
            .kind(),
            &BoolFunctionExprKind::StringCase {
                subject: Box::new(StringExpr::value("one".into())),
                clauses: vec![("one".into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            BoolFunctionExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, function_value())],
                function_value(),
            )
            .kind(),
            &BoolFunctionExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            BoolFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                function_value(),
            )
            .kind(),
            &BoolFunctionExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(function_value()),
            },
        );
    }

    #[test]
    fn bool_function_expr_type() {
        assert_eq!(function_value().type_(), &function_type());
    }

    fn function_value() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            vec![ParamLocal::bool(BoolLocalId(0))],
        ))
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Bool], ValueType::Bool)
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Bool(BoolFunctionFunctionId(0)),
            Vec::new(),
            function_type(),
        ))
    }

    fn tuple_expr() -> crate::plan::TupleExpr {
        crate::plan::TupleExpr::value(
            vec![Expr::function(crate::plan::FunctionExpr::bool(
                function_value(),
            ))],
            vec![ValueType::Function(Box::new(function_type()))],
        )
    }
}
