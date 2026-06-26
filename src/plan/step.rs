use super::expression::{
    BoolExpr, BoolFunctionExpr, Expr, IntExpr, IntFunctionExpr, NilExpr, NilFunctionExpr,
    StringExpr, StringFunctionExpr,
};
use super::id::{
    BoolFunctionLocalId, BoolLocalId, IntFunctionLocalId, IntLocalId, NilFunctionLocalId,
    NilLocalId, StringFunctionLocalId, StringLocalId,
};
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
    LetIntFunction {
        local: IntFunctionLocalId,
        name: EcoString,
        value: IntFunctionExpr,
    },
    LetStringFunction {
        local: StringFunctionLocalId,
        name: EcoString,
        value: StringFunctionExpr,
    },
    LetBoolFunction {
        local: BoolFunctionLocalId,
        name: EcoString,
        value: BoolFunctionExpr,
    },
    LetNilFunction {
        local: NilFunctionLocalId,
        name: EcoString,
        value: NilFunctionExpr,
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

    pub(crate) fn let_int_function(
        local: IntFunctionLocalId,
        name: EcoString,
        value: IntFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetIntFunction { local, name, value },
        }
    }

    pub(crate) fn let_string_function(
        local: StringFunctionLocalId,
        name: EcoString,
        value: StringFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetStringFunction { local, name, value },
        }
    }

    pub(crate) fn let_bool_function(
        local: BoolFunctionLocalId,
        name: EcoString,
        value: BoolFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetBoolFunction { local, name, value },
        }
    }

    pub(crate) fn let_nil_function(
        local: NilFunctionLocalId,
        name: EcoString,
        value: NilFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetNilFunction { local, name, value },
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
        Expr, IntExpr, IntFunctionId, IntFunctionLocalId, IntFunctionValue, IntLocalId, ParamLocal,
    };
    use num_bigint::BigInt;

    #[test]
    fn step_kind_accessors() {
        assert!(matches!(
            Step::let_int(IntLocalId(0), "x".into(), IntExpr::value(BigInt::from(1))).kind(),
            StepKind::LetInt { .. },
        ));
        assert!(matches!(
            Step::let_int_function(IntFunctionLocalId(0), "f".into(), function_expr()).kind(),
            StepKind::LetIntFunction { .. },
        ));
        assert!(matches!(
            Step::evaluate(Expr::int(IntExpr::value(BigInt::from(1)))).kind(),
            StepKind::Evaluate(_),
        ));
    }

    fn function_expr() -> crate::plan::IntFunctionExpr {
        crate::plan::IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }
}
