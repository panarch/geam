use crate::plan::{Expr, Step};
use crate::planner::dsl::expression::{ExprBuilder, FunctionTable};
use crate::planner::dsl::locals::LocalTable;
use ecow::EcoString;

#[derive(Debug, Clone)]
pub(super) enum StepBuilder {
    Let { name: EcoString, value: ExprBuilder },
    Evaluate(ExprBuilder),
}

impl StepBuilder {
    pub(super) fn build(self, locals: &mut LocalTable, functions: &FunctionTable) -> Step {
        match self {
            Self::Let { name, value } => {
                let value = value.build(locals, functions);
                match value {
                    Expr::Int(value) => Step::LetInt {
                        local: locals.define_int(name.clone()),
                        name,
                        value,
                    },
                    Expr::String(value) => Step::LetString {
                        local: locals.define_string(name.clone()),
                        name,
                        value,
                    },
                    Expr::Bool(value) => Step::LetBool {
                        local: locals.define_bool(name.clone()),
                        name,
                        value,
                    },
                    Expr::Nil(value) => Step::LetNil {
                        local: locals.define_nil(name.clone()),
                        name,
                        value,
                    },
                }
            }
            Self::Evaluate(value) => Step::Evaluate(value.build(locals, functions)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{
        BoolExpr, BoolLocalId, IntExpr, IntLocalId, NilExpr, NilLocalId, StringExpr, StringLocalId,
    };
    use crate::planner::dsl::expression::{FunctionTable, bool_, int, local, nil, string};
    use num_bigint::BigInt;

    #[test]
    fn step_builder_let_shadow() {
        let mut locals = LocalTable::default();
        locals.define_int("x".into());

        let actual = StepBuilder::Let {
            name: "x".into(),
            value: local("x"),
        }
        .build(&mut locals, &FunctionTable::default());

        assert_eq!(
            actual,
            Step::LetInt {
                local: IntLocalId(1),
                name: "x".into(),
                value: IntExpr::LocalGet {
                    local: IntLocalId(0),
                    name: "x".into(),
                },
            }
        );
        assert_eq!(
            locals.lookup(&"x".into()),
            crate::plan::LocalId::Int(IntLocalId(1))
        );
    }

    #[test]
    fn step_builder_evaluate() {
        let mut locals = LocalTable::default();

        let actual = StepBuilder::Evaluate(string("side effect"))
            .build(&mut locals, &FunctionTable::default());

        assert_eq!(
            actual,
            Step::Evaluate(Expr::String(StringExpr::Value("side effect".into())))
        );
        assert_eq!(locals.define_int("next".into()), IntLocalId(0));
    }

    #[test]
    fn step_builder_let() {
        let actual = StepBuilder::Let {
            name: "x".into(),
            value: int(1),
        }
        .build(&mut LocalTable::default(), &FunctionTable::default());

        assert_eq!(
            actual,
            Step::LetInt {
                local: IntLocalId(0),
                name: "x".into(),
                value: IntExpr::Value(BigInt::from(1)),
            }
        );
    }

    #[test]
    fn step_builder_typed_let_variants() {
        assert_eq!(
            StepBuilder::Let {
                name: "name".into(),
                value: string("geam"),
            }
            .build(&mut LocalTable::default(), &FunctionTable::default()),
            Step::LetString {
                local: StringLocalId(0),
                name: "name".into(),
                value: StringExpr::Value("geam".into()),
            },
        );
        assert_eq!(
            StepBuilder::Let {
                name: "flag".into(),
                value: bool_(true),
            }
            .build(&mut LocalTable::default(), &FunctionTable::default()),
            Step::LetBool {
                local: BoolLocalId(0),
                name: "flag".into(),
                value: BoolExpr::Value(true),
            },
        );
        assert_eq!(
            StepBuilder::Let {
                name: "nothing".into(),
                value: nil(),
            }
            .build(&mut LocalTable::default(), &FunctionTable::default()),
            Step::LetNil {
                local: NilLocalId(0),
                name: "nothing".into(),
                value: NilExpr::Value,
            },
        );
    }
}
