use crate::plan::Step;
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
                let local = locals.define(name.clone());
                Step::Let { local, name, value }
            }
            Self::Evaluate(value) => Step::Evaluate(value.build(locals, functions)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{Expr, LocalId, Value};
    use crate::planner::dsl::expression::{FunctionTable, int, local, string};
    use num_bigint::BigInt;

    #[test]
    fn step_builder_let_shadow() {
        let mut locals = LocalTable::default();
        locals.define("x".into());

        let actual = StepBuilder::Let {
            name: "x".into(),
            value: local("x"),
        }
        .build(&mut locals, &FunctionTable::default());

        assert_eq!(
            actual,
            Step::Let {
                local: LocalId(1),
                name: "x".into(),
                value: Expr::LocalGet {
                    local: LocalId(0),
                    name: "x".into(),
                },
            }
        );
        assert_eq!(locals.lookup(&"x".into()), LocalId(1));
    }

    #[test]
    fn step_builder_evaluate() {
        let mut locals = LocalTable::default();

        let actual = StepBuilder::Evaluate(string("side effect"))
            .build(&mut locals, &FunctionTable::default());

        assert_eq!(
            actual,
            Step::Evaluate(Expr::Value(Value::String("side effect".into())))
        );
        assert_eq!(locals.define("next".into()), LocalId(0));
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
            Step::Let {
                local: LocalId(0),
                name: "x".into(),
                value: Expr::Value(Value::Int(BigInt::from(1))),
            }
        );
    }
}
