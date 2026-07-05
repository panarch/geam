use crate::plan::{
    ExecutionPlan, FunctionFunctionLocalId, FunctionId, FunctionType, IntFunctionLocalId,
    ParamBinding, ParamLocal, ValueType,
};
use crate::planner::context::{
    AnonymousFunctions, FunctionInfo, FunctionParam, FunctionRuntimeIds,
};
use crate::planner::error::{
    InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedArgumentReason,
    UnsupportedExpressionKind, UnsupportedFunctionReason, UnsupportedTopLevelKind,
};
use crate::planner::function::{function_name, plan_function};
use ecow::EcoString;
use gleam_core::ast::{ArgNames, Statement, TypedExpr, TypedFunction, TypedModule};
use gleam_core::type_::Type;
use std::collections::HashMap;

pub fn plan_module(module: TypedModule) -> Result<ExecutionPlan, PlanError> {
    let definitions = module.definitions;

    let imports = definitions.imports.len();
    let custom_types = definitions.custom_types.len();

    if imports != 0 {
        return Err(PlanError::UnsupportedTopLevel {
            kind: UnsupportedTopLevelKind::Import,
        });
    }
    if custom_types != 0 {
        return Err(PlanError::UnsupportedTopLevel {
            kind: UnsupportedTopLevelKind::CustomType,
        });
    }

    let module_name = module.name;
    let FunctionTable {
        by_name,
        main,
        functions_before_main,
        functions_after_main,
        mut anonymous_functions,
    } = function_table(&definitions.functions)?;
    let main = validate_main_function(main)?;
    let mut functions = Vec::new();

    for function in functions_before_main {
        let planned = plan_function(
            function.info,
            &module_name,
            &by_name,
            function.function,
            &mut anonymous_functions,
        )?;
        functions.push(planned);
    }

    let main = plan_function(
        main.info,
        &module_name,
        &by_name,
        main.function,
        &mut anonymous_functions,
    )?;

    for function in functions_after_main {
        let planned = plan_function(
            function.info,
            &module_name,
            &by_name,
            function.function,
            &mut anonymous_functions,
        )?;
        functions.push(planned);
    }
    let anonymous_functions = anonymous_functions.into_functions();

    Ok(ExecutionPlan::new_with_anonymous(
        module_name,
        main,
        functions,
        anonymous_functions,
    ))
}

struct FunctionTable {
    by_name: HashMap<EcoString, FunctionInfo>,
    main: FunctionToPlan,
    functions_before_main: Vec<FunctionToPlan>,
    functions_after_main: Vec<FunctionToPlan>,
    anonymous_functions: AnonymousFunctions,
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
        reject_todo_body(function)?;
        let return_type = function_return_type(name.clone(), &function.return_type)?;
        let params = function_params(name.clone(), &function.arguments, ParamLabelPolicy::Allow)?;
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

    let anonymous_functions = AnonymousFunctions::new(next_function_index, runtime_ids);

    Ok(FunctionTable {
        by_name,
        main,
        functions_before_main,
        functions_after_main,
        anonymous_functions,
    })
}

fn function_info(
    function_index: usize,
    seed: &FunctionSeed,
    runtime_ids: &mut FunctionRuntimeIds,
) -> FunctionInfo {
    let runtime_id = runtime_ids.next(&seed.return_type);
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

fn reject_todo_body(function: &TypedFunction) -> Result<(), PlanError> {
    if matches!(
        function.body.as_slice(),
        [Statement::Expression(TypedExpr::Todo { .. })]
    ) {
        return Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::Todo,
        });
    }

    Ok(())
}

