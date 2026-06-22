use crate::plan::{FunctionId, ModulePlan};
use crate::planner::dsl::expression::{FunctionEntry, FunctionTable};
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
        let main = functions
            .get("main")
            .map(|function| function.id)
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
    let mut table = FunctionTable::new();
    for (index, function) in functions.iter().enumerate() {
        let return_type = function.return_type(&table);
        table.insert(
            function.name().clone(),
            FunctionEntry {
                id: FunctionId(index),
                return_type,
            },
        );
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{Expr, FunctionPlan, IntExpr, NilExpr};
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
                        return_: Expr::Int(IntExpr::Value(BigInt::from(1))),
                    },
                    FunctionPlan {
                        id: FunctionId(1),
                        name: "helper".into(),
                        params: vec![],
                        steps: vec![],
                        return_: Expr::Nil(NilExpr::Value),
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

    #[test]
    #[should_panic(expected = "function `main` in planner DSL must define a return expression")]
    fn module_build_panics_on_function_without_return() {
        module("main").function(function("main")).build();
    }
}
