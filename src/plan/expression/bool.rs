use super::{BoolFunctionExpr, CallArg, Expr, FloatExpr, IntExpr, StringExpr};
use crate::plan::{BoolFunctionId, BoolLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct BoolExpr {
    kind: BoolExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BoolExprKind {
    Value(bool),
    LocalGet {
        local: BoolLocalId,
        name: EcoString,
    },
    Call {
        function: BoolFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<BoolFunctionExpr>,
        args: Vec<CallArg>,
    },
    Not(Box<BoolExpr>),
    LtInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    LtEqInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    GtInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    GtEqInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    LtFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    LtEqFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    GtFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    GtEqFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    Equal {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    NotEqual {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    And {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
    },
    Or {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<BoolExpr>,
        false_: Box<BoolExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, BoolExpr)>,
        fallback: Box<BoolExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, BoolExpr)>,
        fallback: Box<BoolExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, BoolExpr)>,
        fallback: Box<BoolExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<BoolExpr>,
    },
}

impl BoolExpr {
    pub(crate) fn value(value: bool) -> Self {
        Self {
            kind: BoolExprKind::Value(value),
        }
    }

    pub(crate) fn local_get(local: BoolLocalId, name: EcoString) -> Self {
        Self {
            kind: BoolExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: BoolFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: BoolExprKind::Call { function, args },
        }
    }

    pub(crate) fn function_call(function: BoolFunctionExpr, args: Vec<CallArg>) -> Self {
        Self {
            kind: BoolExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        }
    }

    pub(crate) fn not(value: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::Not(Box::new(value)),
        }
    }

    pub(crate) fn lt_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::LtInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn lte_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::LtEqInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn gt_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::GtInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn gte_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::GtEqInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn lt_float(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: BoolExprKind::LtFloat {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn lte_float(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: BoolExprKind::LtEqFloat {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn gt_float(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: BoolExprKind::GtFloat {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn gte_float(left: FloatExpr, right: FloatExpr) -> Self {
        Self {
            kind: BoolExprKind::GtEqFloat {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn equal(left: Expr, right: Expr) -> Self {
        Self {
            kind: BoolExprKind::Equal {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn not_equal(left: Expr, right: Expr) -> Self {
        Self {
            kind: BoolExprKind::NotEqual {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn and(left: BoolExpr, right: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::And {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn or(left: BoolExpr, right: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::Or {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: BoolExpr, false_: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, BoolExpr)>,
        fallback: BoolExpr,
    ) -> Self {
        Self {
            kind: BoolExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, BoolExpr)>,
        fallback: BoolExpr,
    ) -> Self {
        Self {
            kind: BoolExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, BoolExpr)>,
        fallback: BoolExpr,
    ) -> Self {
        Self {
            kind: BoolExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &BoolExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{BoolExpr, BoolExprKind};
    use crate::plan::{BoolFunctionId, BoolFunctionValue, Expr, IntExpr, Step};

    #[test]
    fn bool_expr_kind_accessors() {
        assert!(matches!(
            BoolExpr::value(true).kind(),
            BoolExprKind::Value(true)
        ));
        assert!(matches!(
            BoolExpr::bool_case(
                BoolExpr::value(true),
                BoolExpr::value(true),
                BoolExpr::value(false)
            )
            .kind(),
            BoolExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            BoolExpr::function_call(function_expr(), Vec::new()).kind(),
            BoolExprKind::FunctionCall { .. }
        ));
        assert!(matches!(
            BoolExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), BoolExpr::value(true))],
                BoolExpr::value(false)
            )
            .kind(),
            BoolExprKind::IntCase { .. }
        ));
        assert!(matches!(
            BoolExpr::string_case(
                crate::plan::StringExpr::value("a".into()),
                vec![("a".into(), BoolExpr::value(true))],
                BoolExpr::value(false)
            )
            .kind(),
            BoolExprKind::StringCase { .. }
        ));
        assert!(matches!(
            BoolExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, BoolExpr::value(true))],
                BoolExpr::value(false)
            )
            .kind(),
            BoolExprKind::FloatCase { .. }
        ));
        assert!(matches!(
            BoolExpr::lt_float(
                crate::plan::FloatExpr::value(1.0),
                crate::plan::FloatExpr::value(2.0)
            )
            .kind(),
            BoolExprKind::LtFloat { .. }
        ));
        assert!(matches!(
            BoolExpr::and(BoolExpr::value(true), BoolExpr::value(false)).kind(),
            BoolExprKind::And { .. }
        ));
        assert!(matches!(
            BoolExpr::or(BoolExpr::value(true), BoolExpr::value(false)).kind(),
            BoolExprKind::Or { .. }
        ));
        assert!(matches!(
            BoolExpr::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::value(false)))],
                BoolExpr::value(true),
            )
            .kind(),
            BoolExprKind::Block { .. }
        ));
    }

    fn function_expr() -> crate::plan::BoolFunctionExpr {
        crate::plan::BoolFunctionExpr::value(BoolFunctionValue::new(BoolFunctionId(0), Vec::new()))
    }
}
