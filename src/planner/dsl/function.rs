use crate::plan::{FunctionId, FunctionPlan, Param};
use crate::planner::dsl::expression::{ExprBuilder, FunctionTable};
use crate::planner::dsl::locals::LocalTable;
use crate::planner::dsl::step::StepBuilder;
use ecow::EcoString;

pub(in crate::planner) fn function(name: impl Into<EcoString>) -> FunctionBuilder {
    FunctionBuilder {
        name: name.into(),
        params: Vec::new(),
        steps: Vec::new(),
        return_: None,
    }
}

#[derive(Debug, Clone)]
pub(in crate::planner) struct FunctionBuilder {
    name: EcoString,
    params: Vec<EcoString>,
    steps: Vec<StepBuilder>,
    return_: Option<ExprBuilder>,
}

impl FunctionBuilder {
    pub(in crate::planner) fn param(mut self, name: impl Into<EcoString>) -> Self {
        self.params.push(name.into());
        self
    }

    pub(in crate::planner) fn let_(
        mut self,
        name: impl Into<EcoString>,
        value: ExprBuilder,
    ) -> Self {
        self.steps.push(StepBuilder::Let {
            name: name.into(),
            value,
        });
        self
    }

    pub(in crate::planner) fn evaluate(mut self, value: ExprBuilder) -> Self {
        self.steps.push(StepBuilder::Evaluate(value));
        self
    }

    pub(in crate::planner) fn return_(mut self, value: ExprBuilder) -> Self {
        self.return_ = Some(value);
        self
    }

    pub(in crate::planner) fn name(&self) -> &EcoString {
        &self.name
    }

    pub(in crate::planner) fn build(
        self,
        id: FunctionId,
        functions: &FunctionTable,
    ) -> FunctionPlan {
        let mut locals = LocalTable::default();
        let params = self
            .params
            .into_iter()
            .map(|name| {
                let local = locals.define(name.clone());
                Param { local, name }
            })
            .collect();

        let steps = self
            .steps
            .into_iter()
            .map(|step| step.build(&mut locals, functions))
            .collect();

        let return_ = self
            .return_
            .unwrap_or_else(|| {
                panic!(
                    "function `{}` in planner DSL must define a return expression",
                    self.name
                )
            })
            .build(&locals, functions);

        FunctionPlan {
            id,
            name: self.name,
            params,
            steps,
            return_,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{Expr, LocalId, Step, Value};
    use crate::planner::dsl::expression::{FunctionTable, call, int, local, string};
    use num_bigint::BigInt;

    #[test]
    fn function_build() {
        let actual = function("main")
            .param("input")
            .let_("x", local("input"))
            .evaluate(string("side effect"))
            .return_(call("helper", [local("x"), string("done")]))
            .build(FunctionId(0), &function_table());

        assert_eq!(
            actual,
            FunctionPlan {
                id: FunctionId(0),
                name: "main".into(),
                params: vec![Param {
                    local: LocalId(0),
                    name: "input".into(),
                }],
                steps: vec![
                    Step::Let {
                        local: LocalId(1),
                        name: "x".into(),
                        value: Expr::LocalGet {
                            local: LocalId(0),
                            name: "input".into(),
                        },
                    },
                    Step::Evaluate(Expr::Value(Value::String("side effect".into()))),
                ],
                return_: Expr::Call {
                    function: FunctionId(1),
                    args: vec![
                        Expr::LocalGet {
                            local: LocalId(1),
                            name: "x".into(),
                        },
                        Expr::Value(Value::String("done".into())),
                    ],
                },
            }
        );
    }

    #[test]
    fn function_builder_return() {
        let actual = function("main")
            .return_(int(1))
            .build(FunctionId(0), &FunctionTable::default());

        assert_eq!(actual.return_, Expr::Value(Value::Int(BigInt::from(1))));
    }

    #[test]
    #[should_panic(expected = "function `main` in planner DSL must define a return expression")]
    fn function_builder_build_without_return() {
        function("main").build(FunctionId(0), &FunctionTable::default());
    }

    fn function_table() -> FunctionTable {
        FunctionTable::from([("helper".into(), FunctionId(1))])
    }
}
