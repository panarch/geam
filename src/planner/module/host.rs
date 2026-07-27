use super::constant::{self, plan_constant_bodies, reserve_constants};
use super::custom_type;
use super::registry::{ModuleRegistry, ProgramRegistry};
use super::{
    FunctionTable, ModuleBodies, ModuleDeclarations, ModuleFunctionDeclarations, ModuleFunctions,
    ModuleRole, function_table,
};
use crate::frontend::{HostedTypedProgram, HostedTypedProgramModule};
use crate::host::{
    HostFunctionDefinition, HostParameter as RegisteredHostParameter, HostValueType,
};
use crate::plan::{
    BoolLocalId, FunctionTemplateId, HostFunctionImplementation, HostFunctionTemplate,
    HostParameter as PlannedHostParameter, HostReturnFamily, HostedModulePlan, HostedPlannedModule,
    IntLocalId, ModuleId, ParamBinding, ParamLocal, PlannedHostModule, PlannedModule,
    SourceContext, ValueShape,
};
use crate::planner::context::{FunctionInfo, FunctionParam, PlanContext};
use crate::planner::error::PlanError;
use crate::planner::function::plan_function;
use crate::planner::type_parameter::TypeParameterScope;
use ecow::EcoString;
use std::collections::HashMap;

pub fn plan_host_program(program: HostedTypedProgram) -> Result<HostedModulePlan, PlanError> {
    let (root_index, modules) = program.into_parts();
    let root = ModuleId::new(root_index);
    let mut declarations = Vec::with_capacity(modules.len());

    for (index, module) in modules.into_iter().enumerate() {
        let id = ModuleId::new(index);
        match module {
            HostedTypedProgramModule::Source(module) => {
                let package = module.module.type_info.package.clone();
                let definitions = module.module.definitions;
                let module_name = module.module.name;
                let custom_types = custom_type::plan_custom_types(
                    &package,
                    &module_name,
                    definitions.custom_types,
                )?;
                declarations.push(HostedModuleDeclarations::Source(ModuleDeclarations {
                    id,
                    package,
                    module_name,
                    source_context: Some(SourceContext::new(module.path, module.source)),
                    custom_types,
                    functions: definitions.functions,
                    constants: definitions.constants,
                }));
            }
            HostedTypedProgramModule::Host(module) => {
                let (package, module_name, functions) = module.into_parts();
                declarations.push(HostedModuleDeclarations::Host {
                    id,
                    package,
                    module_name,
                    functions,
                });
            }
        }
    }

    let mut function_declarations = Vec::with_capacity(declarations.len());
    let mut implementations = Vec::new();
    for declaration in declarations {
        match declaration {
            HostedModuleDeclarations::Source(declaration) => {
                let role = if declaration.id == root {
                    ModuleRole::Root
                } else {
                    ModuleRole::Dependency
                };
                let FunctionTable {
                    by_name,
                    functions,
                    anonymous_functions,
                } = function_table(declaration.id, &declaration.functions, role)?;
                function_declarations.push(HostedModuleFunctionDeclarations::Source(
                    ModuleFunctionDeclarations {
                        id: declaration.id,
                        package: declaration.package,
                        module_name: declaration.module_name,
                        source_context: declaration.source_context,
                        custom_types: declaration.custom_types,
                        functions_by_name: by_name,
                        functions,
                        constants: declaration.constants,
                        anonymous_functions,
                    },
                ));
            }
            HostedModuleDeclarations::Host {
                id,
                package,
                module_name,
                functions,
            } => {
                let mut functions_by_name = HashMap::with_capacity(functions.len());
                let mut templates = Vec::with_capacity(functions.len());
                for (function_index, definition) in functions.into_iter().enumerate() {
                    let (schema, implementation) = definition.into_parts();
                    let id = FunctionTemplateId::in_module(id, function_index);
                    let mut template_params = Vec::with_capacity(schema.parameters().len());
                    let mut function_params = Vec::with_capacity(schema.parameters().len());
                    for parameter in schema.parameters() {
                        let (local, shape, template_param) = match parameter {
                            RegisteredHostParameter::Int(slot) => {
                                let local = IntLocalId(slot.index());
                                (
                                    ParamLocal::int(local),
                                    ValueShape::Int,
                                    PlannedHostParameter::Int(local),
                                )
                            }
                            RegisteredHostParameter::Bool(slot) => {
                                let local = BoolLocalId(slot.index());
                                (
                                    ParamLocal::bool(local),
                                    ValueShape::Bool,
                                    PlannedHostParameter::Bool(local),
                                )
                            }
                        };
                        template_params.push(template_param);
                        function_params.push(FunctionParam::new(
                            local,
                            shape,
                            ParamBinding::Discard,
                            None,
                        ));
                    }
                    let return_family = match schema.return_type() {
                        HostValueType::Int => HostReturnFamily::Int,
                        HostValueType::Bool => HostReturnFamily::Bool,
                    };
                    let return_shape = return_family.shape();
                    let template = HostFunctionTemplate::new(
                        id,
                        package.clone(),
                        module_name.clone(),
                        schema.name().clone(),
                        template_params,
                        return_family,
                        schema.type_().clone(),
                    );
                    functions_by_name.insert(
                        schema.name().clone(),
                        FunctionInfo {
                            signature: template.signature().clone(),
                            type_parameters: TypeParameterScope::default(),
                            return_shape,
                            params: function_params,
                        },
                    );
                    implementations.push(HostFunctionImplementation::new(id, implementation));
                    templates.push(template);
                }
                function_declarations.push(HostedModuleFunctionDeclarations::Host {
                    functions_by_name,
                    module: PlannedHostModule::new(id, package, module_name, templates),
                });
            }
        }
    }

    let mut registry_modules = Vec::with_capacity(function_declarations.len());
    let mut bodies = Vec::with_capacity(function_declarations.len());
    for declaration in function_declarations {
        match declaration {
            HostedModuleFunctionDeclarations::Source(declaration) => {
                let constants = reserve_constants(declaration.id, declaration.constants)?;
                let (constant_signatures, constant_bodies) = constants.into_parts();
                registry_modules.push(ModuleRegistry::new(
                    declaration.module_name,
                    declaration.custom_types,
                    declaration.functions_by_name,
                    constant_signatures,
                ));
                bodies.push(HostedModuleBodies::Source(ModuleBodies {
                    id: declaration.id,
                    package: declaration.package,
                    source_context: declaration.source_context,
                    functions: declaration.functions,
                    constants: constant_bodies,
                    anonymous_functions: declaration.anonymous_functions,
                }));
            }
            HostedModuleFunctionDeclarations::Host {
                functions_by_name,
                module,
            } => {
                registry_modules.push(ModuleRegistry::new(
                    module.module().clone(),
                    Vec::new(),
                    functions_by_name,
                    constant::ConstantSignatures::default(),
                ));
                bodies.push(HostedModuleBodies::Host(module));
            }
        }
    }

    let registry = ProgramRegistry::new(registry_modules);
    let mut functions_to_plan = Vec::with_capacity(bodies.len());
    for module in bodies {
        match module {
            HostedModuleBodies::Source(mut module) => {
                let constants = plan_constant_bodies(
                    module.constants,
                    &registry,
                    &mut module.anonymous_functions,
                )?;
                functions_to_plan.push(HostedModuleFunctions::Source(Box::new(ModuleFunctions {
                    id: module.id,
                    package: module.package,
                    source_context: module.source_context,
                    functions: module.functions,
                    constants,
                    anonymous_functions: module.anonymous_functions,
                })));
            }
            HostedModuleBodies::Host(module) => {
                functions_to_plan.push(HostedModuleFunctions::Host(module));
            }
        }
    }

    let mut planned_modules = Vec::with_capacity(functions_to_plan.len());
    for module in functions_to_plan {
        match module {
            HostedModuleFunctions::Source(module) => {
                let mut module = *module;
                let mut planned_functions = Vec::with_capacity(module.functions.len());
                for function in module.functions {
                    let context = PlanContext::new_in_program(
                        module.id,
                        &registry,
                        &mut module.anonymous_functions,
                    );
                    planned_functions.push(plan_function(
                        function.info,
                        function.function,
                        context,
                    )?);
                }
                planned_functions.sort_by_key(|function| function.id().index());
                planned_modules.push(HostedPlannedModule::from_source(PlannedModule::new(
                    module.id,
                    module.package,
                    crate::plan::module::PlannedModuleParts {
                        module: registry.module_name(module.id).clone(),
                        source_context: module.source_context,
                        custom_types: registry.custom_types(module.id).to_vec(),
                        constants: module.constants,
                        functions: planned_functions,
                        anonymous_functions: module.anonymous_functions.into_functions(),
                    },
                )));
            }
            HostedModuleFunctions::Host(module) => {
                planned_modules.push(HostedPlannedModule::from_host(module));
            }
        }
    }

    Ok(HostedModulePlan::new(
        root,
        FunctionTemplateId::in_module(root, 0),
        planned_modules,
        implementations,
    ))
}

