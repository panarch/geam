use crate::plan::{
    ExecutionPlan, FunctionFunctionLocalId, FunctionId, FunctionType, IntFunctionLocalId,
    ParamLocal, RuntimeFunctionId, ValueType,
};
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
    let definitions = module.definitions;

    let imports = definitions.imports.len();
    let constants = definitions.constants.len();
    let custom_types = definitions.custom_types.len();
    let type_aliases = definitions.type_aliases.len();

    if imports != 0 {
        return Err(PlanError::UnsupportedTopLevel {
            kind: UnsupportedTopLevelKind::Import,
        });
    }
    if constants != 0 {
        return Err(PlanError::UnsupportedTopLevel {
            kind: UnsupportedTopLevelKind::Constant,
        });
    }
    if custom_types != 0 {
        return Err(PlanError::UnsupportedTopLevel {
            kind: UnsupportedTopLevelKind::CustomType,
        });
    }
    if type_aliases != 0 {
        return Err(PlanError::UnsupportedTopLevel {
            kind: UnsupportedTopLevelKind::TypeAlias,
        });
    }

    let module_name = module.name;
    let FunctionTable {
        by_name,
        main,
        functions_before_main,
        functions_after_main,
    } = function_table(&definitions.functions)?;
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
        let return_type = function_return_type(name.clone(), &function.return_type)?;
        let params = function_params(name.clone(), &function.arguments)?;
        seeds.push(FunctionSeed {
            name,
            function: function.clone(),
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
    let runtime_id = runtime_id(&seed.return_type, runtime_ids);
    FunctionInfo {
        id: FunctionId::new(function_index),
        runtime_id,
        return_type: seed.return_type.clone(),
        params: seed.params.clone(),
    }
}

#[derive(Clone)]
struct FunctionSeed {
    name: EcoString,
    function: TypedFunction,
    params: Vec<FunctionParam>,
    return_type: ValueType,
}

fn function_return_type(name: EcoString, type_: &Type) -> Result<ValueType, PlanError> {
    ValueType::from_gleam(type_).ok_or(PlanError::UnsupportedFunction {
        name,
        reason: UnsupportedFunctionReason::UnsupportedReturnType,
    })
}

fn runtime_id(return_type: &ValueType, runtime_ids: &mut FunctionRuntimeIds) -> RuntimeFunctionId {
    match return_type {
        ValueType::Int => runtime_ids.next_int(),
        ValueType::String => runtime_ids.next_string(),
        ValueType::Bool => runtime_ids.next_bool(),
        ValueType::Nil => runtime_ids.next_nil(),
        ValueType::Function(return_type) => runtime_ids.next_function(return_type.as_ref().clone()),
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
    let mut next_int_function = 0;
    let mut next_string_function = 0;
    let mut next_bool_function = 0;
    let mut next_nil_function = 0;
    let mut next_function_function = 0;

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
                    let local = ParamLocal::int(crate::plan::IntLocalId(next_int));
                    next_int += 1;
                    local
                }
                ValueType::String => {
                    let local = ParamLocal::string(crate::plan::StringLocalId(next_string));
                    next_string += 1;
                    local
                }
                ValueType::Bool => {
                    let local = ParamLocal::bool(crate::plan::BoolLocalId(next_bool));
                    next_bool += 1;
                    local
                }
                ValueType::Nil => {
                    let local = ParamLocal::nil(crate::plan::NilLocalId(next_nil));
                    next_nil += 1;
                    local
                }
                ValueType::Function(type_) => function_param_local(
                    type_,
                    &mut next_int_function,
                    &mut next_string_function,
                    &mut next_bool_function,
                    &mut next_nil_function,
                    &mut next_function_function,
                ),
            };
            Ok(FunctionParam { local, name })
        })
        .collect()
}

