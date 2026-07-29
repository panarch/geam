use super::constant::{self, plan_constant_bodies, reserve_constants};
use super::custom_type;
use super::registry::{ModuleRegistry, ProgramRegistry};
use super::{ModuleRole, discarded_function_params, function_table};
use crate::frontend::{HostedTypedProgram, HostedTypedProgramModule};
use crate::host::{
    HostFunctionSchema, HostProfile, RegisteredHostFunction, RegisteredHostImplementationId,
};
use crate::plan::{
    ConstantTemplates, FunctionTemplateId, HostFunctionTemplate, HostImplementationBinding,
    HostedFunctionTemplate, HostedModulePlan, HostedPlannedModule, HostedPlannedModuleParts,
    ModuleId, SourceContext,
};
use crate::planner::context::{FunctionInfo, PlanContext};
use crate::planner::error::{HostProviderLinkReason, PlanError};
use crate::planner::function::{plan_function, plan_selected_external_fallback};
use crate::planner::type_parameter::TypeParameterScope;
use ecow::EcoString;
use gleam_core::ast::TypedFunction;
use std::collections::{BTreeMap, HashMap, HashSet};

pub fn plan_host_program<Profile: HostProfile>(
    program: HostedTypedProgram<Profile>,
) -> Result<HostedModulePlan<Profile>, PlanError> {
    let (root_index, modules, providers, implementations) = program.into_parts();
    plan_host_program_schema(root_index, modules, providers).map(|planned| {
        let implementation_bindings = planned
            .implementations
            .into_iter()
            .map(|(template, implementation)| {
                HostImplementationBinding::new(
                    template,
                    implementations.implementation(implementation),
                )
            })
            .collect();
        HostedModulePlan::new(
            planned.root,
            planned.entry,
            planned.modules,
            implementation_bindings,
        )
    })
}

fn plan_host_program_schema(
    root_index: usize,
    modules: Vec<HostedTypedProgramModule>,
    providers: Vec<crate::host::RegisteredHostProviderModule>,
) -> Result<PlannedHostedProgram, PlanError> {
    let root = ModuleId::new(root_index);
    collect_hosted_module_declarations(modules, providers)
        .and_then(|declarations| link_hosted_modules(root, declarations))
        .and_then(reserve_hosted_constants)
        .and_then(|(registry, modules)| plan_hosted_constant_bodies(registry, modules))
        .and_then(|(registry, modules)| plan_hosted_modules(root, &registry, modules))
}

fn collect_hosted_module_declarations(
    modules: Vec<HostedTypedProgramModule>,
    providers: Vec<crate::host::RegisteredHostProviderModule>,
) -> Result<Vec<HostedModuleDeclaration>, PlanError> {
    let mut provider_modules = providers
        .into_iter()
        .map(|provider| {
            let (package, module, functions) = provider.into_parts();
            ((package, module), functions)
        })
        .collect::<BTreeMap<_, _>>();
    modules
        .into_iter()
        .enumerate()
        .map(|(index, module)| {
            hosted_module_declaration(ModuleId::new(index), module, &mut provider_modules)
        })
        .collect::<Result<Vec<_>, _>>()
        .and_then(|declarations| {
            if let Some(((package, module), functions)) = provider_modules.into_iter().next() {
                let function = functions
                    .iter()
                    .map(|function| function.schema().name().clone())
                    .min()
                    .unwrap_or_default();
                Err(PlanError::HostProviderLink {
                    package,
                    module,
                    function,
                    reason: Box::new(HostProviderLinkReason::MissingModule),
                })
            } else {
                Ok(declarations)
            }
        })
}

fn hosted_module_declaration(
    id: ModuleId,
    module: HostedTypedProgramModule,
    provider_modules: &mut BTreeMap<(EcoString, EcoString), Vec<RegisteredHostFunction>>,
) -> Result<HostedModuleDeclaration, PlanError> {
    match module {
        HostedTypedProgramModule::Source(module) => {
            let path = module.path;
            let source = module.source;
            let package = module.module.type_info.package.clone();
            let definitions = module.module.definitions;
            let module_name = module.module.name;
            custom_type::plan_custom_types(&package, &module_name, definitions.custom_types).map(
                |custom_types| HostedModuleDeclaration::Source {
                    id,
                    providers: provider_modules
                        .remove(&(package.clone(), module_name.clone()))
                        .unwrap_or_default(),
                    package,
                    module_name,
                    source_context: Some(SourceContext::new(path, source)),
                    custom_types,
                    functions: definitions.functions,
                    constants: definitions.constants,
                },
            )
        }
        HostedTypedProgramModule::Host(module) => {
            let (package, module_name, functions) = module.into_parts();
            Ok(HostedModuleDeclaration::Host {
                id,
                package,
                module_name,
                functions,
            })
        }
    }
}

fn link_hosted_modules(
    root: ModuleId,
    declarations: Vec<HostedModuleDeclaration>,
) -> Result<Vec<LinkedModule>, PlanError> {
    declarations
        .into_iter()
        .map(|declaration| link_hosted_module(root, declaration))
        .collect()
}

fn link_hosted_module(
    root: ModuleId,
    declaration: HostedModuleDeclaration,
) -> Result<LinkedModule, PlanError> {
    match declaration {
        HostedModuleDeclaration::Source {
            id,
            package,
            module_name,
            source_context,
            custom_types,
            functions,
            constants,
            providers,
        } => {
            let role = if id == root {
                ModuleRole::Root
            } else {
                ModuleRole::Dependency
            };
            function_table(id, &functions, role).and_then(|table| {
                link_source_functions(
                    package.clone(),
                    module_name.clone(),
                    table.functions,
                    providers,
                )
                .map(|(functions, executable_externals)| LinkedModule {
                    id,
                    package,
                    module_name,
                    source_context,
                    custom_types,
                    functions_by_name: table.by_name,
                    functions,
                    executable_externals,
                    constants,
                    anonymous_functions: table.anonymous_functions,
                })
            })
        }
        HostedModuleDeclaration::Host {
            id,
            package,
            module_name,
            functions,
        } => Ok(link_source_less_module(id, package, module_name, functions)),
    }
}

