use crate::plan::{
    FunctionFunctionLocalId, FunctionId, FunctionType, IntFunctionLocalId, ModulePlan,
    ParamBinding, ParamLocal, SourceContext, ValueType,
};
use crate::planner::context::{
    AnonymousFunctions, FunctionInfo, FunctionParam, FunctionRuntimeIds,
};
use crate::planner::error::{
    InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedArgumentReason,
    UnsupportedFunctionReason, UnsupportedTopLevelKind,
};
use crate::planner::function::{function_name, plan_function};
use ecow::EcoString;
use gleam_core::ast::{ArgNames, Statement, TypedExpr, TypedFunction, TypedModule};
use gleam_core::type_::{Type, TypeVar};
use std::collections::HashMap;
use std::ops::Deref;

pub fn plan_module(module: TypedModule) -> Result<ModulePlan, PlanError> {
    plan_module_inner(module)
}

pub fn plan_module_with_source(
    module: TypedModule,
    source_context: SourceContext,
) -> Result<ModulePlan, PlanError> {
    plan_module_inner(module).map(|plan| plan.with_source_context(source_context))
}

fn plan_module_inner(module: TypedModule) -> Result<ModulePlan, PlanError> {
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

    Ok(ModulePlan::new(module_name, main, functions).with_anonymous_functions(anonymous_functions))
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
        let return_type =
            function_return_type(name.clone(), &function.return_type, &function.body)?;
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

fn function_return_type(
    name: EcoString,
    type_: &Type,
    body: &[gleam_core::ast::TypedStatement],
) -> Result<ValueType, PlanError> {
    if let Some(return_type) = ValueType::from_gleam(type_) {
        return Ok(return_type);
    }

    if is_inferred_return_type(type_)
        && let Some(return_type) = source_stop_return_type(body)
    {
        return Ok(return_type);
    }

    Err(PlanError::UnsupportedFunction {
        name,
        reason: UnsupportedFunctionReason::UnsupportedReturnType,
    })
}

fn is_inferred_return_type(type_: &Type) -> bool {
    let Type::Var { type_ } = type_ else {
        return false;
    };

    match type_.borrow().deref() {
        TypeVar::Link { type_ } => is_inferred_return_type(type_.as_ref()),
        TypeVar::Unbound { .. } => true,
        TypeVar::Generic { .. } => false,
    }
}

fn source_stop_return_type(body: &[gleam_core::ast::TypedStatement]) -> Option<ValueType> {
    matches!(
        body.last(),
        Some(Statement::Expression(expression)) if is_source_stop_expr(expression)
    )
    .then_some(ValueType::Nil)
}

fn is_source_stop_expr(expression: &TypedExpr) -> bool {
    match expression {
        TypedExpr::Panic { .. } | TypedExpr::Todo { .. } => true,
        TypedExpr::Block { statements, .. } => {
            let Statement::Expression(expression) = statements.last() else {
                return false;
            };
            is_source_stop_expr(expression)
        }
        _ => false,
    }
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
    let mut next_int_list = 0;
    let mut next_string_list = 0;
    let mut next_float_list = 0;
    let mut next_bool_list = 0;
    let mut next_nil_list = 0;
    let mut next_tuple_list = 0;
    let mut next_list_list = 0;
    let mut next_function_list = 0;
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
                    let local = match element_type.as_ref() {
                        ValueType::Int => {
                            let local = crate::plan::ListLocal::int(crate::plan::IntListLocalId(
                                next_int_list,
                            ));
                            next_int_list += 1;
                            local
                        }
                        ValueType::String => {
                            let local = crate::plan::ListLocal::string(
                                crate::plan::StringListLocalId(next_string_list),
                            );
                            next_string_list += 1;
                            local
                        }
                        ValueType::Float => {
                            let local = crate::plan::ListLocal::float(
                                crate::plan::FloatListLocalId(next_float_list),
                            );
                            next_float_list += 1;
                            local
                        }
                        ValueType::Bool => {
                            let local = crate::plan::ListLocal::bool(crate::plan::BoolListLocalId(
                                next_bool_list,
                            ));
                            next_bool_list += 1;
                            local
                        }
                        ValueType::Nil => {
                            let local = crate::plan::ListLocal::nil(crate::plan::NilListLocalId(
                                next_nil_list,
                            ));
                            next_nil_list += 1;
                            local
                        }
                        ValueType::Tuple(item_type) => {
                            let local = crate::plan::ListLocal::tuple(
                                crate::plan::TupleListLocalId(next_tuple_list),
                                item_type.clone(),
                            );
                            next_tuple_list += 1;
                            local
                        }
                        ValueType::List(item_type) => {
                            let local = crate::plan::ListLocal::list(
                                crate::plan::ListListLocalId(next_list_list),
                                *item_type.clone(),
                            );
                            next_list_list += 1;
                            local
                        }
                        ValueType::Function(item_type) => {
                            let local = crate::plan::ListLocal::function(
                                crate::plan::FunctionListLocalId(next_function_list),
                                *item_type.clone(),
                            );
                            next_function_list += 1;
                            local
                        }
                    };
                    ParamLocal::list(local)
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
            ValueType::List(item_type) => {
                let local =
                    ParamLocal::list_function(crate::plan::ListFunctionLocal::from_item_type(
                        self.next_list,
                        type_.clone(),
                        item_type.as_ref().clone(),
                    ));
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
        BoolListLocalId, FloatListLocalId, FunctionFunctionId, FunctionListLocalId, FunctionType,
        IntFunctionFunctionId, IntFunctionId, IntListLocalId, IntLocalId, ListListLocalId,
        ListLocal, LocalId, NilExpr, NilFunctionId, NilListLocalId, PanicExpr, PanicSite, Param,
        ParamLocal, RuntimeFunctionId, SourceSpan, StringListLocalId, TupleListLocalId, ValueType,
    };
    use crate::planner::dsl::{
        call_int, call_int_returning_function, function, function_ref, int, int_arg,
        int_return_tail_call, local_int, module,
    };
    use crate::planner::support::{compile, expect_plan_error};
    use crate::planner::{
        InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedArgumentReason,
        UnsupportedExpressionKind, UnsupportedFunctionReason, UnsupportedTopLevelKind,
    };
    use gleam_core::type_;

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
    fn plan_functions_before_and_after_main() {
        let actual = plan_module(compile(
            r#"
fn before() {
  1
}

pub fn main() {
  before() + after()
}

fn after() {
  2
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int(1, Vec::new()).add_int(call_int(2, Vec::new())),
            ),
            [function("before", int(1)), function("after", int(2))],
        );

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
    fn reject_margin_function_table_name_shape() {
        let mut module = compile(
            r#"
pub fn main() {
  1
}
"#,
        );
        module.definitions.functions[0].name = None;

        assert_eq!(
            super::function_table(&module.definitions.functions).err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous>".into(),
                    reason: InvalidFunctionShapeReason::Anonymous,
                },
            }),
        );
    }

    #[test]
    fn plan_empty_source_body_as_nil_generated_todo() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
}
"#,
        ))
        .expect("source should plan");
        assert_eq!(
            actual.main_function().return_(),
            &crate::plan::ReturnExpr::nil(
                NilFunctionId(0),
                NilExpr::panic(PanicExpr::empty_function_at(PanicSite::new(
                    "main".into(),
                    "main".into(),
                    SourceSpan::new(1, 14),
                ))),
            ),
        );
    }

    #[test]
    fn reject_profile_unsupported_return_type_for_non_source_stop_final_shapes() {
        assert_eq!(super::source_stop_return_type(&[]), None);
        assert_eq!(
            super::function_return_type("values".into(), type_::bit_array().as_ref(), &[]),
            Err(PlanError::UnsupportedFunction {
                name: "values".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            }),
        );

        let final_expression = compile(
            r#"
pub fn main() {
  1
}
"#,
        );
        assert_eq!(
            super::source_stop_return_type(&final_expression.definitions.functions[0].body),
            None,
        );

        let final_assignment = compile(
            r#"
pub fn main() {
  let value = 1
}
"#,
        );
        assert_eq!(
            super::source_stop_return_type(&final_assignment.definitions.functions[0].body),
            None,
        );

        let block_with_final_assignment = compile(
            r#"
pub fn main() {
  {
    let value = 1
  }
}
"#,
        );
        assert_eq!(
            super::source_stop_return_type(
                &block_with_final_assignment.definitions.functions[0].body
            ),
            None,
        );

        assert_eq!(
            super::function_return_type(
                "main".into(),
                type_::bit_array().as_ref(),
                &block_with_final_assignment.definitions.functions[0].body,
            ),
            Err(PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            }),
        );
    }

    #[test]
    fn plan_source_stop_with_unbound_return_type_as_nil() {
        let block_with_source_stop = compile(
            r#"
pub fn main() {
  {
    panic
  }
}
"#,
        );

        assert_eq!(
            super::function_return_type(
                "main".into(),
                type_::unbound_var(0).as_ref(),
                &block_with_source_stop.definitions.functions[0].body,
            ),
            Ok(ValueType::Nil),
        );
    }

    #[test]
    fn reject_profile_source_stop_with_generic_return_type() {
        let block_with_source_stop = compile(
            r#"
pub fn main() {
  {
    panic
  }
}
"#,
        );

        assert_eq!(
            super::function_return_type(
                "main".into(),
                type_::generic_var(0).as_ref(),
                &block_with_source_stop.definitions.functions[0].body,
            ),
            Err(PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            }),
        );
        assert_eq!(
            expect_plan_error(
                r#"
fn fail() -> a {
  panic
}

pub fn main() {
  1
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "fail".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            },
        );
    }

    #[test]
    fn reject_profile_source_stop_with_explicit_unsupported_return_type() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() -> BitArray {
  panic
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            },
        );
        assert_eq!(
            expect_plan_error(
                r#"
fn helper() -> BitArray {
  panic
}

pub fn main() {
  1
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "helper".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
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
  echo 1
}

pub fn main() {
  1
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
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
  echo 1
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
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
    fn plan_function_list_params_preserve_item_family_boundaries() {
        let actual = plan_module(compile(
            r#"
fn collect(
  ints: List(Int),
  strings: List(String),
  floats: List(Float),
  bools: List(Bool),
  nils: List(Nil),
  tuples: List(#(Int, String)),
  lists: List(List(Float)),
  functions: List(fn(Int) -> String),
) {
  Nil
}

pub fn main() {
  Nil
}
"#,
        ))
        .expect("source should plan");
        let collect = actual
            .functions()
            .iter()
            .find(|function| function.name() == "collect")
            .expect("collect function should be planned");
        let nested_function_type = FunctionType::new(vec![ValueType::Int], ValueType::String);

        assert_eq!(
            collect.params(),
            &[
                Param::named(
                    ParamLocal::list(ListLocal::int(IntListLocalId(0))),
                    "ints".into(),
                ),
                Param::named(
                    ParamLocal::list(ListLocal::string(StringListLocalId(0))),
                    "strings".into(),
                ),
                Param::named(
                    ParamLocal::list(ListLocal::float(FloatListLocalId(0))),
                    "floats".into(),
                ),
                Param::named(
                    ParamLocal::list(ListLocal::bool(BoolListLocalId(0))),
                    "bools".into(),
                ),
                Param::named(
                    ParamLocal::list(ListLocal::nil(NilListLocalId(0))),
                    "nils".into(),
                ),
                Param::named(
                    ParamLocal::list(ListLocal::tuple(
                        TupleListLocalId(0),
                        vec![ValueType::Int, ValueType::String],
                    )),
                    "tuples".into(),
                ),
                Param::named(
                    ParamLocal::list(ListLocal::list(ListListLocalId(0), ValueType::Float)),
                    "lists".into(),
                ),
                Param::named(
                    ParamLocal::list(ListLocal::function(
                        FunctionListLocalId(0),
                        nested_function_type,
                    )),
                    "functions".into(),
                ),
            ],
        );
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
