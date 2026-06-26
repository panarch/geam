use super::expression::{BoolExpr, Expr, FunctionExpr, IntExpr, NilExpr, StringExpr};
use super::id::{BoolLocalId, IntLocalId, NilLocalId, StringLocalId};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    kind: StepKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StepKind {
    LetInt {
        local: IntLocalId,
        name: EcoString,
        value: IntExpr,
    },
    LetString {
        local: StringLocalId,
        name: EcoString,
        value: StringExpr,
    },
    LetBool {
        local: BoolLocalId,
        name: EcoString,
        value: BoolExpr,
    },
    LetNil {
        local: NilLocalId,
        name: EcoString,
        value: NilExpr,
    },
    LetFunction {
        name: EcoString,
        value: FunctionExpr,
    },
    Evaluate(Expr),
}

impl Step {
    pub(crate) fn let_int(local: IntLocalId, name: EcoString, value: IntExpr) -> Self {
        Self {
            kind: StepKind::LetInt { local, name, value },
        }
    }

    pub(crate) fn let_string(local: StringLocalId, name: EcoString, value: StringExpr) -> Self {
        Self {
            kind: StepKind::LetString { local, name, value },
        }
    }

    pub(crate) fn let_bool(local: BoolLocalId, name: EcoString, value: BoolExpr) -> Self {
        Self {
            kind: StepKind::LetBool { local, name, value },
        }
    }

    pub(crate) fn let_nil(local: NilLocalId, name: EcoString, value: NilExpr) -> Self {
        Self {
            kind: StepKind::LetNil { local, name, value },
        }
    }

    pub(crate) fn let_function(name: EcoString, value: FunctionExpr) -> Self {
        Self {
            kind: StepKind::LetFunction { name, value },
        }
    }

    pub(crate) fn evaluate(value: Expr) -> Self {
        Self {
            kind: StepKind::Evaluate(value),
        }
    }

    pub(crate) fn kind(&self) -> &StepKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{Step, StepKind};
    use crate::plan::{
        Expr, FunctionExpr, FunctionValue, IntExpr, IntFunctionId, IntLocalId, RuntimeFunctionId,
    };
    use num_bigint::BigInt;

    #[test]
    fn step_kind_accessors() {
        assert!(matches!(
            Step::let_int(IntLocalId(0), "x".into(), IntExpr::value(BigInt::from(1))).kind(),
            StepKind::LetInt { .. },
        ));
        assert!(matches!(
            Step::evaluate(Expr::int(IntExpr::value(BigInt::from(1)))).kind(),
            StepKind::Evaluate(_),
        ));
        assert!(matches!(
            Step::let_function(
                "f".into(),
                FunctionExpr::value(FunctionValue::new(
                    RuntimeFunctionId::Int(IntFunctionId(0)),
                    vec![crate::plan::LocalId::Int(IntLocalId(0))],
                )),
            )
            .kind(),
            StepKind::LetFunction { .. },
        ));
    }
}