fn link_source_functions(
    package: EcoString,
    module: EcoString,
    functions: Vec<super::FunctionToPlan>,
    providers: Vec<RegisteredHostFunction>,
) -> Result<(Vec<LinkedFunction>, HashSet<EcoString>), PlanError> {
    let providers = providers
        .into_iter()
        .map(|definition| (definition.schema().name().clone(), definition))
        .collect::<BTreeMap<_, _>>();
    functions
        .into_iter()
        .try_fold(
            (Vec::new(), providers, HashSet::new()),
            |(mut linked, mut providers, mut executable_externals), function| {
                let name = function.name;
                let external = function.function.external_erlang.is_some()
                    || function.function.external_javascript.is_some();
                if let Some(provider) = providers.remove(&name) {
                    if !external {
                        return Err(PlanError::HostProviderLink {
                            package: package.clone(),
                            module: module.clone(),
                            function: name,
                            reason: Box::new(HostProviderLinkReason::NonExternalFunction),
                        });
                    }
                    executable_externals.insert(name);
                    bind_source_host_function(
                        package.clone(),
                        module.clone(),
                        provider,
                        &function.info,
                    )
                    .map(|(template, implementation)| {
                        linked.push(LinkedFunction::Host {
                            template,
                            implementation,
                        });
                        (linked, providers, executable_externals)
                    })
                } else if external {
                    if function.function.body.is_empty() {
                        return Err(PlanError::MissingHostProvider {
                            package: package.clone(),
                            module: module.clone(),
                            function: name,
                        });
                    }
                    executable_externals.insert(name.clone());
                    linked.push(LinkedFunction::ExternalFallback {
                        name,
                        info: function.info,
                        function: function.function,
                    });
                    Ok((linked, providers, executable_externals))
                } else {
                    linked.push(LinkedFunction::Gleam {
                        info: function.info,
                        function: function.function,
                    });
                    Ok((linked, providers, executable_externals))
                }
            },
        )
        .and_then(|(linked, providers, executable_externals)| {
            if let Some((function, _)) = providers.into_iter().next() {
                Err(PlanError::HostProviderLink {
                    package,
                    module,
                    function,
                    reason: Box::new(HostProviderLinkReason::MissingDeclaration),
                })
            } else {
                Ok((linked, executable_externals))
            }
        })
}

fn link_source_less_module(
    id: ModuleId,
    package: EcoString,
    module_name: EcoString,
    functions: Vec<RegisteredHostFunction>,
) -> LinkedModule {
    let function_count = functions.len();
    let mut functions_by_name = HashMap::with_capacity(function_count);
    let mut linked_functions = Vec::with_capacity(function_count);
    for (function_index, definition) in functions.into_iter().enumerate() {
        let function_id = FunctionTemplateId::in_module(id, function_index);
        let (template, implementation) = bind_source_less_host_function(
            function_id,
            package.clone(),
            module_name.clone(),
            definition,
        );
        let info = host_function_info(&template);
        functions_by_name.insert(template.name().clone(), info);
        linked_functions.push(LinkedFunction::Host {
            template,
            implementation,
        });
    }
    LinkedModule {
        id,
        package,
        module_name,
        source_context: None,
        custom_types: Vec::new(),
        functions_by_name,
        functions: linked_functions,
        executable_externals: HashSet::new(),
        constants: Vec::new(),
        anonymous_functions: super::AnonymousFunctions::in_module(id, function_count),
    }
}

fn reserve_hosted_constants(
    modules: Vec<LinkedModule>,
) -> Result<(ProgramRegistry, Vec<ModuleWithConstants>), PlanError> {
    modules
        .into_iter()
        .map(reserve_hosted_module_constants)
        .collect::<Result<Vec<_>, _>>()
        .and_then(|reserved| {
            let (registry_modules, modules): (Vec<_>, Vec<_>) = reserved.into_iter().unzip();
            let registry = ProgramRegistry::new(registry_modules);
            validate_host_custom_schemas(&registry, &modules)?;
            Ok((registry, modules))
        })
}

fn validate_host_custom_schemas(
    registry: &ProgramRegistry,
    modules: &[ModuleWithConstants],
) -> Result<(), PlanError> {
    for module in modules {
        let access = match module.source_context {
            Some(_) => HostCustomTypeAccess::SourceDeclaration,
            None => HostCustomTypeAccess::SourceLessPublicSurface,
        };
        for function in &module.functions {
            let LinkedFunction::Host { template, .. } = function else {
                continue;
            };
            for actual in template.custom_schemas() {
                validate_host_custom_schema(
                    registry,
                    template.package(),
                    template.site(),
                    template.signature(),
                    actual,
                    access,
                )?;
            }
        }
    }
    Ok(())
}

