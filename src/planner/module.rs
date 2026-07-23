use crate::frontend::TypedProgram;
use crate::plan::{
    FunctionFunctionLocalId, FunctionTemplateId, IntFunctionLocalId, ModuleId, ModulePlan,
    ParamBinding, ParamLocal, PlannedModule, SourceContext,
};
use crate::planner::context::{AnonymousFunctions, FunctionInfo, FunctionParam};
use crate::planner::error::{
    InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedFunctionReason,
    UnsupportedTopLevelKind,
};
use crate::planner::function::{function_name, plan_function};
use crate::planner::type_parameter::TypeParameterScope;
use ecow::EcoString;
use gleam_core::ast::{ArgNames, TypedFunction, TypedModule};
use gleam_core::type_::Type;
use std::collections::HashMap;

pub(in crate::planner) use constant::ConstantRegistry;
#[cfg(test)]
pub(in crate::planner) use constant::plan_constants;
use constant::plan_constants_in;

pub fn plan_module(module: TypedModule) -> Result<ModulePlan, PlanError> {
    plan_module_inner(module, None)
}

pub fn plan_module_with_source(
    module: TypedModule,
    source_context: SourceContext,
) -> Result<ModulePlan, PlanError> {
    plan_module_inner(module, Some(source_context))
}

pub fn plan_program(program: TypedProgram) -> Result<ModulePlan, PlanError> {
    let (root_index, modules) = program.into_parts();
    let root = ModuleId::new(root_index);
    let mut planned_modules = Vec::with_capacity(modules.len());

    for (index, module) in modules.into_iter().enumerate() {
        let id = ModuleId::new(index);
        let role = if id == root {
            ModuleRole::Root
        } else {
            ModuleRole::Dependency
        };
        planned_modules.push(plan_typed_module(
            id,
            module.module,
            Some(SourceContext::new(module.path, module.source)),
            role,
        )?);
    }

    Ok(ModulePlan::from_modules(
        root,
        FunctionTemplateId::in_module(root, 0),
        planned_modules,
    ))
}

fn plan_module_inner(
    module: TypedModule,
    source_context: Option<SourceContext>,
) -> Result<ModulePlan, PlanError> {
    let module = plan_typed_module(ModuleId::root(), module, source_context, ModuleRole::Root)?;
    Ok(ModulePlan::from_modules(
        ModuleId::root(),
        FunctionTemplateId::in_module(ModuleId::root(), 0),
        vec![module],
    ))
}

#[derive(Clone, Copy)]
enum ModuleRole {
    Root,
    Dependency,
}

fn plan_typed_module(
    module_id: ModuleId,
    module: TypedModule,
    source_context: Option<SourceContext>,
    role: ModuleRole,
) -> Result<PlannedModule, PlanError> {
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
        functions,
        mut anonymous_functions,
    } = function_table(module_id, &definitions.functions, role)?;
    let constants = plan_constants_in(
        module_id,
        definitions.constants,
        &module_name,
        &by_name,
        &custom_types,
        &mut anonymous_functions,
    )?;
    let mut planned_functions = Vec::with_capacity(functions.len());

    for function in functions {
        let planned = plan_function(
            function.info,
            &module_name,
            &by_name,
            &custom_types,
            &constants,
            function.function,
            &mut anonymous_functions,
        )?;
        planned_functions.push(planned);
    }
    planned_functions.sort_by_key(|function| function.id().index());
    let anonymous_functions = anonymous_functions.into_functions();
    let constants = constants.into_templates();
    let module = PlannedModule::new(
        module_id,
        module_name,
        source_context,
        custom_types,
        constants,
        planned_functions,
        anonymous_functions,
    );

    Ok(module)
}

struct FunctionTable {
    by_name: HashMap<EcoString, FunctionInfo>,
    functions: Vec<FunctionToPlan>,
    anonymous_functions: AnonymousFunctions,
}

struct FunctionToPlan {
    info: FunctionInfo,
    function: TypedFunction,
}