pub(super) fn function_params(
    function_name: EcoString,
    arguments: &[gleam_core::ast::TypedArg],
    label_policy: ParamLabelPolicy,
) -> Result<Vec<FunctionParam>, PlanError> {
    let mut next_int = 0;
    let mut next_float = 0;
    let mut next_string = 0;
    let mut next_bool = 0;
    let mut next_nil = 0;
    let mut next_tuple = 0;
    let mut next_list = 0;
    let mut function_locals = FunctionParamLocalCounters::default();

    arguments
        .iter()
        .map(|argument| {
            let (binding, label) = match &argument.names {
                ArgNames::Named { name, .. } => (ParamBinding::Named(name.clone()), None),
                ArgNames::Discard { .. } => (ParamBinding::Discard, None),
                ArgNames::NamedLabelled { label, name, .. }
                    if label_policy == ParamLabelPolicy::Allow =>
                {
                    (ParamBinding::Named(name.clone()), Some(label.clone()))
                }
                ArgNames::LabelledDiscard { label, .. }
                    if label_policy == ParamLabelPolicy::Allow =>
                {
                    (ParamBinding::Discard, Some(label.clone()))
                }
                ArgNames::NamedLabelled { .. } | ArgNames::LabelledDiscard { .. } => {
                    return Err(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::FunctionShape {
                            name: function_name.clone(),
                            reason: InvalidFunctionShapeReason::LabelledArgument,
                        },
                    });
                }
            };

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
                ValueType::Float => {
                    let local = ParamLocal::float(crate::plan::FloatLocalId(next_float));
                    next_float += 1;
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
                ValueType::Tuple(type_) => {
                    let local =
                        ParamLocal::tuple(crate::plan::TupleLocalId(next_tuple), type_.clone());
                    next_tuple += 1;
                    local
                }
                ValueType::List(element_type) => {
                    let local = ParamLocal::list(
                        crate::plan::ListLocalId(next_list),
                        element_type.as_ref().clone(),
                    );
                    next_list += 1;
                    local
                }
                ValueType::Function(type_) => function_locals.next(type_),
            };
            Ok(FunctionParam {
                local,
                binding,
                label,
            })
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ParamLabelPolicy {
    Allow,
    Reject,
}

#[derive(Default)]
struct FunctionParamLocalCounters {
    next_int: usize,
    next_float: usize,
    next_string: usize,
    next_bool: usize,
    next_nil: usize,
    next_tuple: usize,
    next_list: usize,
    next_function: usize,
}

impl FunctionParamLocalCounters {
    fn next(&mut self, type_: &FunctionType) -> ParamLocal {
        match type_.return_() {
            ValueType::Int => {
                let local =
                    ParamLocal::int_function(IntFunctionLocalId(self.next_int), type_.clone());
                self.next_int += 1;
                local
            }
            ValueType::Float => {
                let local = ParamLocal::float_function(
                    crate::plan::FloatFunctionLocalId(self.next_float),
                    type_.clone(),
                );
                self.next_float += 1;
                local
            }
            ValueType::String => {
                let local = ParamLocal::string_function(
                    crate::plan::StringFunctionLocalId(self.next_string),
                    type_.clone(),
                );
                self.next_string += 1;
                local
            }
            ValueType::Bool => {
                let local = ParamLocal::bool_function(
                    crate::plan::BoolFunctionLocalId(self.next_bool),
                    type_.clone(),
                );
                self.next_bool += 1;
                local
            }
            ValueType::Nil => {
                let local = ParamLocal::nil_function(
                    crate::plan::NilFunctionLocalId(self.next_nil),
                    type_.clone(),
                );
                self.next_nil += 1;
                local
            }
            ValueType::Tuple(_) => {
                let local = ParamLocal::tuple_function(
                    crate::plan::TupleFunctionLocalId(self.next_tuple),
                    type_.clone(),
                );
                self.next_tuple += 1;
                local
            }
            ValueType::List(_) => {
                let local = ParamLocal::list_function(
                    crate::plan::ListFunctionLocalId(self.next_list),
                    type_.clone(),
                );
                self.next_list += 1;
                local
            }
            ValueType::Function(_) => {
                let local = ParamLocal::function_function(
                    FunctionFunctionLocalId(self.next_function),
                    type_.clone(),
                );
                self.next_function += 1;
                local
            }
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
        call_int_returning_function, function, function_ref, int, int_arg, int_return_tail_call,
        local_int, module,
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
    fn plan_type_alias_function_signature_as_underlying_type() {
        let actual = plan_module(compile(
            r#"
pub type UserId =
  Int

fn identity(value: UserId) -> UserId {
  value
}

pub fn main() {
  identity(41)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int_return_tail_call(1, [int_arg(0, int(41))])),
            [function("identity", local_int(0, "value")).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_constant_definition() {
        let actual = plan_module(compile(
            r#"
const answer = 42

pub fn main() {
  answer
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main", function("main", int(42)), []);

        assert_eq!(actual, expected);
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
    fn reject_profile_empty_source_body_is_todo() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Todo,
            },
        );
    }

    #[test]
    fn reject_profile_function_before_main() {
        assert_eq!(
            expect_plan_error(
                r#"
fn values() -> BitArray {
  <<>>
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

fn values() -> BitArray {
  <<>>
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

fn tuple_getter(callback: fn(#(Int)) -> #(String)) {
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
                function("tuple_getter", int(1)).param_tuple_function(
                    0,
                    "callback",
                    [ValueType::Tuple(vec![ValueType::Int])],
                    [ValueType::String],
                ),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_discard_function_argument_slots() {
        let actual = plan_module(compile(
            r#"
fn pick(_: Int, value: Int) {
  value
}

pub fn main() {
  pick(1, 42)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call(1, [int_arg(0, int(1)), int_arg(1, int(42))]),
            ),
            [function("pick", local_int(1, "value"))
                .discard_int_param(0)
                .param_int(1, "value")],
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

fn count(values: BitArray) {
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
    fn reject_profile_type_alias_resolved_to_unsupported_argument_type() {
        assert_eq!(
            expect_plan_error(
                r#"
pub type Bits =
  BitArray

pub fn main() {
  1
}

fn count(values: Bits) {
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
    fn plan_labelled_function_argument_uses_local_name() {
        let actual = plan_module(compile(
            r#"
fn identity(value local: Int) {
  local
}

pub fn main() {
  identity(value: 1)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int_return_tail_call(1, [int_arg(0, int(1))])),
            [function("identity", local_int(0, "local")).param_int(0, "local")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_top_level_non_function_definitions() {
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
