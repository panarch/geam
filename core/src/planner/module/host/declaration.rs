use crate::frontend::HostedTypedProgramModule;
use crate::host::{HostExternalTypeSchema, RegisteredHostFunction, RegisteredHostProviderModule};
use crate::plan::{ExternalTypeDefinition, ModuleId, SourceContext};
use crate::planner::error::{HostProviderLinkReason, PlanError};
use ecow::EcoString;
use gleam_compiler_core::ast::TypedFunction;
use std::collections::BTreeMap;

pub(super) enum HostedModuleDeclaration {
    Source {
        id: ModuleId,
        package: EcoString,
        module_name: EcoString,
        source_context: Option<SourceContext>,
        custom_types: Vec<crate::plan::CustomTypeDefinition>,
        external_types: Vec<ExternalTypeDefinition>,
        functions: Vec<TypedFunction>,
        constants: Vec<gleam_compiler_core::ast::TypedModuleConstant>,
        providers: Vec<RegisteredHostFunction>,
    },
    Host {
        id: ModuleId,
        package: EcoString,
        module_name: EcoString,
        functions: Vec<RegisteredHostFunction>,
    },
}

struct RegisteredProviderItems {
    functions: Vec<RegisteredHostFunction>,
    external_types: Vec<HostExternalTypeSchema>,
}

pub(super) fn collect_hosted_module_declarations(
    modules: Vec<HostedTypedProgramModule>,
    providers: Vec<RegisteredHostProviderModule>,
) -> Result<Vec<HostedModuleDeclaration>, PlanError> {
    let mut provider_modules = providers
        .into_iter()
        .map(|provider| {
            let (package, module, functions, external_types) = provider.into_parts();
            (
                (package, module),
                RegisteredProviderItems {
                    functions,
                    external_types,
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
    let external_types = provider_modules
        .iter()
        .flat_map(|((package, module), items)| {
            items.external_types.iter().map(|schema| {
                crate::plan::ExternalTypeName::new(
                    package.clone(),
                    module.clone(),
                    schema.name().clone(),
                )
            })
        })
        .collect::<std::collections::HashSet<_>>();
    modules
        .into_iter()
        .enumerate()
        .map(|(index, module)| {
            hosted_module_declaration(
                ModuleId::new(index),
                module,
                &mut provider_modules,
                &external_types,
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .and_then(|declarations| {
            if let Some(error) =
                provider_modules
                    .into_iter()
                    .find_map(|((package, module), items)| {
                        let function = items
                            .functions
                            .iter()
                            .map(|function| function.schema().name().clone())
                            .min();
                        if let Some(function) = function {
                            return Some(PlanError::HostProviderLink {
                                package,
                                module,
                                function,
                                reason: Box::new(HostProviderLinkReason::MissingModule),
                            });
                        }
                        items
                            .external_types
                            .iter()
                            .map(|schema| schema.name().clone())
                            .min()
                            .map(|type_| PlanError::ExternalTypeProviderLink {
                                package,
                                module,
                                type_,
                                reason: Box::new(
                                    crate::planner::ExternalTypeProviderLinkReason::MissingModule,
                                ),
                            })
                    })
            {
                Err(error)
            } else {
                Ok(declarations)
            }
        })
}

fn hosted_module_declaration(
    id: ModuleId,
    module: HostedTypedProgramModule,
    provider_modules: &mut BTreeMap<(EcoString, EcoString), RegisteredProviderItems>,
    external_types: &std::collections::HashSet<crate::plan::ExternalTypeName>,
) -> Result<HostedModuleDeclaration, PlanError> {
    match module {
        HostedTypedProgramModule::Source(module) => {
            let path = module.path;
            let source = module.source;
            let package = module.module.type_info.package.clone();
            let definitions = module.module.definitions;
            let module_name = module.module.name;
            let providers = provider_modules
                .remove(&(package.clone(), module_name.clone()))
                .unwrap_or(RegisteredProviderItems {
                    functions: Vec::new(),
                    external_types: Vec::new(),
                });
            super::super::external_type::plan_hosted_types(
                &package,
                &module_name,
                definitions.custom_types,
                providers.external_types,
                external_types,
            )
            .map(|types| HostedModuleDeclaration::Source {
                id,
                providers: providers.functions,
                package,
                module_name,
                source_context: Some(SourceContext::new(path, source)),
                custom_types: types.custom_types,
                external_types: types.external_types,
                functions: definitions.functions,
                constants: definitions.constants,
            })
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

#[cfg(test)]
mod tests {
    use super::super::plan_host_program;
    use crate::frontend::{ModuleSource, PackageSource, compile_typed_host_program};
    use crate::host::{HostModule, HostProviderModule, HostProviderSet, StatelessHostProfile};
    use crate::planner::{ExternalTypeProviderLinkReason, HostProviderLinkReason, PlanError};
    use ecow::EcoString;
    use num_bigint::BigInt;

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
            Some(PlanError::ExternalTypeProviderLink {
                package: "application".into(),
                module: "zsupport".into(),
                type_: "Thing".into(),
                reason: Box::new(ExternalTypeProviderLinkReason::MissingRegistration),
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
    fn empty_provider_module_has_no_source_linkage_target() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "missing")
            .expect("provider module should be valid");
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

        let plan = plan_host_program(typed).expect("empty provider should not require a module");

        assert_eq!(plan.modules().len(), 1);
        assert_eq!(plan.modules()[0].module(), "main");
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
}