fn validate_host_custom_schema(
    registry: &ProgramRegistry,
    package: &EcoString,
    site: &crate::plan::HostCallSite,
    signature: &crate::plan::FunctionTemplateSignature,
    actual: &crate::host::HostCustomTypeSchema,
    access: HostCustomTypeAccess,
) -> Result<(), PlanError> {
    let name = crate::plan::CustomTypeName::new(
        actual.package().clone(),
        actual.module().clone(),
        actual.name().clone(),
    );
    let definition = host_custom_type_definition(registry, package, site, &name)?;
    let visible = match access {
        HostCustomTypeAccess::SourceLessPublicSurface => {
            !definition.is_opaque()
                && definition.publicity() == crate::plan::CustomTypePublicity::Public
        }
        HostCustomTypeAccess::SourceDeclaration => {
            let same_package = definition.name().package() == package;
            let same_module = same_package && definition.name().module() == site.module();
            if definition.is_opaque() {
                same_module
            } else {
                match definition.publicity() {
                    crate::plan::CustomTypePublicity::Public => true,
                    crate::plan::CustomTypePublicity::Internal => same_package,
                    crate::plan::CustomTypePublicity::Private => same_module,
                }
            }
        }
    };
    if !visible {
        return Err(PlanError::HostProviderLink {
            package: package.clone(),
            module: site.module().clone(),
            function: site.function().clone(),
            reason: Box::new(HostProviderLinkReason::CustomTypeVisibility { custom_type: name }),
        });
    }
    let expected = host_custom_type_schema(definition);
    if actual != &expected {
        return Err(PlanError::HostProviderLink {
            package: package.clone(),
            module: site.module().clone(),
            function: site.function().clone(),
            reason: Box::new(HostProviderLinkReason::CustomSchemaMismatch {
                expected,
                actual: actual.clone(),
            }),
        });
    }
    for shape in signature
        .shape()
        .argument_shapes()
        .iter()
        .chain([signature.shape().return_shape()])
    {
        if let Some(actual) =
            invalid_host_custom_type_argument_count(shape, &name, definition.parameters().len())
        {
            return Err(PlanError::HostProviderLink {
                package: package.clone(),
                module: site.module().clone(),
                function: site.function().clone(),
                reason: Box::new(HostProviderLinkReason::CustomTypeArgumentCount {
                    custom_type: name,
                    expected: definition.parameters().len(),
                    actual,
                }),
            });
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum HostCustomTypeAccess {
    SourceDeclaration,
    SourceLessPublicSurface,
}

fn invalid_host_custom_type_argument_count(
    shape: &crate::plan::ValueShape,
    custom_type: &crate::plan::CustomTypeName,
    expected: usize,
) -> Option<usize> {
    let mut pending = vec![shape];
    while let Some(shape) = pending.pop() {
        match shape {
            crate::plan::ValueShape::Tuple(elements) => {
                pending.extend(elements.iter().rev());
            }
            crate::plan::ValueShape::List(item) => pending.push(item),
            crate::plan::ValueShape::Custom(custom) => {
                if custom.type_name() == custom_type && custom.arguments().len() != expected {
                    return Some(custom.arguments().len());
                }
                pending.extend(custom.arguments().iter().rev());
            }
            crate::plan::ValueShape::Parameter(_)
            | crate::plan::ValueShape::Int
            | crate::plan::ValueShape::Float
            | crate::plan::ValueShape::String
            | crate::plan::ValueShape::BitArray
            | crate::plan::ValueShape::UtfCodepoint
            | crate::plan::ValueShape::Bool
            | crate::plan::ValueShape::Nil
            | crate::plan::ValueShape::Function(_) => {}
        }
    }
    None
}

fn host_custom_type_definition<'registry>(
    registry: &'registry ProgramRegistry,
    package: &EcoString,
    site: &crate::plan::HostCallSite,
    name: &crate::plan::CustomTypeName,
) -> Result<&'registry crate::plan::CustomTypeDefinition, PlanError> {
    registry
        .custom_type(name)
        .ok_or_else(|| PlanError::HostProviderLink {
            package: package.clone(),
            module: site.module().clone(),
            function: site.function().clone(),
            reason: Box::new(HostProviderLinkReason::MissingCustomType {
                custom_type: name.clone(),
            }),
        })
}

fn host_custom_type_schema(
    definition: &crate::plan::CustomTypeDefinition,
) -> crate::host::HostCustomTypeSchema {
    crate::host::HostCustomTypeSchema::new(
        definition.name().package().clone(),
        definition.name().module().clone(),
        definition.name().name().clone(),
        definition.parameters().len(),
        definition.constructors().iter().map(|constructor| {
            crate::host::HostCustomConstructorSchema::new(
                constructor.name().clone(),
                constructor.fields().iter().map(|field| {
                    crate::host::HostCustomFieldSchema::new(
                        field.label().cloned(),
                        host_schema_type(field.type_()),
                    )
                }),
            )
        }),
    )
}

fn host_schema_type(type_: &crate::plan::CustomTypeTemplate) -> crate::host::HostSchemaType {
    use crate::host::HostSchemaType as H;
    use crate::plan::CustomTypeTemplate as T;

    match type_ {
        T::Int => H::Int,
        T::Float => H::Float,
        T::String => H::String,
        T::BitArray => H::BitArray,
        T::UtfCodepoint => H::UtfCodepoint,
        T::Bool => H::Bool,
        T::Nil => H::Nil,
        T::Tuple(elements) => H::tuple(elements.iter().map(host_schema_type)),
        T::List(item) => H::list(host_schema_type(item)),
        T::Function { arguments, return_ } => H::function(
            arguments.iter().map(host_schema_type),
            host_schema_type(return_),
        ),
        T::Custom { name, arguments } => H::custom(
            name.package().clone(),
            name.module().clone(),
            name.name().clone(),
            arguments.iter().map(host_schema_type),
        ),
        T::Parameter(parameter) => H::parameter(parameter.0),
    }
}

fn reserve_hosted_module_constants(
    module: LinkedModule,
) -> Result<(ModuleRegistry, ModuleWithConstants), PlanError> {
    reserve_constants(module.id, module.constants).map(|constants| {
        let (constant_signatures, constant_bodies) = constants.into_parts();
        (
            ModuleRegistry::new(
                module.module_name,
                module.custom_types.clone(),
                module.functions_by_name,
                constant_signatures,
            )
            .with_executable_externals(module.executable_externals),
            ModuleWithConstants {
                id: module.id,
                package: module.package,
                source_context: module.source_context,
                custom_types: module.custom_types,
                functions: module.functions,
                constants: constant_bodies,
                anonymous_functions: module.anonymous_functions,
            },
        )
    })
}

fn plan_hosted_constant_bodies(
    registry: ProgramRegistry,
    modules: Vec<ModuleWithConstants>,
) -> Result<(ProgramRegistry, Vec<ModuleToPlan>), PlanError> {
    modules
        .into_iter()
        .map(|mut module| {
            plan_constant_bodies(module.constants, &registry, &mut module.anonymous_functions).map(
                |constants| ModuleToPlan {
                    id: module.id,
                    package: module.package,
                    source_context: module.source_context,
                    custom_types: module.custom_types,
                    functions: module.functions,
                    constants,
                    anonymous_functions: module.anonymous_functions,
                },
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|modules| (registry, modules))
}

fn plan_hosted_modules(
    root: ModuleId,
    registry: &ProgramRegistry,
    modules: Vec<ModuleToPlan>,
) -> Result<PlannedHostedProgram, PlanError> {
    let mut implementations = Vec::new();
    modules
        .into_iter()
        .map(|module| plan_hosted_module(module, registry, &mut implementations))
        .collect::<Result<Vec<_>, _>>()
        .map(|modules| PlannedHostedProgram {
            root,
            entry: FunctionTemplateId::in_module(root, 0),
            modules,
            implementations,
        })
}

fn plan_hosted_module(
    mut module: ModuleToPlan,
    registry: &ProgramRegistry,
    implementations: &mut Vec<(FunctionTemplateId, RegisteredHostImplementationId)>,
) -> Result<HostedPlannedModule, PlanError> {
    module
        .functions
        .into_iter()
        .map(|function| {
            plan_hosted_function(
                module.id,
                function,
                registry,
                &mut module.anonymous_functions,
                implementations,
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|mut functions| {
            functions.sort_by_key(|function| match function {
                HostedFunctionTemplate::GleamBody(function) => function.id().index(),
                HostedFunctionTemplate::HostTemplate(function) => function.id().index(),
            });
            HostedPlannedModule::new(HostedPlannedModuleParts {
                id: module.id,
                package: module.package,
                module: registry.module_name(module.id).clone(),
                source_context: module.source_context,
                custom_types: module.custom_types,
                constants: module.constants,
                functions,
                anonymous_functions: module.anonymous_functions.into_functions(),
            })
        })
}

fn plan_hosted_function(
    module: ModuleId,
    function: LinkedFunction,
    registry: &ProgramRegistry,
    anonymous_functions: &mut super::AnonymousFunctions,
    implementations: &mut Vec<(FunctionTemplateId, RegisteredHostImplementationId)>,
) -> Result<HostedFunctionTemplate, PlanError> {
    match function {
        LinkedFunction::Gleam { info, function } => plan_function(
            info,
            function,
            PlanContext::new_in_program(module, registry, anonymous_functions),
        )
        .map(|function| HostedFunctionTemplate::GleamBody(Box::new(function))),
        LinkedFunction::ExternalFallback {
            name,
            info,
            function,
        } => plan_selected_external_fallback(
            name,
            info,
            function,
            PlanContext::new_in_program(module, registry, anonymous_functions),
        )
        .map(|function| HostedFunctionTemplate::GleamBody(Box::new(function))),
        LinkedFunction::Host {
            template,
            implementation,
        } => {
            implementations.push((template.id(), implementation));
            Ok(HostedFunctionTemplate::HostTemplate(Box::new(template)))
        }
    }
}

struct PlannedHostedProgram {
    root: ModuleId,
    entry: FunctionTemplateId,
    modules: Vec<HostedPlannedModule>,
    implementations: Vec<(FunctionTemplateId, RegisteredHostImplementationId)>,
}

enum HostedModuleDeclaration {
    Source {
        id: ModuleId,
        package: EcoString,
        module_name: EcoString,
        source_context: Option<SourceContext>,
        custom_types: Vec<crate::plan::CustomTypeDefinition>,
        functions: Vec<TypedFunction>,
        constants: Vec<gleam_core::ast::TypedModuleConstant>,
        providers: Vec<RegisteredHostFunction>,
    },
    Host {
        id: ModuleId,
        package: EcoString,
        module_name: EcoString,
        functions: Vec<RegisteredHostFunction>,
    },
}

struct LinkedModule {
    id: ModuleId,
    package: EcoString,
    module_name: EcoString,
    source_context: Option<SourceContext>,
    custom_types: Vec<crate::plan::CustomTypeDefinition>,
    functions_by_name: HashMap<EcoString, FunctionInfo>,
    functions: Vec<LinkedFunction>,
    executable_externals: HashSet<EcoString>,
    constants: Vec<gleam_core::ast::TypedModuleConstant>,
    anonymous_functions: super::AnonymousFunctions,
}

enum LinkedFunction {
    Gleam {
        info: FunctionInfo,
        function: TypedFunction,
    },
    ExternalFallback {
        name: EcoString,
        info: FunctionInfo,
        function: TypedFunction,
    },
    Host {
        template: HostFunctionTemplate,
        implementation: RegisteredHostImplementationId,
    },
}

struct ModuleWithConstants {
    id: ModuleId,
    package: EcoString,
    source_context: Option<SourceContext>,
    custom_types: Vec<crate::plan::CustomTypeDefinition>,
    functions: Vec<LinkedFunction>,
    constants: constant::ConstantBodies,
    anonymous_functions: super::AnonymousFunctions,
}

struct ModuleToPlan {
    id: ModuleId,
    package: EcoString,
    source_context: Option<SourceContext>,
    custom_types: Vec<crate::plan::CustomTypeDefinition>,
    functions: Vec<LinkedFunction>,
    constants: ConstantTemplates,
    anonymous_functions: super::AnonymousFunctions,
}

fn bind_source_host_function(
    package: EcoString,
    module: EcoString,
    definition: RegisteredHostFunction,
    source: &FunctionInfo,
) -> Result<(HostFunctionTemplate, RegisteredHostImplementationId), PlanError> {
    let (schema, implementation) = definition.into_parts();
    let registered_shape = host_function_shape(&schema);
    if source.signature.scheme() != schema.scheme() || source.signature.shape() != &registered_shape
    {
        return Err(PlanError::HostProviderLink {
            package,
            module,
            function: schema.name().clone(),
            reason: Box::new(HostProviderLinkReason::SchemeMismatch {
                expected_scheme: source.signature.scheme().clone(),
                expected_type: source.signature.shape().type_(),
                actual_scheme: schema.scheme().clone(),
                actual_type: registered_shape.type_(),
            }),
        });
    }
    let site =
        crate::plan::HostCallSite::new(module, schema.name().clone(), source.definition_span);
    let template =
        HostFunctionTemplate::from_schema(source.signature.clone(), package, site, schema);
    Ok((template, implementation))
}

fn bind_source_less_host_function(
    id: FunctionTemplateId,
    package: EcoString,
    module: EcoString,
    definition: RegisteredHostFunction,
) -> (HostFunctionTemplate, RegisteredHostImplementationId) {
    let (schema, implementation) = definition.into_parts();
    let registered_shape = host_function_shape(&schema);
    let signature =
        crate::plan::FunctionTemplateSignature::new(id, schema.scheme().clone(), registered_shape);
    let site = crate::plan::HostCallSite::new(
        module,
        schema.name().clone(),
        crate::plan::SourceSpan::new(0, 0),
    );
    let template = HostFunctionTemplate::from_schema(signature, package, site, schema);
    (template, implementation)
}

fn host_function_shape(schema: &HostFunctionSchema) -> crate::plan::FunctionShape {
    crate::plan::FunctionShape::new(
        schema
            .parameters()
            .iter()
            .map(crate::host::HostTypeDescriptor::value_shape)
            .collect(),
        schema.return_type().value_shape(),
    )
}

fn host_function_info(template: &HostFunctionTemplate) -> FunctionInfo {
    let shape = template.signature().shape();
    FunctionInfo {
        signature: template.signature().clone(),
        type_parameters: TypeParameterScope::default(),
        return_shape: shape.return_shape().clone(),
        params: discarded_function_params(shape.argument_shapes()),
        definition_span: template.site().span(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostCustomTypeAccess, host_schema_type, plan_host_program, validate_host_custom_schema,
    };
    use crate::frontend::{ModuleSource, PackageSource, compile_typed_host_program};
    use crate::host::{
        HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema, HostModule,
        HostParameter, HostProviderModule, HostProviderSet, HostSchemaType, StatelessHostProfile,
    };
    use crate::plan::{
        CustomConstructorDefinition, CustomConstructorRefinement, CustomFieldDefinition,
        CustomTypeDefinition, CustomTypeName, CustomTypeParameterId, CustomTypePublicity,
        CustomTypeTemplate, CustomValueShape, FunctionShape, FunctionTemplateId,
        FunctionTemplateSignature, FunctionType, HostCallSite, ModuleId, SourceSpan,
        TypeParameterId, TypeScheme, ValueShape, ValueType,
    };
    use crate::planner::module::constant::ConstantSignatures;
    use crate::planner::module::registry::{ModuleRegistry, ProgramRegistry};
    use crate::planner::{HostProviderLinkReason, PlanError, UnsupportedFunctionReason};
    use ecow::EcoString;
    use num_bigint::BigInt;

    #[test]
    fn maps_every_planned_custom_field_type_to_the_exact_host_schema() {
        let custom_name = CustomTypeName::new("domain".into(), "domain/tree".into(), "Tree".into());
        let cases = [
            (CustomTypeTemplate::Int, HostSchemaType::Int),
            (CustomTypeTemplate::Float, HostSchemaType::Float),
            (CustomTypeTemplate::String, HostSchemaType::String),
            (CustomTypeTemplate::BitArray, HostSchemaType::BitArray),
            (
                CustomTypeTemplate::UtfCodepoint,
                HostSchemaType::UtfCodepoint,
            ),
            (CustomTypeTemplate::Bool, HostSchemaType::Bool),
            (CustomTypeTemplate::Nil, HostSchemaType::Nil),
            (
                CustomTypeTemplate::Tuple(vec![CustomTypeTemplate::Int, CustomTypeTemplate::Bool]),
                HostSchemaType::tuple([HostSchemaType::Int, HostSchemaType::Bool]),
            ),
            (
                CustomTypeTemplate::List(Box::new(CustomTypeTemplate::String)),
                HostSchemaType::list(HostSchemaType::String),
            ),
            (
                CustomTypeTemplate::Function {
                    arguments: vec![CustomTypeTemplate::Int, CustomTypeTemplate::Bool],
                    return_: Box::new(CustomTypeTemplate::String),
                },
                HostSchemaType::function(
                    [HostSchemaType::Int, HostSchemaType::Bool],
                    HostSchemaType::String,
                ),
            ),
            (
                CustomTypeTemplate::Custom {
                    name: custom_name.clone(),
                    arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
                },
                HostSchemaType::custom(
                    "domain",
                    "domain/tree",
                    "Tree",
                    [HostSchemaType::parameter(0)],
                ),
            ),
            (
                CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                HostSchemaType::parameter(1),
            ),
        ];

        for (template, expected) in cases {
            assert_eq!(host_schema_type(&template), expected);
        }
    }

    #[test]
    fn source_less_host_custom_type_requires_a_planned_definition() {
        let package = EcoString::from("application");
        let site = HostCallSite::new("host/custom".into(), "accept".into(), SourceSpan::new(0, 0));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        let actual = HostCustomTypeSchema::new("application", "domain/missing", "Missing", 0, []);
        let registry = ProgramRegistry::new(Vec::new());

        assert_eq!(
            validate_host_custom_schema(
                &registry,
                &package,
                &site,
                &signature,
                &actual,
                HostCustomTypeAccess::SourceLessPublicSurface,
            )
            .err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "host/custom".into(),
                function: "accept".into(),
                reason: Box::new(HostProviderLinkReason::MissingCustomType {
                    custom_type: CustomTypeName::new(
                        "application".into(),
                        "domain/missing".into(),
                        "Missing".into(),
                    ),
                }),
            }),
        );
    }

    #[test]
    fn validates_a_custom_schema_referenced_through_a_nested_type_argument() {
        let custom_type = CustomTypeName::new("application".into(), "main".into(), "Boxed".into());
        let definition = CustomTypeDefinition::new(
            custom_type.clone(),
            CustomTypePublicity::Public,
            false,
            vec![CustomTypeParameterId(0)],
            vec![CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![CustomFieldDefinition::new(
                    Some("value".into()),
                    CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                )],
            )],
        );
        let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
            "main".into(),
            vec![definition],
            std::collections::HashMap::new(),
            ConstantSignatures::default(),
        )]);
        let schema = HostCustomTypeSchema::new(
            "application",
            "main",
            "Boxed",
            1,
            [HostCustomConstructorSchema::new(
                "Boxed",
                [HostCustomFieldSchema::new(
                    Some("value"),
                    HostSchemaType::parameter(0),
                )],
            )],
        );
        let parameter = TypeParameterId(0);
        let shape = FunctionShape::new(
            vec![ValueShape::List(Box::new(ValueShape::Custom(
                CustomValueShape::new(
                    custom_type,
                    vec![ValueShape::Parameter(parameter)],
                    CustomConstructorRefinement::Any,
                ),
            )))],
            ValueShape::Bool,
        );
        let module = ModuleId::new(0);
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(module, 0),
            TypeScheme::new(1),
            shape,
        );
        let package = EcoString::from("application");
        let site = HostCallSite::new("main".into(), "accept".into(), SourceSpan::new(0, 0));

        assert_eq!(
            validate_host_custom_schema(
                &registry,
                &package,
                &site,
                &signature,
                &schema,
                HostCustomTypeAccess::SourceLessPublicSurface,
            ),
            Ok(()),
        );
    }

    #[test]
    fn source_provider_rejects_a_custom_schema_mismatch_before_shape_validation() {
        let custom_type = CustomTypeName::new(
            "application".into(),
            "domain/marker".into(),
            "Marker".into(),
        );
        let definition = CustomTypeDefinition::new(
            custom_type,
            CustomTypePublicity::Public,
            false,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Expected".into(),
                0,
                Vec::new(),
            )],
        );
        let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
            "domain/marker".into(),
            vec![definition],
            std::collections::HashMap::new(),
            ConstantSignatures::default(),
        )]);
        let package = EcoString::from("application");
        let site = HostCallSite::new("host/marker".into(), "accept".into(), SourceSpan::new(0, 0));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        let actual = HostCustomTypeSchema::new(
            "application",
            "domain/marker",
            "Marker",
            0,
            [HostCustomConstructorSchema::new(
                "Actual",
                Vec::<HostCustomFieldSchema>::new(),
            )],
        );

        assert_eq!(
            validate_host_custom_schema(
                &registry,
                &package,
                &site,
                &signature,
                &actual,
                HostCustomTypeAccess::SourceDeclaration,
            )
            .err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "host/marker".into(),
                function: "accept".into(),
                reason: Box::new(HostProviderLinkReason::CustomSchemaMismatch {
                    expected: HostCustomTypeSchema::new(
                        "application",
                        "domain/marker",
                        "Marker",
                        0,
                        [HostCustomConstructorSchema::new(
                            "Expected",
                            Vec::<HostCustomFieldSchema>::new(),
                        )],
                    ),
                    actual,
                }),
            }),
        );
    }

    #[test]
    fn host_custom_types_preserve_source_visibility() {
        let custom_type =
            CustomTypeName::new("domain".into(), "domain/marker".into(), "Marker".into());
        let actual = HostCustomTypeSchema::new(
            "domain",
            "domain/marker",
            "Marker",
            0,
            [HostCustomConstructorSchema::new(
                "Marker",
                Vec::<HostCustomFieldSchema>::new(),
            )],
        );
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        let cases = [
            (
                CustomTypePublicity::Public,
                false,
                EcoString::from("application"),
                EcoString::from("host/custom"),
                Ok(()),
            ),
            (
                CustomTypePublicity::Internal,
                false,
                EcoString::from("domain"),
                EcoString::from("host/custom"),
                Ok(()),
            ),
            (
                CustomTypePublicity::Private,
                false,
                EcoString::from("domain"),
                EcoString::from("domain/marker"),
                Ok(()),
            ),
            (
                CustomTypePublicity::Public,
                true,
                EcoString::from("domain"),
                EcoString::from("domain/marker"),
                Ok(()),
            ),
            (
                CustomTypePublicity::Internal,
                false,
                EcoString::from("application"),
                EcoString::from("host/custom"),
                Err(PlanError::HostProviderLink {
                    package: "application".into(),
                    module: "host/custom".into(),
                    function: "accept".into(),
                    reason: Box::new(HostProviderLinkReason::CustomTypeVisibility {
                        custom_type: custom_type.clone(),
                    }),
                }),
            ),
            (
                CustomTypePublicity::Private,
                false,
                EcoString::from("domain"),
                EcoString::from("host/custom"),
                Err(PlanError::HostProviderLink {
                    package: "domain".into(),
                    module: "host/custom".into(),
                    function: "accept".into(),
                    reason: Box::new(HostProviderLinkReason::CustomTypeVisibility {
                        custom_type: custom_type.clone(),
                    }),
                }),
            ),
            (
                CustomTypePublicity::Public,
                true,
                EcoString::from("domain"),
                EcoString::from("host/custom"),
                Err(PlanError::HostProviderLink {
                    package: "domain".into(),
                    module: "host/custom".into(),
                    function: "accept".into(),
                    reason: Box::new(HostProviderLinkReason::CustomTypeVisibility {
                        custom_type: custom_type.clone(),
                    }),
                }),
            ),
        ];

        for (publicity, opaque, package, module, expected) in cases {
            let definition = CustomTypeDefinition::new(
                custom_type.clone(),
                publicity,
                opaque,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Marker".into(),
                    0,
                    Vec::new(),
                )],
            );
            let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
                "domain/marker".into(),
                vec![definition],
                std::collections::HashMap::new(),
                ConstantSignatures::default(),
            )]);
            let site = HostCallSite::new(module, "accept".into(), SourceSpan::new(0, 0));

            assert_eq!(
                validate_host_custom_schema(
                    &registry,
                    &package,
                    &site,
                    &signature,
                    &actual,
                    HostCustomTypeAccess::SourceDeclaration,
                ),
                expected,
            );
        }
    }

    #[test]
    fn source_less_host_custom_types_require_a_public_non_opaque_surface() {
        let custom_type =
            CustomTypeName::new("domain".into(), "domain/marker".into(), "Marker".into());
        let actual = HostCustomTypeSchema::new(
            "domain",
            "domain/marker",
            "Marker",
            0,
            [HostCustomConstructorSchema::new(
                "Marker",
                Vec::<HostCustomFieldSchema>::new(),
            )],
        );
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );

        for (publicity, opaque) in [
            (CustomTypePublicity::Internal, false),
            (CustomTypePublicity::Private, false),
            (CustomTypePublicity::Public, true),
        ] {
            let definition = CustomTypeDefinition::new(
                custom_type.clone(),
                publicity,
                opaque,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Marker".into(),
                    0,
                    Vec::new(),
                )],
            );
            let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
                "domain/marker".into(),
                vec![definition],
                std::collections::HashMap::new(),
                ConstantSignatures::default(),
            )]);
            let package = EcoString::from("domain");
            let site =
                HostCallSite::new("host/custom".into(), "accept".into(), SourceSpan::new(0, 0));

            assert_eq!(
                validate_host_custom_schema(
                    &registry,
                    &package,
                    &site,
                    &signature,
                    &actual,
                    HostCustomTypeAccess::SourceLessPublicSurface,
                ),
                Err(PlanError::HostProviderLink {
                    package: "domain".into(),
                    module: "host/custom".into(),
                    function: "accept".into(),
                    reason: Box::new(HostProviderLinkReason::CustomTypeVisibility {
                        custom_type: custom_type.clone(),
                    }),
                }),
            );
        }
    }

    #[test]
    fn opaque_custom_type_visibility_precedes_schema_matching() {
        let custom_type =
            CustomTypeName::new("domain".into(), "domain/marker".into(), "Marker".into());
        let definition = CustomTypeDefinition::new(
            custom_type.clone(),
            CustomTypePublicity::Public,
            true,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Expected".into(),
                0,
                Vec::new(),
            )],
        );
        let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
            "domain/marker".into(),
            vec![definition],
            std::collections::HashMap::new(),
            ConstantSignatures::default(),
        )]);
        let package = EcoString::from("domain");
        let site = HostCallSite::new("host/custom".into(), "accept".into(), SourceSpan::new(0, 0));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        let actual = HostCustomTypeSchema::new(
            "domain",
            "domain/marker",
            "Marker",
            0,
            [HostCustomConstructorSchema::new(
                "Actual",
                Vec::<HostCustomFieldSchema>::new(),
            )],
        );

        assert_eq!(
            validate_host_custom_schema(
                &registry,
                &package,
                &site,
                &signature,
                &actual,
                HostCustomTypeAccess::SourceDeclaration,
            ),
            Err(PlanError::HostProviderLink {
                package: "domain".into(),
                module: "host/custom".into(),
                function: "accept".into(),
                reason: Box::new(HostProviderLinkReason::CustomTypeVisibility { custom_type }),
            }),
        );
    }

    #[test]
    fn source_less_host_custom_type_applies_every_declared_type_argument() {
        let custom_type =
            CustomTypeName::new("application".into(), "domain/box".into(), "Boxed".into());
        let definition = CustomTypeDefinition::new(
            custom_type.clone(),
            CustomTypePublicity::Public,
            false,
            vec![CustomTypeParameterId(0)],
            vec![CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![CustomFieldDefinition::new(
                    None,
                    CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                )],
            )],
        );
        let registry = ProgramRegistry::new(vec![ModuleRegistry::new(
            "domain/box".into(),
            vec![definition],
            std::collections::HashMap::new(),
            ConstantSignatures::default(),
        )]);
        let package = EcoString::from("application");
        let site = HostCallSite::new("host/box".into(), "accept".into(), SourceSpan::new(0, 0));
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
            TypeScheme::new(0),
            FunctionShape::new(
                vec![ValueShape::Tuple(
                    vec![ValueShape::Custom(CustomValueShape::new(
                        custom_type.clone(),
                        Vec::new(),
                        CustomConstructorRefinement::Any,
                    ))]
                    .into_boxed_slice(),
                )],
                ValueShape::Bool,
            ),
        );
        let actual = HostCustomTypeSchema::new(
            "application",
            "domain/box",
            "Boxed",
            1,
            [HostCustomConstructorSchema::new(
                "Boxed",
                [HostCustomFieldSchema::new(
                    None::<EcoString>,
                    HostSchemaType::parameter(0),
                )],
            )],
        );

        assert_eq!(
            validate_host_custom_schema(
                &registry,
                &package,
                &site,
                &signature,
                &actual,
                HostCustomTypeAccess::SourceLessPublicSurface,
            )
            .err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "host/box".into(),
                function: "accept".into(),
                reason: Box::new(HostProviderLinkReason::CustomTypeArgumentCount {
                    custom_type: CustomTypeName::new(
                        "application".into(),
                        "domain/box".into(),
                        "Boxed".into(),
                    ),
                    expected: 1,
                    actual: 0,
                }),
            }),
        );
    }

    #[test]
    fn plan_host_program_bodyless_templates_with_module_qualified_ids() {
        let choose = |condition: bool, left: BigInt, right: BigInt| {
            if condition { left } else { right }
        };
        assert_eq!(
            choose(false, BigInt::from(10), BigInt::from(20)),
            BigInt::from(20),
        );
        assert_eq!(
            choose(true, BigInt::from(10), BigInt::from(20)),
            BigInt::from(10),
        );
        let all = |a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool| {
            a && b && c && d && e && f && g
        };
        assert!(all(true, true, true, true, true, true, true));

        let hosts = HostProviderSet::new([HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")
            .with_function("subtract", <BigInt as std::ops::Sub>::sub)
            .expect("host function should be valid")
            .with_function("ready", <bool as Default>::default)
            .expect("host function should be valid")
            .with_function("choose", choose)
            .expect("host function should be valid")
            .with_function("all", all)
            .expect("host function should be valid")
            .with_function(
                "consume",
                |_: BigInt,
                 _: f64,
                 _: EcoString,
                 _: crate::BitArrayValue,
                 _: char,
                 _: bool,
                 (): ()| (),
            )
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
import host/math.{add}

pub fn main() {
  add(1, 2)
}
"#,
                )],
            )],
            hosts,
        )
        .expect("host program should compile");
        let plan = plan_host_program(typed).expect("host program should plan");

        assert_eq!(plan.root(), ModuleId::new(1));
        assert_eq!(
            plan.entry(),
            FunctionTemplateId::in_module(ModuleId::new(1), 0)
        );
        assert_eq!(
            plan.modules()
                .iter()
                .map(|module| (module.package().as_str(), module.module().as_str()))
                .collect::<Vec<_>>(),
            [("host_support", "host/math"), ("application", "main")],
        );
        assert_eq!(plan.modules()[0].id(), ModuleId::new(0));
        assert_eq!(plan.modules()[1].id(), ModuleId::new(1));
        assert!(plan.modules()[0].source_context().is_none());
        assert!(plan.modules()[1].source_context().is_some());
        let host = &plan.modules()[0];
        assert_eq!(host.id(), ModuleId::new(0));
        assert_eq!(host.functions().len(), 6);
        let functions = host
            .functions()
            .iter()
            .map(|function| {
                function
                    .host_template()
                    .expect("source-less module should retain host templates")
            })
            .collect::<Vec<_>>();
        assert_eq!(functions[0].name(), "add");
        assert_eq!(
            functions[0].id(),
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
        );
        assert_eq!(functions[0].package(), "host_support");
        assert_eq!(functions[0].module(), "host/math");
        assert_eq!(functions[0].scheme().parameters(), &[]);
        assert_eq!(
            functions[0].signature().shape(),
            &FunctionShape::new(vec![ValueShape::Int, ValueShape::Int], ValueShape::Int,),
        );
        assert_eq!(
            functions[0].type_(),
            &FunctionType::new(vec![ValueType::Int, ValueType::Int], ValueType::Int),
        );
        assert!(matches!(
            functions[0].layout(),
            [HostParameter::Int(left), HostParameter::Int(right)]
                if left.index() == 0 && right.index() == 1
        ));
        assert_eq!(functions[1].name(), "subtract");
        assert_eq!(functions[2].name(), "ready");
        assert_eq!(
            functions[2].signature().shape(),
            &FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        assert_eq!(
            functions[2].type_(),
            &FunctionType::new(Vec::new(), ValueType::Bool),
        );
        assert_eq!(functions[3].name(), "choose");
        assert_eq!(
            functions[3].signature().shape(),
            &FunctionShape::new(
                vec![ValueShape::Bool, ValueShape::Int, ValueShape::Int],
                ValueShape::Int,
            ),
        );
        assert_eq!(
            functions[3].type_(),
            &FunctionType::new(
                vec![ValueType::Bool, ValueType::Int, ValueType::Int],
                ValueType::Int,
            ),
        );
        assert!(matches!(
            functions[3].layout(),
            [
                HostParameter::Bool(condition),
                HostParameter::Int(left),
                HostParameter::Int(right),
            ] if condition.index() == 0 && left.index() == 0 && right.index() == 1
        ));
        assert_eq!(functions[4].name(), "all");
        assert_eq!(
            functions[4].signature().shape(),
            &FunctionShape::new(vec![ValueShape::Bool; 7], ValueShape::Bool),
        );
        assert_eq!(
            functions[4].type_(),
            &FunctionType::new(vec![ValueType::Bool; 7], ValueType::Bool),
        );
        assert!(matches!(
            functions[4].layout(),
            [
                HostParameter::Bool(first),
                HostParameter::Bool(second),
                HostParameter::Bool(third),
                HostParameter::Bool(fourth),
                HostParameter::Bool(fifth),
                HostParameter::Bool(sixth),
                HostParameter::Bool(seventh),
            ] if first.index() == 0
                && second.index() == 1
                && third.index() == 2
                && fourth.index() == 3
                && fifth.index() == 4
                && sixth.index() == 5
                && seventh.index() == 6
        ));
        assert_eq!(functions[5].name(), "consume");
        assert_eq!(
            functions[5].signature().shape(),
            &FunctionShape::new(
                vec![
                    ValueShape::Int,
                    ValueShape::Float,
                    ValueShape::String,
                    ValueShape::BitArray,
                    ValueShape::UtfCodepoint,
                    ValueShape::Bool,
                    ValueShape::Nil,
                ],
                ValueShape::Nil,
            ),
        );
        assert_eq!(
            functions[5].type_(),
            &FunctionType::new(
                vec![
                    ValueType::Int,
                    ValueType::Float,
                    ValueType::String,
                    ValueType::BitArray,
                    ValueType::UtfCodepoint,
                    ValueType::Bool,
                    ValueType::Nil,
                ],
                ValueType::Nil,
            ),
        );
        assert!(matches!(
            functions[5].layout(),
            [
                HostParameter::Int(int),
                HostParameter::Float(float),
                HostParameter::String(string),
                HostParameter::BitArray(bit_array),
                HostParameter::UtfCodepoint(utf_codepoint),
                HostParameter::Bool(bool_),
                HostParameter::Nil(nil),
            ] if int.index() == 0
                && float.index() == 0
                && string.index() == 0
                && bit_array.index() == 0
                && utf_codepoint.index() == 0
                && bool_.index() == 0
                && nil.index() == 0
        ));
        let source = plan.modules()[1].functions()[0]
            .gleam_body()
            .expect("root module should retain its source function");
        assert_eq!(source.name(), "main");
    }

    #[test]
    fn plan_host_program_source_dependencies_as_dependency_modules() {
        let typed = compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["library"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        "pub fn main() { 1 }",
                    )],
                ),
                PackageSource::new(
                    "library",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "support",
                        "support.gleam",
                        "pub fn unused() { 2 }",
                    )],
                ),
            ],
            HostProviderSet::new(Vec::<HostModule>::new())
                .expect("empty host modules should be valid"),
        )
        .expect("hosted source program should compile");
        let plan = plan_host_program(typed).expect("hosted source program should plan");

        assert_eq!(plan.root(), ModuleId::new(1));
        assert_eq!(
            plan.modules()
                .iter()
                .map(|module| (module.package().as_str(), module.module().as_str()))
                .collect::<Vec<_>>(),
            [("library", "support"), ("application", "main")],
        );
        assert_eq!(
            plan.modules()[0].functions()[0]
                .gleam_body()
                .expect("dependency should remain a source function")
                .name(),
            "unused",
        );
    }

    #[test]
    fn source_provider_and_gleam_fallback_keep_distinct_body_owners() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("provided", std::convert::identity::<BigInt>)
            .expect("provider function should be valid");
        let source = r#"
