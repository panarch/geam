use super::constant::ModuleToPlan;
use super::link::LinkedFunction;
use crate::host::{RegisteredHostConstructions, RegisteredHostImplementationId};
use crate::plan::{
    FunctionTemplateId, HostedFunctionTemplate, HostedPlannedModule, HostedPlannedModuleParts,
    ModuleId,
};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use crate::planner::function::{plan_function, plan_selected_external_fallback};
use crate::planner::module::registry::ProgramRegistry;

pub(super) struct PlannedHostedProgram {
    pub(super) root: ModuleId,
    pub(super) entry: FunctionTemplateId,
    pub(super) modules: Vec<HostedPlannedModule>,
    pub(super) implementations: Vec<(
        FunctionTemplateId,
        RegisteredHostConstructions,
        RegisteredHostImplementationId,
    )>,
}

pub(super) fn plan_hosted_modules(
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
    implementations: &mut Vec<(
        FunctionTemplateId,
        RegisteredHostConstructions,
        RegisteredHostImplementationId,
    )>,
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
                external_types: module.external_types,
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
    anonymous_functions: &mut super::super::AnonymousFunctions,
    implementations: &mut Vec<(
        FunctionTemplateId,
        RegisteredHostConstructions,
        RegisteredHostImplementationId,
    )>,
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
            constructions,
            implementation,
        } => {
            implementations.push((template.id(), constructions, implementation));
            Ok(HostedFunctionTemplate::HostTemplate(Box::new(template)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::plan_host_program;
    use crate::frontend::{ModuleSource, PackageSource, compile_typed_host_program};
    use crate::host::{HostModule, HostProviderSet};
    use crate::planner::PlanError;
    use ecow::EcoString;

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
}
