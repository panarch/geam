use super::{BoolExpr, CallArg, IntExpr, StringFunctionExpr};
use crate::plan::{Step, StringFunctionId, StringLocalId};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct StringExpr {
    kind: StringExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StringExprKind {
    Value(EcoString),
    LocalGet {
        local: StringLocalId,
        name: EcoString,
    },
    Call {
        function: StringFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<StringFunctionExpr>,
        args: Vec<CallArg>,
    },
    Concatenate {
        left: Box<StringExpr>,
        right: Box<StringExpr>,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<StringExpr>,
        false_: Box<StringExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, StringExpr)>,
        fallback: Box<StringExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<StringExpr>,
    },
}

impl StringExpr {
    pub(crate) fn value(value: EcoString) -> Self {
        Self {
            kind: StringExprKind::Value(value),
        }
    }

    pub(crate) fn local_get(local: StringLocalId, name: EcoString) -> Self {
        Self {
            kind: StringExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: StringFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: StringExprKind::Call { function, args },
        }
    }

    pub(crate) fn function_call(function: StringFunctionExpr, args: Vec<CallArg>) -> Self {
        Self {
            kind: StringExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        }
    }

    pub(crate) fn concatenate(left: StringExpr, right: StringExpr) -> Self {
        Self {
            kind: StringExprKind::Concatenate {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: StringExpr, false_: StringExpr) -> Self {
        Self {
            kind: StringExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, StringExpr)>,
        fallback: StringExpr,
    ) -> Self {
        Self {
            kind: StringExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: StringExpr) -> Self {
        Self {
            kind: StringExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &StringExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{StringExpr, StringExprKind};
    use crate::plan::{
        BoolExpr, Expr, IntExpr, LocalId, Step, StringFunctionId, StringFunctionValue,
        StringLocalId,
    };

    #[test]
    fn string_expr_kind_accessors() {
        assert!(matches!(
            StringExpr::value("geam".into()).kind(),
            StringExprKind::Value(_)
        ));
        assert!(matches!(
            StringExpr::bool_case(
                BoolExpr::value(true),
                StringExpr::value("yes".into()),
                StringExpr::value("no".into())
            )
            .kind(),
            StringExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            StringExpr::function_call(function_expr(), Vec::new()).kind(),
            StringExprKind::FunctionCall { .. }
        ));
        assert!(matches!(
            StringExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), StringExpr::value("one".into()))],
                StringExpr::value("other".into())
            )
            .kind(),
            StringExprKind::IntCase { .. }
        ));
        assert!(matches!(
            StringExpr::block(
                vec![Step::evaluate(Expr::string(StringExpr::value("a".into())))],
                StringExpr::value("b".into()),
            )
            .kind(),
            StringExprKind::Block { .. }
        ));
    }

    fn function_expr() -> crate::plan::StringFunctionExpr {
        crate::plan::StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            vec![LocalId::String(StringLocalId(0))],
        ))
    }
}