fn function_param_local(
    type_: &FunctionType,
    next_int_function: &mut usize,
    next_string_function: &mut usize,
    next_bool_function: &mut usize,
    next_nil_function: &mut usize,
    next_function_function: &mut usize,
) -> ParamLocal {
    match type_.return_() {
        ValueType::Int => {
            let local =
                ParamLocal::int_function(IntFunctionLocalId(*next_int_function), type_.clone());
            *next_int_function += 1;
            local
        }
        ValueType::String => {
            let local = ParamLocal::string_function(
                crate::plan::StringFunctionLocalId(*next_string_function),
                type_.clone(),
            );
            *next_string_function += 1;
            local
        }
        ValueType::Bool => {
            let local = ParamLocal::bool_function(
                crate::plan::BoolFunctionLocalId(*next_bool_function),
                type_.clone(),
            );
            *next_bool_function += 1;
            local
        }
        ValueType::Nil => {
            let local = ParamLocal::nil_function(
                crate::plan::NilFunctionLocalId(*next_nil_function),
                type_.clone(),
            );
            *next_nil_function += 1;
            local
        }
        ValueType::Function(_) => {
            let local = ParamLocal::function_function(
                FunctionFunctionLocalId(*next_function_function),
                type_.clone(),
            );
            *next_function_function += 1;
            local
        }
    }
}

fn validate_main_function(main: FunctionToPlan) -> Result<FunctionToPlan, PlanError> {
    if main.info.arity() != 0 {
        return Err(PlanError::UnsupportedFunction {
            name: "main".into(),
            reason: UnsupportedFunctionReason::MainWithArguments,
        });
    }

    Ok(main)
}

#[cfg(test)]
mod tests {
    use super::plan_module;
    use crate::plan::{
        FunctionFunctionId, FunctionType, IntFunctionFunctionId, IntFunctionId, IntLocalId,
        LocalId, ParamLocal, RuntimeFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        call_int_returning_function, function, function_ref, int, local_int, module,
    };
    use crate::planner::support::{compile, expect_plan_error};
    use crate::planner::{
        PlanError, UnsupportedArgumentReason, UnsupportedExpressionKind, UnsupportedFunctionReason,
        UnsupportedTopLevelKind,
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
    fn reject_profile_function_body_before_main() {
        assert_eq!(
            expect_plan_error(
                r#"
fn helper() -> Int {
  panic
}

pub fn main() {
  1
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Panic,
            },
        );
    }

    #[test]
    fn reject_profile_function_body_after_main() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  1
}

fn helper() -> Int {
  panic
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Panic,
            },
        );
    }

    #[test]
    fn plan_function_returning_function_after_main_reference() {
        let actual = plan_module(compile(
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
        ))
        .expect("source should plan");
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let expected = module(
            "main",
            function("main", int(1)).evaluate(function_ref(
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: returned_function_type.clone(),
                },
                Vec::<ParamLocal>::new(),
            )),
            [
                function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value"),
                function(
                    "get",
                    function_ref(
                        RuntimeFunctionId::Int(IntFunctionId(1)),
                        [LocalId::Int(IntLocalId(0))],
                    ),
                ),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_returning_function_after_main_call() {
        let actual = plan_module(compile(
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
        ))
        .expect("source should plan");
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let expected = module(
            "main",
            function("main", int(1)).evaluate(call_int_returning_function(
                0,
                [],
                returned_function_type,
            )),
            [
                function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value"),
                function(
                    "get",
                    function_ref(
                        RuntimeFunctionId::Int(IntFunctionId(1)),
                        [LocalId::Int(IntLocalId(0))],
                    ),
                ),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_argument_with_function_argument_type() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1
}

fn higher(callback: fn(fn(Int) -> Int) -> Int) {
  1
}

fn getter(callback: fn() -> fn(Int) -> Int) {
  1
}
"#,
        ))
        .expect("source should plan");
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let expected = module(
            "main",
            function("main", int(1)),
            [
                function("higher", int(1)).param_int_function(
                    0,
                    "callback",
                    [ValueType::Function(Box::new(
                        returned_function_type.clone(),
                    ))],
                ),
                function("getter", int(1)).param_function_function(
                    0,
                    "callback",
                    FunctionType::new(
                        Vec::new(),
                        ValueType::Function(Box::new(returned_function_type)),
                    ),
                ),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_unsupported_function_argument_type() {
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
    fn reject_profile_function_argument_name_shape() {
        assert_eq!(
            expect_plan_error(
                r#"
fn helper(_: Int) {
  1
}

pub fn main() {
  helper(1)
}
"#,
            ),
            PlanError::UnsupportedArgument {
                function: "helper".into(),
                reason: UnsupportedArgumentReason::Discard,
            },
        );

        assert_eq!(
            expect_plan_error(
                r#"
fn identity(value value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
            ),
            PlanError::UnsupportedArgument {
                function: "identity".into(),
                reason: UnsupportedArgumentReason::Labelled,
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