@external(erlang, "host", "provided")
fn provided(value: Int) -> Int

@external(erlang, "host", "fallback")
fn fallback(value: Int) -> Int {
  value + 1
}

pub fn main() {
  #(provided(1), fallback(2))
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("source should compile");
        let plan = plan_host_program(typed).expect("provider and fallback should plan");
        let functions = plan.modules()[0].functions();

        assert_eq!(
            functions
                .iter()
                .map(|function| {
                    (
                        function
                            .gleam_body()
                            .map(|function| function.name().as_str()),
                        function
                            .host_template()
                            .map(|function| function.name().as_str()),
                    )
                })
                .collect::<Vec<_>>(),
            [
                (Some("main"), None),
                (None, Some("provided")),
                (Some("fallback"), None),
            ],
        );
    }

    #[test]
    fn reject_profile_host_program_source_owner_boundaries() {
        let cases = [
            (
                "pub fn other() { 1 }",
                PlanError::UnsupportedFunction {
                    name: "main".into(),
                    reason: UnsupportedFunctionReason::MissingMain,
                },
            ),
            (
                r#"
@external(erlang, "external", "thing")
pub type Thing

pub fn main() { 1 }
"#,
                PlanError::UnsupportedTopLevel {
                    kind: crate::planner::UnsupportedTopLevelKind::ExternalCustomType,
                },
            ),
            (
                r#"
const unsupported = <<1:native>>

pub fn main() { 1 }
"#,
                PlanError::UnsupportedBitArraySegment {
                    reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
                },
            ),
            (
                r#"
fn unsupported() { <<1:native>> }

pub fn main() { 1 }
"#,
                PlanError::UnsupportedBitArraySegment {
                    reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
                },
            ),
        ];

        for (source, expected) in cases {
            let typed = compile_typed_host_program(
                "application",
                "main",
                [PackageSource::new(
                    "application",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new("main", "main.gleam", source)],
                )],
                HostProviderSet::new(Vec::<HostModule>::new())
                    .expect("empty host modules should be valid"),
            )
            .expect("profile-out source should still compile");
            assert_eq!(plan_host_program(typed).err(), Some(expected));
        }
    }

    #[test]
    fn reject_profile_host_program_custom_declaration_precedence() {
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [
                    ModuleSource::new("main", "main.gleam", "pub fn other() { 1 }"),
                    ModuleSource::new(
                        "zsupport",
                        "zsupport.gleam",
                        r#"
@external(erlang, "external", "thing")
pub type Thing

pub fn value() { 1 }
"#,
                    ),
                ],
            )],
            HostProviderSet::new(Vec::<HostModule>::new())
                .expect("empty host modules should be valid"),
        )
        .expect("profile-out source should still compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::UnsupportedTopLevel {
                kind: crate::planner::UnsupportedTopLevelKind::ExternalCustomType,
            }),
        );
    }

    #[test]
    fn missing_provider_precedes_source_body_planning() {
        let typed = compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["library"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
const unsupported = <<1:native>>

pub fn main() { 1 }
"#,
                    )],
                ),
                PackageSource::new(
                    "library",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "support",
                        "support.gleam",
                        r#"
@external(erlang, "support", "native")
pub fn native() -> Int
"#,
                    )],
                ),
            ],
            HostProviderSet::new(Vec::<HostModule>::new())
                .expect("empty host modules should be valid"),
        )
        .expect("profile-out source should still compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::MissingHostProvider {
                package: "library".into(),
                module: "support".into(),
                function: "native".into(),
            }),
        );
    }

    #[test]
    fn selected_external_fallback_body_preserves_its_planning_error() {
        let source = r#"
@external(erlang, "host", "fallback")
fn fallback() -> BitArray {
  <<1:native>>
}

pub fn main() {
  1
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::new(Vec::<HostModule>::new())
                .expect("empty host modules should be valid"),
        )
        .expect("external fallback should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
            }),
        );
    }

    #[test]
    fn provider_module_must_link_to_a_source_module() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "missing")
            .expect("provider module should be valid")
            .with_function("native", BigInt::default)
            .expect("provider function should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { 1 }",
                )],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("host program should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "missing".into(),
                function: "native".into(),
                reason: Box::new(HostProviderLinkReason::MissingModule),
            }),
        );
    }

    #[test]
    fn missing_provider_module_reports_the_lexically_first_function() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "missing")
            .expect("provider module should be valid")
            .with_function("zeta", BigInt::default)
            .expect("provider function should be valid")
            .with_function("alpha", BigInt::default)
            .expect("provider function should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { 1 }",
                )],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("host program should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "missing".into(),
                function: "alpha".into(),
                reason: Box::new(HostProviderLinkReason::MissingModule),
            }),
        );
    }

    #[test]
    fn provider_function_must_link_to_a_source_declaration() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("native", BigInt::default)
            .expect("provider function should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { 1 }",
                )],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("host program should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "native".into(),
                reason: Box::new(HostProviderLinkReason::MissingDeclaration),
            }),
        );
    }

    #[test]
    fn missing_provider_declaration_reports_the_lexically_first_function() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("zeta", BigInt::default)
            .expect("provider function should be valid")
            .with_function("alpha", BigInt::default)
            .expect("provider function should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { 1 }",
                )],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("host program should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "alpha".into(),
                reason: Box::new(HostProviderLinkReason::MissingDeclaration),
            }),
        );
    }

    #[test]
    fn provider_function_cannot_override_an_ordinary_gleam_body() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("native", BigInt::default)
            .expect("provider function should be valid");
        let source = r#"
fn native() {
  1
}

pub fn main() {
  native()
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("host program should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "native".into(),
                reason: Box::new(HostProviderLinkReason::NonExternalFunction),
            }),
        );
    }

    #[test]
    fn provider_function_scheme_must_exactly_match_the_external_declaration() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("native", |value: BigInt| value)
            .expect("provider function should be valid");
        let source = r#"
@external(erlang, "host", "native")
fn native(value: Bool) -> Bool

pub fn main() {
  native(True)
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("host program should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "native".into(),
                reason: Box::new(HostProviderLinkReason::SchemeMismatch {
                    expected_scheme: crate::plan::TypeScheme::new(0),
                    expected_type: FunctionType::new(vec![ValueType::Bool], ValueType::Bool,),
                    actual_scheme: crate::plan::TypeScheme::new(0),
                    actual_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                }),
            }),
        );
    }
}
