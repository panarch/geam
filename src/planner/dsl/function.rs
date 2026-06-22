use crate::plan::{FunctionId, FunctionPlan, Param, ValueType};
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
    params: Vec<(EcoString, ValueType)>,
    steps: Vec<StepBuilder>,
    return_: Option<ExprBuilder>,
}

impl FunctionBuilder {
    pub(in crate::planner) fn param(mut self, name: impl Into<EcoString>) -> Self {
        self.params.push((name.into(), ValueType::Int));
        self
    }

    pub(in crate::planner) fn param_string(mut self, name: impl Into<EcoString>) -> Self {
        self.params.push((name.into(), ValueType::String));
        self
    }

    pub(in crate::planner) fn param_bool(mut self, name: impl Into<EcoString>) -> Self {
        self.params.push((name.into(), ValueType::Bool));
        self
    }

    pub(in crate::planner) fn param_nil(mut self, name: impl Into<EcoString>) -> Self {
        self.params.push((name.into(), ValueType::Nil));
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

    pub(super) fn return_type(&self, functions: &FunctionTable) -> ValueType {
        let mut locals = LocalTable::default();
        for (name, type_) in &self.params {
            locals.define(name.clone(), *type_);
        }
        self.return_
            .as_ref()
            .unwrap_or_else(|| {
                panic!(
                    "function `{}` in planner DSL must define a return expression",
                    self.name
                )
            })
            .value_type(&locals, functions)
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
            .map(|(name, type_)| {
                let local = locals.define(name.clone(), type_);
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
    use crate::plan::{Expr, IntExpr, IntLocalId, LocalId, Step, StringExpr};
    use crate::planner::dsl::expression::{FunctionEntry, FunctionTable, call, int, local, string};
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
                    local: LocalId::Int(IntLocalId(0)),
                    name: "input".into(),
                }],
                steps: vec![
                    Step::LetInt {
                        local: IntLocalId(1),
                        name: "x".into(),
                        value: IntExpr::LocalGet {
                            local: IntLocalId(0),
                            name: "input".into(),
                        },
                    },
                    Step::Evaluate(Expr::String(StringExpr::Value("side effect".into()))),
                ],
                return_: Expr::Int(IntExpr::Call {
                    function: FunctionId(1),
                    args: vec![
                        Expr::Int(IntExpr::LocalGet {
                            local: IntLocalId(1),
                            name: "x".into(),
                        }),
                        Expr::String(StringExpr::Value("done".into())),
                    ],
                }),
            }
        );
    }

    #[test]
    fn function_builder_return() {
        let actual = function("main")
            .return_(int(1))
            .build(FunctionId(0), &FunctionTable::default());

        assert_eq!(actual.return_, Expr::Int(IntExpr::Value(BigInt::from(1))));
    }

    #[test]
    #[should_panic(expected = "function `main` in planner DSL must define a return expression")]
    fn function_builder_build_without_return() {
        function("main").build(FunctionId(0), &FunctionTable::default());
    }

    fn function_table() -> FunctionTable {
        FunctionTable::from([(
            "helper".into(),
            FunctionEntry {
                id: FunctionId(1),
                return_type: ValueType::Int,
            },
        )])
    }
}
