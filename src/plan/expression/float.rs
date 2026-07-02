use super::{BoolExpr, CallArg, FloatFunctionExpr, IntExpr, StringExpr, TupleExpr};
use crate::plan::{FloatFunctionId, FloatLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct FloatExpr {
    kind: FloatExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FloatExprKind {
    Value(f64),
    LocalGet {
        local: FloatLocalId,
        name: EcoString,
    },
    Call {
        function: FloatFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<FloatFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    Add {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    Sub {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    Mult {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    Div {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<FloatExpr>,
        false_: Box<FloatExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, FloatExpr)>,
        fallback: Box<FloatExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, FloatExpr)>,
        fallback: Box<FloatExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, FloatExpr)>,
        fallback: Box<FloatExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<FloatExpr>,
    },
}

impl FloatExpr {
    pub(crate) fn value(value: f64) -> Self {
        Self {
            kind: FloatExprKind::Value(value),
        }
    }

    pub(crate) fn local_get(local: FloatLocalId, name: EcoString) -> Self {
        Self {
            kind: FloatExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: FloatFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: FloatExprKind::Call { function, args },
        }
    }

    pub(crate) fn function_call(function: FloatFunctionExpr, args: Vec<CallArg>) -> Self {
        Self {
            kind: FloatExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize) -> Self {
        Self {
            kind: FloatExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn add(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: FloatExprKind::Add {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn sub(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: FloatExprKind::Sub {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn mult(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: FloatExprKind::Mult {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn div(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: FloatExprKind::Div {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: FloatExpr, false_: FloatExpr) -> Self {
        Self {
            kind: FloatExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, FloatExpr)>,
        fallback: FloatExpr,
    ) -> Self {
        Self {
            kind: FloatExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, FloatExpr)>,
        fallback: FloatExpr,
    ) -> Self {
        Self {
            kind: FloatExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, FloatExpr)>,
        fallback: FloatExpr,
    ) -> Self {
        Self {
            kind: FloatExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: FloatExpr) -> Self {
        Self {
            kind: FloatExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &FloatExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{FloatExpr, FloatExprKind};
    use crate::plan::{BoolExpr, Expr, FloatFunctionId, FloatFunctionValue, Step};

    #[test]
    fn float_expr_kind_accessors() {
        assert!(matches!(
            FloatExpr::value(1.0).kind(),
            FloatExprKind::Value(_)
        ));
        assert!(matches!(
            FloatExpr::bool_case(
                BoolExpr::value(true),
                FloatExpr::value(1.0),
                FloatExpr::value(0.0)
            )
            .kind(),
            FloatExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            FloatExpr::function_call(function_expr(), Vec::new()).kind(),
            FloatExprKind::FunctionCall { .. }
        ));
        assert!(matches!(
            FloatExpr::tuple_index(
                crate::plan::TupleExpr::value(
                    vec![Expr::float(FloatExpr::value(1.0))],
                    vec![crate::plan::ValueType::Float],
                ),
                0,
            )
            .kind(),
            FloatExprKind::TupleIndex { .. }
        ));
        assert!(matches!(
            FloatExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, FloatExpr::value(10.0))],
                FloatExpr::value(0.0)
            )
            .kind(),
            FloatExprKind::FloatCase { .. }
        ));
        assert!(matches!(
            FloatExpr::string_case(
                crate::plan::StringExpr::value("a".into()),
                vec![("a".into(), FloatExpr::value(10.0))],
                FloatExpr::value(0.0)
            )
            .kind(),
            FloatExprKind::StringCase { .. }
        ));
        assert!(matches!(
            FloatExpr::block(
                vec![Step::evaluate(Expr::float(FloatExpr::value(1.0)))],
                FloatExpr::value(2.0),
            )
            .kind(),
            FloatExprKind::Block { .. }
        ));
    }

    fn function_expr() -> crate::plan::FloatFunctionExpr {
        crate::plan::FloatFunctionExpr::value(FloatFunctionValue::new(
            FloatFunctionId(0),
            Vec::new(),
        ))
    }
}