fn function_table(
    module: ModuleId,
    functions: &[gleam_core::ast::TypedFunction],
    role: ModuleRole,
) -> Result<FunctionTable, PlanError> {
    let mut seeds = Vec::new();

    for function in functions {
        let name = function_name(function)?;
        let mut type_parameters = TypeParameterScope::default();
        let return_shape = function_return_shape_in(&function.return_type, &mut type_parameters);
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

    enum FunctionIndexing {
        Root { main_index: usize },
        Dependency,
    }

    let indexing = match role {
        ModuleRole::Root => {
            let main_index = seeds
                .iter()
                .position(|seed| seed.name == "main")
                .ok_or_else(|| PlanError::UnsupportedFunction {
                    name: "main".into(),
                    reason: UnsupportedFunctionReason::MissingMain,
                })?;
            if !seeds[main_index].params.is_empty() {
                return Err(PlanError::UnsupportedFunction {
                    name: "main".into(),
                    reason: UnsupportedFunctionReason::MainWithArguments,
                });
            }
            FunctionIndexing::Root { main_index }
        }
        ModuleRole::Dependency => FunctionIndexing::Dependency,
    };

    let mut by_name = HashMap::with_capacity(seeds.len());
    let mut functions_to_plan = Vec::with_capacity(seeds.len());
    for (source_index, seed) in seeds.into_iter().enumerate() {
        let local_index = match indexing {
            FunctionIndexing::Root { main_index } if source_index == main_index => 0,
            FunctionIndexing::Root { main_index } if source_index < main_index => source_index + 1,
            FunctionIndexing::Root { .. } | FunctionIndexing::Dependency => source_index,
        };
        let info = function_info(module, local_index, &seed);
        by_name.insert(seed.name.clone(), info.clone());
        functions_to_plan.push(FunctionToPlan {
            info,
            function: seed.function,
        });
    }

    let anonymous_functions = AnonymousFunctions::in_module(module, functions_to_plan.len());

    Ok(FunctionTable {
        by_name,
        functions: functions_to_plan,
        anonymous_functions,
    })
}

fn function_info(module: ModuleId, function_index: usize, seed: &FunctionSeed) -> FunctionInfo {
    FunctionInfo {
        signature: crate::plan::FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(module, function_index),
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

fn function_return_shape_in(
    type_: &Type,
    parameters: &mut TypeParameterScope,
) -> crate::plan::ValueShape {
    crate::plan::ValueShape::from_gleam_in(type_, parameters)
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

#[cfg(test)]
mod tests {
    use super::{plan_module, plan_program};
    use crate::frontend::{ModuleSource, compile_typed_program};
    use crate::plan::{
        BitArrayListLocalId, BoolListLocalId, ConstantTemplate, ConstantTemplateId,
        ConstantTemplateSignature, ConstantTemplates, ConstantValue, CustomConstructorDefinition,
        CustomFieldDefinition, CustomLocalId, CustomType, CustomTypeDefinition, CustomTypeName,
        CustomTypePublicity, CustomTypeTemplate, FloatListLocalId, FunctionFunctionId,
        FunctionListLocalId, FunctionType, GenericExpr, GenericFunctionLocal,
        GenericFunctionLocalId, GenericFunctionType, GenericListLocalId, GenericLocal,
        GenericLocalId, IntFunctionFunctionId, IntFunctionId, IntListLocalId, IntLocalId,
        ListListLocalId, ListLocal, LocalId, NilListLocalId, PanicExpr, PanicSite, Param,
        ParamLocal, ReturnBody, ReturnExpr, RuntimeFunctionId, SourceSpan, StringListLocalId,
        TupleListLocalId, TypeParameterId, TypeScheme, ValueShape, ValueType,
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
    fn plan_program_owns_dependency_first_modules_and_a_root_entry() {
        let typed = compile_typed_program(
            "root",
            [
                ModuleSource::new(
                    "alpha",
                    "support.gleam",
                    r#"
pub const answer = 1

pub fn main(value: Int) {
  value
}
"#,
                ),
                ModuleSource::new(
                    "root",
                    "main.gleam",
                    r#"
pub const answer = 2

pub fn main() {
  answer
}
"#,
                ),
            ],
        )
        .expect("program should compile");
        let plan = plan_program(typed).expect("program should plan");

        assert_eq!(plan.root(), crate::plan::ModuleId::new(1));
        assert_eq!(plan.module(), "root");
        assert_eq!(plan.entry().module(), plan.root());
        assert_eq!(plan.entry().index(), 0);
        assert_eq!(
            plan.modules()
                .iter()
                .map(|module| module.module().as_str())
                .collect::<Vec<_>>(),
            ["alpha", "root"],
        );
        assert_eq!(plan.modules()[0].id(), crate::plan::ModuleId::new(0));
        assert_eq!(plan.modules()[1].id(), crate::plan::ModuleId::new(1));
        assert_eq!(
            plan.modules()[0].functions()[0].id().module(),
            crate::plan::ModuleId::new(0),
        );
        assert_eq!(
            plan.modules()[1].functions()[0].id().module(),
            crate::plan::ModuleId::new(1),
        );
        assert_eq!(
            plan.modules()[0].constants()[0].id().module(),
            crate::plan::ModuleId::new(0),
        );
        assert_eq!(
            plan.modules()[1].constants()[0].id().module(),
            crate::plan::ModuleId::new(1),
        );
        assert_eq!(plan.modules()[0].functions()[0].params().len(), 1);
        assert_eq!(
            plan.source_context().map(|context| context.source()),
            Some(
                r#"
pub const answer = 2

pub fn main() {
  answer
}
"#,
            )
        );
    }

    #[test]
    fn plan_program_validates_every_dependency_body() {
        let typed = compile_typed_program(
            "main",
            [
                ModuleSource::new("main", "main.gleam", "pub fn main() { 1 }"),
                ModuleSource::new(
                    "support",
                    "support.gleam",
                    "pub fn unsupported() { <<1:native>> }",
                ),
            ],
        )
        .expect("program should compile");

        assert_eq!(
            plan_program(typed),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
            }),
        );
    }

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
            function("main", int_return_tail_call(1, [int_arg(int(41))])),
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
            ConstantTemplateSignature::int(ConstantTemplateId::new(0), 0, TypeScheme::new(0));
        let instantiation = signature
            .try_instantiate(Vec::new())
            .expect("a monomorphic constant should instantiate");
        let constants = ConstantTemplates::from_entries(vec![(
            ConstantTemplate::new(signature, "answer".into()),
            ConstantValue::int(42.into()),
        )]);
        let expected = module(
            "main",
            function(
                "main",
                crate::plan::IntReturn::expr(
                    ConstantTemplates::reference(instantiation)
                        .into_int()
                        .expect("an Int constant reference should retain its family"),
                ),
            ),
            [],
        )
        .with_constants(constants);

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
            super::function_table(
                crate::plan::ModuleId::root(),
                &module.definitions.functions,
                super::ModuleRole::Root,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous>".into(),
                    reason: InvalidFunctionShapeReason::Anonymous,
                },
            }),
        );
    }

    #[test]
    fn plan_empty_source_body_as_parametric_generated_todo() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
}
"#,
        ))
        .expect("source should plan");
        assert_eq!(
            actual.main_function().return_(),
            &crate::plan::ReturnExpr::generic_body(
                TypeParameterId(0),
                ReturnBody::expr(GenericExpr::panic(
                    TypeParameterId(0),
                    PanicExpr::empty_function_at(PanicSite::new(
                        "main".into(),
                        "main".into(),
                        SourceSpan::new(1, 14),
                    )),
                )),
            ),
        );
    }

    #[test]
    fn function_return_type_preserves_custom_non_source_stop_shapes() {
        let result_type = result_type();
        assert_eq!(
            ValueShape::from_gleam(type_::result(type_::int(), type_::nil()).as_ref())
                .map(|shape| shape.value_type()),
            Some(result_type.clone()),
        );

        assert_eq!(
            ValueShape::from_gleam(type_::result(type_::int(), type_::nil()).as_ref())
                .map(|shape| shape.value_type()),
            Some(result_type),
        );
    }

    #[test]
    fn preserve_unbound_return_type_as_parameter() {
        let mut parameters = super::TypeParameterScope::default();
        assert_eq!(
            super::function_return_shape_in(type_::unbound_var(0).as_ref(), &mut parameters)
                .value_type(),
            ValueType::Parameter(TypeParameterId(0)),
        );
        assert_eq!(parameters.scheme(), TypeScheme::new(1));
    }

    #[test]
    fn parametric_function_return_shapes_preserve_inferred_results() {
        let mut concrete_parameters = super::TypeParameterScope::default();
        assert_eq!(
            super::function_return_shape_in(type_::int().as_ref(), &mut concrete_parameters),
            ValueShape::Int,
        );

        let mut source_stop_parameters = super::TypeParameterScope::default();
        assert_eq!(
            super::function_return_shape_in(
                type_::unbound_var(41).as_ref(),
                &mut source_stop_parameters,
            ),
            ValueShape::Parameter(TypeParameterId(0)),
        );
        assert_eq!(source_stop_parameters.scheme(), TypeScheme::new(1));

        let mut inferred_parameters = super::TypeParameterScope::default();
        assert_eq!(
            super::function_return_shape_in(
                type_::unbound_var(41).as_ref(),
                &mut inferred_parameters
            ),
            ValueShape::Parameter(TypeParameterId(0)),
        );
        assert_eq!(inferred_parameters.scheme(), TypeScheme::new(1));
    }

    #[test]
    fn preserve_generic_return_without_template_scope() {
        let mut parameters = super::TypeParameterScope::default();
        assert_eq!(
            super::function_return_shape_in(type_::generic_var(0).as_ref(), &mut parameters)
                .value_type(),
            ValueType::Parameter(TypeParameterId(0)),
        );
        assert_eq!(parameters.scheme(), TypeScheme::new(1));
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
    fn plan_representable_unresolved_generic_main_as_specialization_root() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  []
}
"#,
        ))
        .expect("an empty generic list has a runtime representation");

        assert_eq!(actual.main_function().scheme(), &TypeScheme::new(1));
        assert_eq!(
            actual.main_function().signature().shape().return_shape(),
            &ValueShape::List(Box::new(ValueShape::Parameter(TypeParameterId(0)))),
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
            ValueShape::from_gleam(main.definitions.functions[0].return_type.as_ref())
                .map(|shape| shape.value_type()),
            Some(result_type.clone()),
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
            ValueShape::from_gleam(helper.return_type.as_ref()).map(|shape| shape.value_type()),
            Some(result_type),
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
                int_return_tail_call(1, [int_arg(int(1)), int_arg(int(42))]),
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
            function("main", int_return_tail_call(1, [int_arg(int(1))])),
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