enum HostedModuleDeclarations {
    Source(ModuleDeclarations),
    Host {
        id: ModuleId,
        package: EcoString,
        module_name: EcoString,
        functions: Vec<HostFunctionDefinition>,
    },
}

enum HostedModuleFunctionDeclarations {
    Source(ModuleFunctionDeclarations),
    Host {
        functions_by_name: HashMap<EcoString, FunctionInfo>,
        module: PlannedHostModule,
    },
}

enum HostedModuleBodies {
    Source(ModuleBodies),
    Host(PlannedHostModule),
}

enum HostedModuleFunctions {
    Source(Box<ModuleFunctions>),
    Host(PlannedHostModule),
}

#[cfg(test)]
mod tests {
    use super::plan_host_program;
    use crate::frontend::{ModuleSource, PackageSource, compile_typed_host_program};
    use crate::host::{HostModule, HostModules};
    use crate::plan::{
        BoolLocalId, FunctionShape, FunctionTemplateId, FunctionType, HostParameter,
        HostReturnFamily, IntLocalId, ModuleId, ValueShape, ValueType,
    };
    use crate::planner::{PlanError, UnsupportedFunctionReason};
    use ecow::EcoString;
    use num_bigint::BigInt;

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

        let hosts = HostModules::new([HostModule::new("host_support", "host/math")
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
        assert!(plan.modules()[0].source().is_none());
        assert!(plan.modules()[1].host().is_none());
        let host = plan.modules()[0]
            .host()
            .expect("host module should retain host templates");
        assert_eq!(host.id(), ModuleId::new(0));
        assert_eq!(host.functions().len(), 5);
        assert_eq!(host.functions()[0].name(), "add");
        assert_eq!(
            host.functions()[0].id(),
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
        );
        assert_eq!(host.functions()[0].package(), "host_support");
        assert_eq!(host.functions()[0].module(), "host/math");
        assert_eq!(host.functions()[0].scheme().parameters(), &[]);
        assert_eq!(
            host.functions()[0].signature().shape(),
            &FunctionShape::new(vec![ValueShape::Int, ValueShape::Int], ValueShape::Int,),
        );
        assert_eq!(
            host.functions()[0].type_(),
            &FunctionType::new(vec![ValueType::Int, ValueType::Int], ValueType::Int),
        );
        assert_eq!(host.functions()[1].name(), "subtract");
        assert_eq!(host.functions()[2].name(), "ready");
        assert_eq!(host.functions()[2].parameters(), &[]);
        assert_eq!(host.functions()[2].return_family(), HostReturnFamily::Bool,);
        assert_eq!(
            host.functions()[2].signature().shape(),
            &FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        assert_eq!(
            host.functions()[2].type_(),
            &FunctionType::new(Vec::new(), ValueType::Bool),
        );
        assert_eq!(host.functions()[3].name(), "choose");
        assert_eq!(
            host.functions()[3].parameters(),
            [
                HostParameter::Bool(BoolLocalId(0)),
                HostParameter::Int(IntLocalId(0)),
                HostParameter::Int(IntLocalId(1)),
            ],
        );
        assert_eq!(host.functions()[3].return_family(), HostReturnFamily::Int,);
        assert_eq!(
            host.functions()[3].signature().shape(),
            &FunctionShape::new(
                vec![ValueShape::Bool, ValueShape::Int, ValueShape::Int],
                ValueShape::Int,
            ),
        );
        assert_eq!(
            host.functions()[3].type_(),
            &FunctionType::new(
                vec![ValueType::Bool, ValueType::Int, ValueType::Int],
                ValueType::Int,
            ),
        );
        assert_eq!(host.functions()[4].name(), "all");
        assert_eq!(
            host.functions()[4].parameters(),
            [
                HostParameter::Bool(BoolLocalId(0)),
                HostParameter::Bool(BoolLocalId(1)),
                HostParameter::Bool(BoolLocalId(2)),
                HostParameter::Bool(BoolLocalId(3)),
                HostParameter::Bool(BoolLocalId(4)),
                HostParameter::Bool(BoolLocalId(5)),
                HostParameter::Bool(BoolLocalId(6)),
            ],
        );
        assert_eq!(host.functions()[4].return_family(), HostReturnFamily::Bool);
        assert_eq!(
            host.functions()[4].signature().shape(),
            &FunctionShape::new(vec![ValueShape::Bool; 7], ValueShape::Bool),
        );
        assert_eq!(
            host.functions()[4].type_(),
            &FunctionType::new(vec![ValueType::Bool; 7], ValueType::Bool),
        );
        let source = plan.modules()[1]
            .source()
            .expect("root module should retain its source plan");
        assert_eq!(source.functions()[0].name(), "main");
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
            HostModules::new(Vec::<HostModule>::new()).expect("empty host modules should be valid"),
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
            plan.modules()[0]
                .source()
                .expect("dependency should remain a source module")
                .functions()[0]
                .name(),
            "unused",
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
                HostModules::new(Vec::<HostModule>::new())
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
            HostModules::new(Vec::<HostModule>::new()).expect("empty host modules should be valid"),
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
    fn reject_profile_host_program_constant_body_precedence() {
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
            HostModules::new(Vec::<HostModule>::new()).expect("empty host modules should be valid"),
        )
        .expect("profile-out source should still compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
            }),
        );
    }
}
