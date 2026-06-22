use crate::plan::ModulePlan;
use crate::planner::dsl::function::FunctionBuilder;
use ecow::EcoString;

pub(in crate::planner) fn module(name: impl Into<EcoString>) -> ModuleBuilder {
    ModuleBuilder {
        name: name.into(),
        functions: Vec::new(),
    }
}

#[derive(Debug, Clone)]
pub(in crate::planner) struct ModuleBuilder {
    name: EcoString,
    functions: Vec<FunctionBuilder>,
}

impl ModuleBuilder {
    pub(in crate::planner) fn function(mut self, function: FunctionBuilder) -> Self {
        self.functions.push(function);
        self
    }

    pub(in crate::planner) fn build(self) -> ModulePlan {
        ModulePlan {
            module: self.name,
            functions: self
                .functions
                .into_iter()
                .map(FunctionBuilder::build)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{Expr, FunctionPlan, Value};
    use crate::planner::dsl::expression::{int, nil};
    use crate::planner::dsl::function::function;
    use num_bigint::BigInt;

    #[test]
    fn module_build() {
        let actual = module("main")
            .function(function("main").return_(int(1)))
            .function(function("helper").return_(nil()))
            .build();

        assert_eq!(
            actual,
            ModulePlan {
                module: "main".into(),
                functions: vec![
                    FunctionPlan {
                        name: "main".into(),
                        params: vec![],
                        steps: vec![],
                        return_: Expr::Value(Value::Int(BigInt::from(1))),
                    },
                    FunctionPlan {
                        name: "helper".into(),
                        params: vec![],
                        steps: vec![],
                        return_: Expr::Value(Value::Nil),
                    },
                ],
            }
        );
    }
}
