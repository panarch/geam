use super::super::constant::{ConstantBodies, plan_constant_bodies, reserve_constants};
use super::link::{LinkedFunction, LinkedModule, validate_host_custom_schemas};
use crate::plan::{ConstantTemplates, ModuleId, SourceContext};
use crate::planner::error::PlanError;
use crate::planner::module::registry::{ModuleRegistry, ProgramRegistry};
use ecow::EcoString;

pub(super) struct ModuleWithConstants {
    id: ModuleId,
    package: EcoString,
    source_context: Option<SourceContext>,
    custom_types: Vec<crate::plan::CustomTypeDefinition>,
    functions: Vec<LinkedFunction>,
    constants: ConstantBodies,
    anonymous_functions: super::super::AnonymousFunctions,
}

pub(super) struct ModuleToPlan {
    pub(super) id: ModuleId,
    pub(super) package: EcoString,
    pub(super) source_context: Option<SourceContext>,
    pub(super) custom_types: Vec<crate::plan::CustomTypeDefinition>,
    pub(super) functions: Vec<LinkedFunction>,
    pub(super) constants: ConstantTemplates,
    pub(super) anonymous_functions: super::super::AnonymousFunctions,
}

pub(super) fn reserve_hosted_constants(
    modules: Vec<LinkedModule>,
) -> Result<(ProgramRegistry, Vec<ModuleWithConstants>), PlanError> {
    modules
        .into_iter()
        .map(reserve_hosted_module_constants)
        .collect::<Result<Vec<_>, _>>()
        .and_then(|reserved| {
            let (registry_modules, modules): (Vec<_>, Vec<_>) = reserved.into_iter().unzip();
            let registry = ProgramRegistry::new(registry_modules);
            for module in &modules {
                validate_host_custom_schemas(
                    &registry,
                    module.source_context.as_ref(),
                    &module.functions,
                )?;
            }
            Ok((registry, modules))
        })
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

pub(super) fn plan_hosted_constant_bodies(
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
