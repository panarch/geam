use crate::plan::{
    FunctionFunctionLocalId, FunctionTemplateId, IntFunctionLocalId, ModulePlan, ParamBinding,
    ParamLocal, SourceContext, ValueType,
};
use crate::planner::context::{AnonymousFunctions, FunctionInfo, FunctionParam};
use crate::planner::error::{
    InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedFunctionReason,
    UnsupportedTopLevelKind,
};
use crate::planner::function::{function_name, plan_function};
use crate::planner::type_parameter::TypeParameterScope;
use ecow::EcoString;
use gleam_core::ast::{ArgNames, Statement, TypedExpr, TypedFunction, TypedModule};
use gleam_core::type_::{Type, TypeVar};
use std::collections::HashMap;
use std::ops::Deref;

pub(in crate::planner) use constant::ConstantRegistry;
#[cfg(test)]
pub(in crate::planner) use constant::plan_constants;

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
    let package = module.type_info.package.clone();
    let definitions = module.definitions;

    let imports = definitions.imports.len();

    if imports != 0 {
        return Err(PlanError::UnsupportedTopLevel {
            kind: UnsupportedTopLevelKind::Import,
        });
    }
    let module_name = module.name;
    let custom_types =
        custom_type::plan_custom_types(&package, &module_name, definitions.custom_types)?;
    let FunctionTable {
        by_name,
        main,
        functions_before_main,
        functions_after_main,
        mut anonymous_functions,
    } = function_table(&definitions.functions)?;
    let main = validate_main_function(main)?;
    let constants = constant::plan_constants(
        definitions.constants,
        &module_name,
        &by_name,
        &custom_types,
        &mut anonymous_functions,
    )?;
    let mut functions = Vec::new();

    for function in functions_before_main {
        let planned = plan_function(
            function.info,
            &module_name,
            &by_name,
            &custom_types,
            &constants,
            function.function,
            &mut anonymous_functions,
        )?;
        functions.push(planned);
    }

    let main = plan_function(
        main.info,
        &module_name,
        &by_name,
        &custom_types,
        &constants,
        main.function,
        &mut anonymous_functions,
    )?;

    for function in functions_after_main {
        let planned = plan_function(
            function.info,
            &module_name,
            &by_name,
            &custom_types,
            &constants,
            function.function,
            &mut anonymous_functions,
        )?;
        functions.push(planned);
    }
    let anonymous_functions = anonymous_functions.into_functions();
    let constants = constants.into_templates();
    validate_executable_main(&main)?;

    Ok(ModulePlan::new(module_name, main, functions)
        .with_custom_types(custom_types)
        .with_constants(constants)
        .with_anonymous_functions(anonymous_functions))
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
        let mut type_parameters = TypeParameterScope::default();
        let return_shape =
            function_return_shape_in(&function.return_type, &function.body, &mut type_parameters);
        let params = function_params_allowing_labels_in(&function.arguments, &mut type_parameters);
        let scheme = type_parameters.scheme();
        seeds.push(FunctionSeed {
            name,
            function: function.clone(),
            params,
            return_shape,
            scheme,
            type_parameters,
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

    let main_info = function_info(0, &main_seed);
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

        let info = function_info(next_function_index, &seed);
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

    let anonymous_functions = AnonymousFunctions::new(next_function_index);

    Ok(FunctionTable {
        by_name,
        main,
        functions_before_main,
        functions_after_main,
        anonymous_functions,
    })
}

fn function_info(function_index: usize, seed: &FunctionSeed) -> FunctionInfo {
    FunctionInfo {
        signature: crate::plan::FunctionTemplateSignature::new(
            FunctionTemplateId::new(function_index),
            seed.scheme.clone(),
            crate::plan::FunctionShape::new(
                seed.params
                    .iter()
                    .map(|param| param.shape().clone())
                    .collect(),
                seed.return_shape.clone(),
            ),
        ),
        type_parameters: seed.type_parameters.clone(),
        return_shape: seed.return_shape.clone(),
        params: seed.params.clone(),
    }
}

#[derive(Clone)]
struct FunctionSeed {
    name: EcoString,
    function: TypedFunction,
    params: Vec<FunctionParam>,
    return_shape: crate::plan::ValueShape,
    scheme: crate::plan::TypeScheme,
    type_parameters: TypeParameterScope,
}

#[cfg(test)]
fn function_return_type(
    name: EcoString,
    type_: &Type,
    body: &[gleam_core::ast::TypedStatement],
) -> Result<ValueType, PlanError> {
    function_return_shape(name, type_, body).map(|shape| shape.value_type())
}

#[cfg(test)]
fn function_return_shape(
    name: EcoString,
    type_: &Type,
    body: &[gleam_core::ast::TypedStatement],
) -> Result<crate::plan::ValueShape, PlanError> {
    if let Some(return_shape) = crate::plan::ValueShape::from_gleam(type_) {
        return Ok(return_shape);
    }

    if is_inferred_return_type(type_)
        && let Some(return_type) = source_stop_return_type(body)
    {
        return Ok(crate::plan::ValueShape::from_value_type(return_type));
    }

    Err(PlanError::UnsupportedFunction {
        name,
        reason: UnsupportedFunctionReason::UnsupportedReturnType,
    })
}

fn function_return_shape_in(
    type_: &Type,
    body: &[gleam_core::ast::TypedStatement],
    parameters: &mut TypeParameterScope,
) -> crate::plan::ValueShape {
    if !is_inferred_return_type(type_) {
        return crate::plan::ValueShape::from_gleam_in(type_, parameters);
    }

    if let Some(return_type) = source_stop_return_type(body) {
        return crate::plan::ValueShape::from_value_type(return_type);
    }

    crate::plan::ValueShape::from_gleam_in(type_, parameters)
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

pub(super) fn function_params_in(
    function_name: EcoString,
    arguments: &[gleam_core::ast::TypedArg],
    parameters: &mut TypeParameterScope,
) -> Result<Vec<FunctionParam>, PlanError> {
    if arguments.iter().any(|argument| {
        matches!(
            argument.names,
            ArgNames::NamedLabelled { .. } | ArgNames::LabelledDiscard { .. }
        )
    }) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: function_name,
                reason: InvalidFunctionShapeReason::LabelledArgument,
            },
        });
    }

    Ok(function_params_allowing_labels_in(arguments, parameters))
}

