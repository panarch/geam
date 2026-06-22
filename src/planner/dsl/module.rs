use crate::plan::{FunctionId, ModulePlan};
use crate::planner::dsl::expression::FunctionTable;
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
        let functions = function_table(&self.functions);
        let main = *functions
            .get("main")
            .expect("planner DSL module must define a main function");

        ModulePlan {
            module: self.name,
            main,
            functions: self
                .functions
                .into_iter()
                .enumerate()
                .map(|(index, function)| function.build(FunctionId(index), &functions))
                .collect(),
        }
    }
}

fn function_table(functions: &[FunctionBuilder]) -> FunctionTable {
    functions
        .iter()
        .enumerate()
        .map(|(index, function)| (function.name().clone(), FunctionId(index)))
        .collect()
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
                main: FunctionId(0),
                functions: vec![
                    FunctionPlan {
                        id: FunctionId(0),
                        name: "main".into(),
                        params: vec![],
                        steps: vec![],
                        return_: Expr::Value(Value::Int(BigInt::from(1))),
                    },
                    FunctionPlan {
                        id: FunctionId(1),
                        name: "helper".into(),
                        params: vec![],
                        steps: vec![],
                        return_: Expr::Value(Value::Nil),
                    },
                ],
            }
        );
    }

    #[test]
    #[should_panic(expected = "planner DSL module must define a main function")]
    fn module_build_without_main_function() {
        module("main")
            .function(function("helper").return_(nil()))
            .build();
    }
}
