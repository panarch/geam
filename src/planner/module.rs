use crate::plan::{FunctionId, ModulePlan};
use crate::planner::context::FunctionInfo;
use crate::planner::error::PlanError;
use crate::planner::function::{function_name, plan_function};
use ecow::EcoString;
use gleam_core::ast::TypedModule;
use std::collections::HashMap;

pub fn plan_module(module: TypedModule) -> Result<ModulePlan, PlanError> {
    reject_top_level("import", module.definitions.imports.len())?;
    reject_top_level("constant", module.definitions.constants.len())?;
    reject_top_level("custom type", module.definitions.custom_types.len())?;
    reject_top_level("type alias", module.definitions.type_aliases.len())?;

    let module_name = module.name;
    let functions = function_table(&module.definitions.functions)?;
    let main = main_function(&functions)?;
    let functions = module
        .definitions
        .functions
        .into_iter()
        .enumerate()
        .map(|(index, function)| {
            plan_function(FunctionId(index), &module_name, &functions, function)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ModulePlan {
        module: module_name,
        main,
        functions,
    })
}

fn function_table(
    functions: &[gleam_core::ast::TypedFunction],
) -> Result<HashMap<EcoString, FunctionInfo>, PlanError> {
    functions
        .iter()
        .enumerate()
        .map(|(index, function)| {
            Ok((
                function_name(function)?,
                FunctionInfo {
                    id: FunctionId(index),
                    arity: function.arguments.len(),
                },
            ))
        })
        .collect()
}

fn main_function(functions: &HashMap<EcoString, FunctionInfo>) -> Result<FunctionId, PlanError> {
    let main = functions
        .get("main")
        .ok_or_else(|| PlanError::UnsupportedFunction {
            name: "main".into(),
            reason: "main function is required",
        })?;

    if main.arity != 0 {
        return Err(PlanError::UnsupportedFunction {
            name: "main".into(),
            reason: "main must not take arguments",
        });
    }

    Ok(main.id)
}

fn reject_top_level(kind: &'static str, count: usize) -> Result<(), PlanError> {
    if count == 0 {
        Ok(())
    } else {
        Err(PlanError::UnsupportedTopLevel { kind })
    }
}

#[cfg(test)]
mod tests {
    use super::plan_module;
    use crate::planner::PlanError;
    use crate::planner::dsl::{function, int, module};
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use gleam_core::ast::TypedImport;

    #[test]
    fn plan_integer_return() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main")
            .function(function("main").return_(int(1)))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_constant_definition() {
        assert_eq!(
            expect_plan_error(
                r#"
const answer = 42

pub fn main() {
  answer
}
"#,
            ),
            PlanError::UnsupportedTopLevel { kind: "constant" },
        );
    }

    #[test]
    fn reject_profile_missing_main_function() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn other() {
  1
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: "main function is required",
            },
        );
    }

    #[test]
    fn reject_profile_main_function_with_arguments() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main(value: Int) {
  value
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: "main must not take arguments",
            },
        );
    }

    #[test]
    fn reject_profile_top_level_non_function_definitions() {
        assert_plan_error(
            r#"
const answer = 42

pub fn main() {
  answer
}
"#,
            PlanError::UnsupportedTopLevel { kind: "constant" },
        );

        assert_plan_error(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  1
}
"#,
            PlanError::UnsupportedTopLevel {
                kind: "custom type",
            },
        );

        assert_plan_error(
            r#"
pub type UserId =
  Int

pub fn main() {
  1
}
"#,
            PlanError::UnsupportedTopLevel { kind: "type alias" },
        );
    }

    #[test]
    fn reject_margin_import_definition() {
        let mut module = compile_minimal_module();
        module.definitions.imports.push(TypedImport {
            documentation: None,
            location: dummy_span(),
            module_location: dummy_span(),
            module: "gleam/io".into(),
            as_name: None,
            unqualified_values: Vec::new(),
            unqualified_types: Vec::new(),
            package: "gleam_stdlib".into(),
        });

        assert_eq!(
            plan_module(module),
            Err(PlanError::UnsupportedTopLevel { kind: "import" }),
        );
    }

    fn assert_plan_error(src: &str, expected: PlanError) {
        assert_eq!(expect_plan_error(src), expected);
    }
}