fn function_params_allowing_labels_in(
    arguments: &[gleam_core::ast::TypedArg],
    parameters: &mut TypeParameterScope,
) -> Vec<FunctionParam> {
    let mut next_generic = 0;
    let mut next_int = 0;
    let mut next_float = 0;
    let mut next_string = 0;
    let mut next_bit_array = 0;
    let mut next_utf_codepoint = 0;
    let mut next_custom = 0;
    let mut next_bool = 0;
    let mut next_nil = 0;
    let mut next_tuple = 0;
    let mut next_int_list = 0;
    let mut next_string_list = 0;
    let mut next_bit_array_list = 0;
    let mut next_utf_codepoint_list = 0;
    let mut next_custom_list = 0;
    let mut next_float_list = 0;
    let mut next_bool_list = 0;
    let mut next_nil_list = 0;
    let mut next_tuple_list = 0;
    let mut next_list_list = 0;
    let mut next_function_list = 0;
    let mut next_generic_list = 0;
    let mut function_locals = FunctionParamLocalCounters::default();

    arguments
        .iter()
        .map(|argument| {
            let (binding, label) = match &argument.names {
                ArgNames::Named { name, .. } => (ParamBinding::Named(name.clone()), None),
                ArgNames::Discard { .. } => (ParamBinding::Discard, None),
                ArgNames::NamedLabelled { label, name, .. } => {
                    (ParamBinding::Named(name.clone()), Some(label.clone()))
                }
                ArgNames::LabelledDiscard { label, .. } => {
                    (ParamBinding::Discard, Some(label.clone()))
                }
            };

            let shape = crate::plan::ValueShape::from_gleam_in(&argument.type_, parameters);
            let local = match &shape {
                crate::plan::ValueShape::Parameter(parameter) => {
                    let local = ParamLocal::generic(crate::plan::GenericLocal::new(
                        crate::plan::GenericLocalId(next_generic),
                        *parameter,
                    ));
                    next_generic += 1;
                    local
                }
                crate::plan::ValueShape::Int => {
                    let local = ParamLocal::int(crate::plan::IntLocalId(next_int));
                    next_int += 1;
                    local
                }
                crate::plan::ValueShape::Float => {
                    let local = ParamLocal::float(crate::plan::FloatLocalId(next_float));
                    next_float += 1;
                    local
                }
                crate::plan::ValueShape::String => {
                    let local = ParamLocal::string(crate::plan::StringLocalId(next_string));
                    next_string += 1;
                    local
                }
                crate::plan::ValueShape::BitArray => {
                    let local = ParamLocal::bit_array(crate::plan::BitArrayLocalId(next_bit_array));
                    next_bit_array += 1;
                    local
                }
                crate::plan::ValueShape::UtfCodepoint => {
                    let local = ParamLocal::utf_codepoint(crate::plan::UtfCodepointLocalId(
                        next_utf_codepoint,
                    ));
                    next_utf_codepoint += 1;
                    local
                }
                crate::plan::ValueShape::Custom(custom_shape) => {
                    let local = ParamLocal::custom_shape(
                        crate::plan::CustomLocalId(next_custom),
                        custom_shape.clone(),
                    );
                    next_custom += 1;
                    local
                }
                crate::plan::ValueShape::Bool => {
                    let local = ParamLocal::bool(crate::plan::BoolLocalId(next_bool));
                    next_bool += 1;
                    local
                }
                crate::plan::ValueShape::Nil => {
                    let local = ParamLocal::nil(crate::plan::NilLocalId(next_nil));
                    next_nil += 1;
                    local
                }
                crate::plan::ValueShape::Tuple(elements) => {
                    let local = ParamLocal::tuple(
                        crate::plan::TupleLocalId(next_tuple),
                        elements
                            .iter()
                            .map(crate::plan::ValueShape::value_type)
                            .collect(),
                    );
                    next_tuple += 1;
                    local
                }
                crate::plan::ValueShape::List(element_shape) => {
                    let local = match element_shape.as_ref() {
                        crate::plan::ValueShape::Parameter(parameter) => {
                            let local = crate::plan::ListLocal::generic(
                                crate::plan::GenericListLocalId(next_generic_list),
                                *parameter,
                            );
                            next_generic_list += 1;
                            local
                        }
                        crate::plan::ValueShape::Int => {
                            let local = crate::plan::ListLocal::int(crate::plan::IntListLocalId(
                                next_int_list,
                            ));
                            next_int_list += 1;
                            local
                        }
                        crate::plan::ValueShape::String => {
                            let local = crate::plan::ListLocal::string(
                                crate::plan::StringListLocalId(next_string_list),
                            );
                            next_string_list += 1;
                            local
                        }
                        crate::plan::ValueShape::BitArray => {
                            let local = crate::plan::ListLocal::bit_array(
                                crate::plan::BitArrayListLocalId(next_bit_array_list),
                            );
                            next_bit_array_list += 1;
                            local
                        }
                        crate::plan::ValueShape::UtfCodepoint => {
                            let local = crate::plan::ListLocal::utf_codepoint(
                                crate::plan::UtfCodepointListLocalId(next_utf_codepoint_list),
                            );
                            next_utf_codepoint_list += 1;
                            local
                        }
                        crate::plan::ValueShape::Custom(item_shape) => {
                            let local = crate::plan::ListLocal::custom(
                                crate::plan::CustomListLocalId(next_custom_list),
                                item_shape.type_().clone(),
                            );
                            next_custom_list += 1;
                            local
                        }
                        crate::plan::ValueShape::Float => {
                            let local = crate::plan::ListLocal::float(
                                crate::plan::FloatListLocalId(next_float_list),
                            );
                            next_float_list += 1;
                            local
                        }
                        crate::plan::ValueShape::Bool => {
                            let local = crate::plan::ListLocal::bool(crate::plan::BoolListLocalId(
                                next_bool_list,
                            ));
                            next_bool_list += 1;
                            local
                        }
                        crate::plan::ValueShape::Nil => {
                            let local = crate::plan::ListLocal::nil(crate::plan::NilListLocalId(
                                next_nil_list,
                            ));
                            next_nil_list += 1;
                            local
                        }
                        crate::plan::ValueShape::Tuple(item_shape) => {
                            let local = crate::plan::ListLocal::tuple(
                                crate::plan::TupleListLocalId(next_tuple_list),
                                item_shape
                                    .iter()
                                    .map(crate::plan::ValueShape::value_type)
                                    .collect(),
                            );
                            next_tuple_list += 1;
                            local
                        }
                        crate::plan::ValueShape::List(item_shape) => {
                            let local = crate::plan::ListLocal::list(
                                crate::plan::ListListLocalId(next_list_list),
                                item_shape.value_type(),
                            );
                            next_list_list += 1;
                            local
                        }
                        crate::plan::ValueShape::Function(item_shape) => {
                            let local = crate::plan::ListLocal::function(
                                crate::plan::FunctionListLocalId(next_function_list),
                                item_shape.type_(),
                            );
                            next_function_list += 1;
                            local
                        }
                    };
                    ParamLocal::list(local)
                }
                crate::plan::ValueShape::Function(function_shape) => {
                    function_locals.next_shape(function_shape)
                }
            };
            FunctionParam::new(local, shape, binding, label)
        })
        .collect()
}

