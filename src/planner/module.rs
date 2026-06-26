use crate::plan::{ExecutionPlan, FunctionId, LocalId, RuntimeFunctionId, ValueType};
use crate::planner::context::{FunctionInfo, FunctionParam, FunctionRuntimeIds};
use crate::planner::error::{
    PlanError, UnsupportedArgumentReason, UnsupportedFunctionReason, UnsupportedTopLevelKind,
};
use crate::planner::function::{function_name, plan_function};
use ecow::EcoString;
use gleam_core::ast::{ArgNames, TypedFunction, TypedModule};
use gleam_core::type_::Type;
use std::collections::HashMap;

pub fn plan_module(module: TypedModule) -> Result<ExecutionPlan, PlanError> {
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
    let FunctionTable {
        by_name,
        main,
        functions_before_main,
        functions_after_main,
    } = function_table(&module.definitions.functions)?;
    let main = validate_main_function(main)?;
    let mut functions = Vec::new();

    for function in functions_before_main {
        let planned = plan_function(function.info, &module_name, &by_name, function.function)?;
        functions.push(planned);
    }

    let main = plan_function(main.info, &module_name, &by_name, main.function)?;

    for function in functions_after_main {
        let planned = plan_function(function.info, &module_name, &by_name, function.function)?;
        functions.push(planned);
    }

    Ok(ExecutionPlan::new(module_name, main, functions))
}

struct FunctionTable {
    by_name: HashMap<EcoString, FunctionInfo>,
    main: FunctionToPlan,
    functions_before_main: Vec<FunctionToPlan>,
    functions_after_main: Vec<FunctionToPlan>,
}

struct FunctionToPlan {
    info: FunctionInfo,
    function: TypedFunction,
}

fn function_table(
    functions: &[gleam_core::ast::TypedFunction],
) -> Result<FunctionTable, PlanError> {
    let mut seeds = Vec::new();

    for function in functions {
        let name = function_name(function)?;
        let return_type = FunctionReturnType::from_gleam(name.clone(), &function.return_type)?;
        let params = function_params(name.clone(), &function.arguments)?;
        seeds.push(FunctionSeed {
            name,
            function: function.clone(),
            arity: function.arguments.len(),
            params,
            return_type,
        });
    }

    let Some((main_index, main_seed)) = seeds
        .iter()
        .enumerate()
        .find(|(_, seed)| seed.name == "main")
        .map(|(index, seed)| (index, seed.clone()))
    else {
        return Err(PlanError::UnsupportedFunction {
            name: "main".into(),
            reason: UnsupportedFunctionReason::MissingMain,
        });
    };

    let mut runtime_ids = FunctionRuntimeIds::default();
    let main_info = function_info(0, &main_seed, &mut runtime_ids);
    let main = FunctionToPlan {
        info: main_info.clone(),
        function: main_seed.function,
    };
    let mut by_name = HashMap::from([(main_seed.name, main_info)]);
    let mut functions_before_main = Vec::new();
    let mut functions_after_main = Vec::new();
    let mut next_function_index = 1;

    for (source_index, seed) in seeds.into_iter().enumerate() {
        if source_index == main_index {
            continue;
        }

        let info = function_info(next_function_index, &seed, &mut runtime_ids);
        next_function_index += 1;
        by_name.insert(seed.name.clone(), info.clone());
        let function = FunctionToPlan {
            info,
            function: seed.function,
        };

        if source_index < main_index {
            functions_before_main.push(function);
        } else {
            functions_after_main.push(function);
        }
    }

    Ok(FunctionTable {
        by_name,
        main,
        functions_before_main,
        functions_after_main,
    })
}

fn function_info(
    function_index: usize,
    seed: &FunctionSeed,
    runtime_ids: &mut FunctionRuntimeIds,
) -> FunctionInfo {
    let return_type = seed.return_type.value_type();
    let runtime_id = seed.return_type.runtime_id(runtime_ids);
    FunctionInfo {
        id: FunctionId::new(function_index),
        runtime_id,
        arity: seed.arity,
        params: seed.params.clone(),
        type_: crate::plan::FunctionType::new(param_types(&seed.params), return_type.clone()),
        return_type,
    }
}

#[derive(Clone)]
struct FunctionSeed {
    name: EcoString,
    function: TypedFunction,
    arity: usize,
    params: Vec<FunctionParam>,
    return_type: FunctionReturnType,
}

#[derive(Clone, Copy)]
enum FunctionReturnType {
    Int,
    String,
    Bool,
    Nil,
}

