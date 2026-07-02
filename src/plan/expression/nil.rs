use super::{BoolExpr, CallArg, IntExpr, NilFunctionExpr, StringExpr};
use crate::plan::{NilFunctionId, NilLocalId, Step};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct NilExpr {
    kind: NilExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NilExprKind {
    Value,
    LocalGet {
        local: NilLocalId,
        name: EcoString,
    },
    Call {
        function: NilFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<NilFunctionExpr>,
        args: Vec<CallArg>,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<NilExpr>,
        false_: Box<NilExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, NilExpr)>,
        fallback: Box<NilExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, NilExpr)>,
        fallback: Box<NilExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<NilExpr>,
    },
}

impl NilExpr {
    pub(crate) fn value() -> Self {
        Self {
            kind: NilExprKind::Value,
        }
    }

    pub(crate) fn local_get(local: NilLocalId, name: EcoString) -> Self {
        Self {
            kind: NilExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: NilFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: NilExprKind::Call { function, args },
        }
    }

    pub(crate) fn function_call(function: NilFunctionExpr, args: Vec<CallArg>) -> Self {
        Self {
            kind: NilExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: NilExpr, false_: NilExpr) -> Self {
        Self {
            kind: NilExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, NilExpr)>,
        fallback: NilExpr,
    ) -> Self {
        Self {
            kind: NilExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, NilExpr)>,
        fallback: NilExpr,
    ) -> Self {
        Self {
            kind: NilExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: NilExpr) -> Self {
        Self {
            kind: NilExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &NilExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{NilExpr, NilExprKind};
    use crate::plan::{BoolExpr, Expr, IntExpr, NilFunctionId, NilFunctionValue, Step};

    #[test]
    fn nil_expr_kind_accessors() {
        assert!(matches!(NilExpr::value().kind(), NilExprKind::Value));
        assert!(matches!(
            NilExpr::function_call(function_expr(), Vec::new()).kind(),
            NilExprKind::FunctionCall { .. }
        ));
        assert!(matches!(
            NilExpr::bool_case(BoolExpr::value(true), NilExpr::value(), NilExpr::value()).kind(),
            NilExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            NilExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), NilExpr::value())],
                NilExpr::value()
            )
            .kind(),
            NilExprKind::IntCase { .. }
        ));
        assert!(matches!(
            NilExpr::string_case(
                crate::plan::StringExpr::value("a".into()),
                vec![("a".into(), NilExpr::value())],
                NilExpr::value()
            )
            .kind(),
            NilExprKind::StringCase { .. }
        ));
        assert!(matches!(
            NilExpr::block(
                vec![Step::evaluate(Expr::nil(NilExpr::value()))],
                NilExpr::value(),
            )
            .kind(),
            NilExprKind::Block { .. }
        ));
    }

    fn function_expr() -> crate::plan::NilFunctionExpr {
        crate::plan::NilFunctionExpr::value(NilFunctionValue::new(NilFunctionId(0), Vec::new()))
    }
}
