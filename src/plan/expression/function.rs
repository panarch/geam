use super::{BoolExpr, IntExpr};
use crate::plan::{FunctionType, FunctionValue, Step};
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    type_: FunctionType,
    kind: FunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionExprKind {
    Value(FunctionValue),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<FunctionExpr>,
        false_: Box<FunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, FunctionExpr)>,
        fallback: Box<FunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<FunctionExpr>,
    },
}

impl FunctionExpr {
    pub(crate) fn value(value: FunctionValue) -> Self {
        Self {
            type_: value.type_(),
            kind: FunctionExprKind::Value(value),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: FunctionExpr, false_: FunctionExpr) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: FunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, FunctionExpr)>,
        fallback: FunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: FunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: FunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: FunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &FunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionExpr, FunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionValue, IntExpr, IntFunctionId, IntLocalId, LocalId,
        RuntimeFunctionId, Step,
    };

    #[test]
    fn function_expr_kind_accessors() {
        assert!(matches!(
            FunctionExpr::value(function_value()).kind(),
            FunctionExprKind::Value(_)
        ));
        assert!(matches!(
            FunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), FunctionExpr::value(function_value()))],
                FunctionExpr::value(function_value()),
            )
            .kind(),
            FunctionExprKind::IntCase { .. }
        ));
        assert!(matches!(
            FunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                FunctionExpr::value(function_value()),
            )
            .kind(),
            FunctionExprKind::Block { .. }
        ));
        assert!(matches!(
            FunctionExpr::bool_case(
                BoolExpr::value(true),
                FunctionExpr::value(function_value()),
                FunctionExpr::value(function_value()),
            )
            .kind(),
            FunctionExprKind::BoolCase { .. }
        ));
    }

    fn function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![LocalId::Int(IntLocalId(0))],
        )
    }
}
