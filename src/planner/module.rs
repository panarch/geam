use crate::plan::{FunctionId, ModulePlan};
use crate::planner::context::FunctionInfo;
use crate::planner::error::{PlanError, UnsupportedFunctionReason, UnsupportedTopLevelKind};
use crate::planner::function::{function_name, plan_function};
use ecow::EcoString;
use gleam_core::ast::TypedModule;
use std::collections::HashMap;

pub fn plan_module(module: TypedModule) -> Result<ModulePlan, PlanError> {
    reject_top_level(
        UnsupportedTopLevelKind::Import,
        module.definitions.imports.len(),
    )?;
    reject_top_level(
        UnsupportedTopLevelKind::Constant,
        module.definitions.constants.len(),
    )?;
    reject_top_level(
        UnsupportedTopLevelKind::CustomType,
        module.definitions.custom_types.len(),
    )?;
    reject_top_level(
        UnsupportedTopLevelKind::TypeAlias,
        module.definitions.type_aliases.len(),
    )?;

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
            reason: UnsupportedFunctionReason::MissingMain,
        })?;

    if main.arity != 0 {
        return Err(PlanError::UnsupportedFunction {
            name: "main".into(),
            reason: UnsupportedFunctionReason::MainWithArguments,
        });
    }

    Ok(main.id)
}

fn reject_top_level(kind: UnsupportedTopLevelKind, count: usize) -> Result<(), PlanError> {
    if count == 0 {
        Ok(())
    } else {
        Err(PlanError::UnsupportedTopLevel { kind })
    }
}

#[cfg(test)]
mod tests {
    use super::plan_module;
    use crate::planner::dsl::{function, int, module};
    use crate::planner::support::{compile, expect_plan_error};
    use crate::planner::{PlanError, UnsupportedFunctionReason, UnsupportedTopLevelKind};

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
            PlanError::UnsupportedTopLevel {
                kind: UnsupportedTopLevelKind::Constant,
            },
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
                reason: UnsupportedFunctionReason::MissingMain,
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
                reason: UnsupportedFunctionReason::MainWithArguments,
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
            PlanError::UnsupportedTopLevel {
                kind: UnsupportedTopLevelKind::Constant,
            },
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
                kind: UnsupportedTopLevelKind::CustomType,
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
            PlanError::UnsupportedTopLevel {
                kind: UnsupportedTopLevelKind::TypeAlias,
            },
        );
    }

    #[test]
    fn reject_profile_import_definition() {
        assert_eq!(
            expect_plan_error(
                r#"
import gleam

pub fn main() {
  1
}
"#,
            ),
            PlanError::UnsupportedTopLevel {
                kind: UnsupportedTopLevelKind::Import,
            },
        );
    }

    fn assert_plan_error(src: &str, expected: PlanError) {
        assert_eq!(expect_plan_error(src), expected);
    }
}