impl FunctionReturnType {
    fn from_gleam(name: EcoString, type_: &Type) -> Result<Self, PlanError> {
        if type_.is_int() {
            Ok(Self::Int)
        } else if type_.is_string() {
            Ok(Self::String)
        } else if type_.is_bool() {
            Ok(Self::Bool)
        } else if type_.is_nil() {
            Ok(Self::Nil)
        } else {
            Err(PlanError::UnsupportedFunction {
                name,
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            })
        }
    }

    fn value_type(self) -> ValueType {
        match self {
            Self::Int => ValueType::Int,
            Self::String => ValueType::String,
            Self::Bool => ValueType::Bool,
            Self::Nil => ValueType::Nil,
        }
    }

    fn runtime_id(self, runtime_ids: &mut FunctionRuntimeIds) -> RuntimeFunctionId {
        match self {
            Self::Int => runtime_ids.next_int(),
            Self::String => runtime_ids.next_string(),
            Self::Bool => runtime_ids.next_bool(),
            Self::Nil => runtime_ids.next_nil(),
        }
    }
}

fn function_params(
    function_name: EcoString,
    arguments: &[gleam_core::ast::TypedArg],
) -> Result<Vec<FunctionParam>, PlanError> {
    let mut next_int = 0;
    let mut next_string = 0;
    let mut next_bool = 0;
    let mut next_nil = 0;

    arguments
        .iter()
        .map(|argument| {
            let Some(name) = argument.names.get_variable_name().cloned() else {
                return Err(PlanError::UnsupportedArgument {
                    function: function_name.clone(),
                    reason: UnsupportedArgumentReason::Discard,
                });
            };

            if !matches!(argument.names, ArgNames::Named { .. }) {
                return Err(PlanError::UnsupportedArgument {
                    function: function_name.clone(),
                    reason: UnsupportedArgumentReason::Labelled,
                });
            }

            let Some(type_) = ValueType::from_gleam(&argument.type_) else {
                return Err(PlanError::UnsupportedArgument {
                    function: function_name.clone(),
                    reason: UnsupportedArgumentReason::UnsupportedType,
                });
            };
            let local = match &type_ {
                ValueType::Int => {
                    let local = LocalId::Int(crate::plan::IntLocalId(next_int));
                    next_int += 1;
                    local
                }
                ValueType::String => {
                    let local = LocalId::String(crate::plan::StringLocalId(next_string));
                    next_string += 1;
                    local
                }
                ValueType::Bool => {
                    let local = LocalId::Bool(crate::plan::BoolLocalId(next_bool));
                    next_bool += 1;
                    local
                }
                ValueType::Nil => {
                    let local = LocalId::Nil(crate::plan::NilLocalId(next_nil));
                    next_nil += 1;
                    local
                }
                ValueType::Function(_) => {
                    return Err(PlanError::UnsupportedArgument {
                        function: function_name.clone(),
                        reason: UnsupportedArgumentReason::UnsupportedType,
                    });
                }
            };
            Ok(FunctionParam { local, name, type_ })
        })
        .collect()
}

fn param_types(params: &[FunctionParam]) -> Vec<ValueType> {
    params.iter().map(|param| param.type_.clone()).collect()
}

fn validate_main_function(main: FunctionToPlan) -> Result<FunctionToPlan, PlanError> {
    if main.info.arity != 0 {
        return Err(PlanError::UnsupportedFunction {
            name: "main".into(),
            reason: UnsupportedFunctionReason::MainWithArguments,
        });
    }

    Ok(main)
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
    use crate::planner::{
        PlanError, UnsupportedArgumentReason, UnsupportedFunctionReason, UnsupportedTopLevelKind,
    };

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
        let expected = module("main", function("main", int(1)), []);

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
    fn reject_profile_function_before_main() {
        assert_eq!(
            expect_plan_error(
                r#"
fn values() {
  [1]
}

pub fn main() {
  1
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "values".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            },
        );
    }

    #[test]
    fn reject_profile_function_after_main() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  1
}

fn values() {
  [1]
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "values".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            },
        );
    }

    #[test]
    fn reject_profile_function_return_type_after_main_reference() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  get
  1
}

fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "get".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            },
        );
    }

    #[test]
    fn reject_profile_function_return_type_after_main_call() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  get()
  1
}

fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "get".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            },
        );
    }

    #[test]
    fn reject_profile_function_argument_type() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  1
}

fn count(values: List(Int)) {
  1
}
"#,
            ),
            PlanError::UnsupportedArgument {
                function: "count".into(),
                reason: UnsupportedArgumentReason::UnsupportedType,
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