#[derive(Default)]
struct FunctionParamLocalCounters {
    next_generic: usize,
    next_int: usize,
    next_float: usize,
    next_string: usize,
    next_bit_array: usize,
    next_utf_codepoint: usize,
    next_custom: usize,
    next_bool: usize,
    next_nil: usize,
    next_tuple: usize,
    next_list: usize,
    next_function: usize,
}

impl FunctionParamLocalCounters {
    fn next_shape(&mut self, shape: &crate::plan::FunctionShape) -> ParamLocal {
        let type_ = shape.type_();
        match shape.return_shape() {
            crate::plan::ValueShape::Parameter(parameter) => {
                let local = ParamLocal::generic_function(crate::plan::GenericFunctionLocal::new(
                    crate::plan::GenericFunctionLocalId(self.next_generic),
                    crate::plan::GenericFunctionType::new(
                        shape.argument_shapes().to_vec(),
                        *parameter,
                    ),
                ));
                self.next_generic += 1;
                local
            }
            crate::plan::ValueShape::Int => {
                let local =
                    ParamLocal::int_function(IntFunctionLocalId(self.next_int), type_.clone());
                self.next_int += 1;
                local
            }
            crate::plan::ValueShape::Float => {
                let local = ParamLocal::float_function(
                    crate::plan::FloatFunctionLocalId(self.next_float),
                    type_.clone(),
                );
                self.next_float += 1;
                local
            }
            crate::plan::ValueShape::String => {
                let local = ParamLocal::string_function(
                    crate::plan::StringFunctionLocalId(self.next_string),
                    type_.clone(),
                );
                self.next_string += 1;
                local
            }
            crate::plan::ValueShape::BitArray => {
                let local = ParamLocal::bit_array_function(
                    crate::plan::BitArrayFunctionLocalId(self.next_bit_array),
                    type_.clone(),
                );
                self.next_bit_array += 1;
                local
            }
            crate::plan::ValueShape::UtfCodepoint => {
                let local = ParamLocal::utf_codepoint_function(
                    crate::plan::UtfCodepointFunctionLocalId(self.next_utf_codepoint),
                    type_.clone(),
                );
                self.next_utf_codepoint += 1;
                local
            }
            crate::plan::ValueShape::Custom(return_shape) => {
                let local = ParamLocal::custom_function(crate::plan::CustomFunctionLocal::new(
                    crate::plan::CustomFunctionLocalId(self.next_custom),
                    crate::plan::CustomFunctionType::from_shapes(
                        shape.argument_shapes().to_vec(),
                        return_shape.clone(),
                    ),
                ));
                self.next_custom += 1;
                local
            }
            crate::plan::ValueShape::Bool => {
                let local = ParamLocal::bool_function(
                    crate::plan::BoolFunctionLocalId(self.next_bool),
                    type_.clone(),
                );
                self.next_bool += 1;
                local
            }
            crate::plan::ValueShape::Nil => {
                let local = ParamLocal::nil_function(
                    crate::plan::NilFunctionLocalId(self.next_nil),
                    type_.clone(),
                );
                self.next_nil += 1;
                local
            }
            crate::plan::ValueShape::Tuple(_) => {
                let local = ParamLocal::tuple_function(
                    crate::plan::TupleFunctionLocalId(self.next_tuple),
                    type_.clone(),
                );
                self.next_tuple += 1;
                local
            }
            crate::plan::ValueShape::List(item_shape) => {
                let local =
                    ParamLocal::list_function(crate::plan::ListFunctionLocal::from_item_type(
                        self.next_list,
                        type_.clone(),
                        item_shape.value_type(),
                    ));
                self.next_list += 1;
                local
            }
            crate::plan::ValueShape::Function(return_shape) => {
                let local = ParamLocal::function_function(crate::plan::FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(self.next_function),
                    crate::plan::FunctionFunctionType::from_shapes(
                        shape.argument_shapes().to_vec(),
                        return_shape.as_ref().clone(),
                    ),
                ));
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

fn validate_executable_main(main: &crate::plan::FunctionTemplate) -> Result<(), PlanError> {
    if main.scheme().is_monomorphic() {
        Ok(())
    } else {
        Err(PlanError::UnsupportedFunction {
            name: "main".into(),
            reason: UnsupportedFunctionReason::UnsupportedReturnType,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::plan_module;
    use crate::plan::{
        BitArrayListLocalId, BoolListLocalId, ConstantTemplate, ConstantTemplateId,
        ConstantTemplateSignature, ConstantTemplates, ConstantValue, CustomConstructorDefinition,
        CustomFieldDefinition, CustomLocalId, CustomType, CustomTypeDefinition, CustomTypeName,
        CustomTypePublicity, CustomTypeTemplate, FloatListLocalId, FunctionFunctionId,
        FunctionListLocalId, FunctionType, GenericExpr, GenericFunctionLocal,
        GenericFunctionLocalId, GenericFunctionType, GenericListLocalId, GenericLocal,
        GenericLocalId, IntFunctionFunctionId, IntFunctionId, IntListLocalId, IntLocalId,
        ListListLocalId, ListLocal, LocalId, NilExpr, NilFunctionId, NilListLocalId, PanicExpr,
        PanicSite, Param, ParamLocal, ReturnBody, ReturnExpr, RuntimeFunctionId, SourceSpan,
        StringListLocalId, TupleListLocalId, TypeParameterId, TypeScheme, ValueShape, ValueType,
    };
    use crate::planner::dsl::{
        call_int, call_int_returning_function, function, function_ref, int, int_arg,
        int_return_tail_call, local_int, module,
    };
    use crate::planner::support::{compile, expect_plan_error};
    use crate::planner::{
        InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
        UnsupportedFunctionReason, UnsupportedTopLevelKind,
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
        let signature =
            ConstantTemplateSignature::int(ConstantTemplateId(0), 0, TypeScheme::new(0));
        let expected = module("main", function("main", int(42)), []).with_constants(
            ConstantTemplates::from_entries(vec![(
                ConstantTemplate::new(signature, "answer".into()),
                ConstantValue::int(42.into()),
            )]),
        );

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
    fn function_return_type_preserves_custom_non_source_stop_shapes() {
        let result_type = result_type();
        assert!(!super::is_inferred_return_type(type_::int().as_ref()));
        assert_eq!(super::source_stop_return_type(&[]), None);
        assert_eq!(
            super::function_return_type(
                "values".into(),
                type_::result(type_::int(), type_::nil()).as_ref(),
                &[],
            ),
            Ok(result_type.clone()),
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
                type_::result(type_::int(), type_::nil()).as_ref(),
                &block_with_final_assignment.definitions.functions[0].body,
            ),
            Ok(result_type),
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
    fn parametric_function_return_shapes_preserve_inferred_and_source_stop_results() {
        let source_stop = compile(
            r#"
pub fn main() {
  panic
}
"#,
        );
        let mut concrete_parameters = super::TypeParameterScope::default();
        assert_eq!(
            super::function_return_shape_in(type_::int().as_ref(), &[], &mut concrete_parameters),
            ValueShape::Int,
        );

        let mut source_stop_parameters = super::TypeParameterScope::default();
        assert_eq!(
            super::function_return_shape_in(
                type_::unbound_var(41).as_ref(),
                &source_stop.definitions.functions[0].body,
                &mut source_stop_parameters,
            ),
            ValueShape::Nil,
        );

        let mut inferred_parameters = super::TypeParameterScope::default();
        assert_eq!(
            super::function_return_shape_in(
                type_::unbound_var(41).as_ref(),
                &[],
                &mut inferred_parameters,
            ),
            ValueShape::Parameter(TypeParameterId(0)),
        );
        assert_eq!(inferred_parameters.scheme(), TypeScheme::new(1));
    }

    #[test]
    fn reject_margin_source_stop_generic_return_without_template_scope() {
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
    }

    #[test]
    fn plan_source_stop_generic_function_as_template() {
        let actual = plan_module(compile(
            r#"
fn fail() -> a {
  panic
}

pub fn main() {
  1
}
"#,
        ))
        .expect("generic source-stop function should plan as a template");
        let fail = &actual.functions()[0];
        let parameter = TypeParameterId(0);
        assert_eq!(fail.scheme(), &TypeScheme::new(1));
        assert_eq!(
            fail.return_(),
            &ReturnExpr::generic_body(
                parameter,
                ReturnBody::expr(GenericExpr::panic(
                    parameter,
                    PanicExpr::panic_at(
                        None,
                        PanicSite::new("main".into(), "fail".into(), SourceSpan::new(20, 25),),
                    ),
                )),
            ),
        );
    }

    #[test]
    fn reject_profile_unresolved_generic_main_as_specialization_root() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  []
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            },
        );
    }

    #[test]
    fn function_return_type_preserves_explicit_custom_source_stop_shapes() {
        let result_type = result_type();
        let main = compile(
            r#"
pub fn main() -> Result(Int, Nil) {
  panic
}
"#,
        );
        assert_eq!(
            super::function_return_type(
                "main".into(),
                main.definitions.functions[0].return_type.as_ref(),
                &main.definitions.functions[0].body,
            ),
            Ok(result_type.clone()),
        );

        let helper = compile(
            r#"
pub fn main() {
  1
}

fn helper() -> Result(Int, Nil) {
  panic
}
"#,
        );
        let helper = &helper.definitions.functions[1];
        assert_eq!(
            super::function_return_type("helper".into(), helper.return_type.as_ref(), &helper.body,),
            Ok(result_type),
        );
    }

    #[test]
    fn plan_custom_returning_functions_before_and_after_main() {
        let actual = plan_module(compile(
            r#"
fn before() -> Result(Int, Nil) {
  Ok(1)
}

pub fn main() {
  1
}

fn after() -> Result(Int, Nil) {
  Ok(2)
}
"#,
        ))
        .expect("concrete custom return types should plan");
        let functions = actual
            .functions()
            .iter()
            .map(|function| {
                (
                    function.name().clone(),
                    function.id(),
                    function.return_().value_type(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            functions,
            vec![
                (
                    "before".into(),
                    crate::plan::FunctionTemplateId::new(1),
                    result_type(),
                ),
                (
                    "after".into(),
                    crate::plan::FunctionTemplateId::new(2),
                    result_type(),
                ),
            ],
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
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(2)),
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
                2,
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
                    crate::plan::FunctionFunctionType::new(Vec::new(), returned_function_type),
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
    fn plan_custom_function_argument_type() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1
}

fn count(values: Result(Int, Nil)) {
  1
}
"#,
        ))
        .expect("concrete custom arguments should plan");
        assert_eq!(
            actual.functions()[0].params(),
            &[Param::named(
                ParamLocal::custom(CustomLocalId(0), result_custom_type()),
                "values".into(),
            )],
        );
    }

    #[test]
    fn reject_margin_custom_function_argument_with_mismatched_generic_return() {
        let mut module = compile(
            r#"
fn count(value: Int) { value }
pub fn main() { count(1) }
"#,
        );
        module.definitions.functions[0].arguments[0].type_ = type_::generic_var(0);

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "count".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn plan_type_alias_resolved_to_custom_argument_type() {
        let actual = plan_module(compile(
            r#"
pub type Outcome =
  Result(Int, Nil)

pub fn main() {
  1
}

fn count(values: Outcome) {
  1
}
"#,
        ))
        .expect("aliases to concrete custom arguments should plan");
        assert_eq!(
            actual.functions()[0].params(),
            &[Param::named(
                ParamLocal::custom(CustomLocalId(0), result_custom_type()),
                "values".into(),
            )],
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
  bit_arrays: List(BitArray),
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
                    ParamLocal::list(ListLocal::bit_array(BitArrayListLocalId(0))),
                    "bit_arrays".into(),
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
    fn plan_generic_params_preserve_scheme_owned_local_shapes() {
        let actual = plan_module(compile(
            r#"
fn apply(
  function: fn(value) -> value,
  value: value,
  values: List(value),
) -> value {
  function(value)
}

pub fn main() {
  apply(fn(value) { value }, 1, [1])
}
"#,
        ))
        .expect("concretely called generic params should plan as one template");
        let apply = actual
            .functions()
            .iter()
            .find(|function| function.name() == "apply")
            .expect("apply template should be planned");
        let parameter = TypeParameterId(0);
        let callable = GenericFunctionType::new(vec![ValueShape::Parameter(parameter)], parameter);

        assert_eq!(apply.scheme(), &TypeScheme::new(1));
        assert_eq!(
            apply.params(),
            &[
                Param::named(
                    ParamLocal::generic_function(GenericFunctionLocal::new(
                        GenericFunctionLocalId(0),
                        callable,
                    )),
                    "function".into(),
                ),
                Param::named(
                    ParamLocal::generic(GenericLocal::new(GenericLocalId(0), parameter)),
                    "value".into(),
                ),
                Param::named(
                    ParamLocal::list(ListLocal::generic(GenericListLocalId(0), parameter)),
                    "values".into(),
                ),
            ],
        );
    }

    #[test]
    fn plan_local_custom_type_definition() {
        let plan = plan_module(compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  1
}
"#,
        ))
        .expect("custom type should plan");

        assert_eq!(
            plan.custom_types(),
            &[CustomTypeDefinition::new(
                CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                CustomTypePublicity::Public,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Boxed".into(),
                    0,
                    vec![CustomFieldDefinition::new(None, CustomTypeTemplate::Int)],
                )],
            )],
        );
    }

    #[test]
    fn reject_profile_module_propagates_external_custom_type_owner_error() {
        let module = crate::frontend::compile_typed_module(
            "main",
            "main.gleam",
            r#"
@external(erlang, "external", "thing")
pub type Thing

pub fn main() { 1 }
"#,
        )
        .expect("an external custom type should analyse");

        assert_eq!(
            plan_module(module),
            Err(PlanError::UnsupportedTopLevel {
                kind: UnsupportedTopLevelKind::ExternalCustomType,
            }),
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

    fn result_type() -> ValueType {
        ValueType::Custom(result_custom_type())
    }

    fn result_custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
            vec![ValueType::Int, ValueType::Nil],
        )
    }
}
mod constant;
mod custom_type;
