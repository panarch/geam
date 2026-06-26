use super::{BoolExpr, CallArg};
use crate::plan::{IntFunctionId, IntLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct IntExpr {
    kind: IntExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum IntExprKind {
    Value(BigInt),
    LocalGet {
        local: IntLocalId,
        name: EcoString,
    },
    Call {
        function: IntFunctionId,
        args: Vec<CallArg>,
    },
    Add {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Sub {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Mult {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Div {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Remainder {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Negate(Box<IntExpr>),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<IntExpr>,
        false_: Box<IntExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, IntExpr)>,
        fallback: Box<IntExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<IntExpr>,
    },
}

impl IntExpr {
    pub(crate) fn value(value: BigInt) -> Self {
        Self {
            kind: IntExprKind::Value(value),
        }
    }

    pub(crate) fn local_get(local: IntLocalId, name: EcoString) -> Self {
        Self {
            kind: IntExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: IntFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: IntExprKind::Call { function, args },
        }
    }

    pub(crate) fn add(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Add {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn sub(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Sub {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn mult(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Mult {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn div(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Div {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn remainder(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Remainder {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn negate(value: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Negate(Box::new(value)),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: IntExpr, false_: IntExpr) -> Self {
        Self {
            kind: IntExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, IntExpr)>,
        fallback: IntExpr,
    ) -> Self {
        Self {
            kind: IntExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &IntExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{IntExpr, IntExprKind};
    use crate::plan::{BoolExpr, Expr, Step};

    #[test]
    fn int_expr_kind_accessors() {
        assert!(matches!(
            IntExpr::value(1.into()).kind(),
            IntExprKind::Value(_)
        ));
        assert!(matches!(
            IntExpr::bool_case(
                BoolExpr::value(true),
                IntExpr::value(1.into()),
                IntExpr::value(0.into())
            )
            .kind(),
            IntExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            IntExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), IntExpr::value(10.into()))],
                IntExpr::value(0.into())
            )
            .kind(),
            IntExprKind::IntCase { .. }
        ));
        assert!(matches!(
            IntExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                IntExpr::value(2.into()),
            )
            .kind(),
            IntExprKind::Block { .. }
        ));
    }
}
