mod parameter;
mod return_;
mod sealing;
mod table;
mod template;

use super::function;
use super::specialization::{RepresentationContext, SpecializationKey, SpecializedValueShape};
use super::{
    LoweringContext, ProgramConstantTemplates, SpecializationState,
    try_resolve_specialization_fixed_point,
};
use crate::host::HostProfile;
use crate::plan::execution::host::{
    HostFunctionTables, HostSpecializationError, HostedExecutionProfile,
};
use crate::plan::execution::{ExecutionModuleContext, ExecutionProgram, ExecutionProgramCommon};
use crate::plan::{HostedModulePlan, HostedModulePlanParts};
use std::collections::HashSet;
use table::HostFunctionRegistry;
use template::{HostLoweringTemplate, HostTemplateCatalog};

pub(in crate::plan::execution) fn lower_hosted<Profile: HostProfile>(
    module_plan: HostedModulePlan<Profile>,
) -> Result<
    (
        ExecutionProgram<HostedExecutionProfile>,
        HostFunctionTables<Profile>,
    ),
    HostSpecializationError,
> {
    let HostedModulePlanParts {
        root,
        entry,
        modules,
        implementation_bindings,
    } = module_plan.into_parts();
    let implementations = HostFunctionRegistry::new(implementation_bindings);
    let mut module_contexts = Vec::with_capacity(modules.len());
    let mut templates = HostTemplateCatalog::new();
    let mut constant_templates = Vec::with_capacity(modules.len());
    let mut custom_types = Vec::new();

    for module in modules {
        let parts = module.into_parts();
        module_contexts.push(ExecutionModuleContext::new(
            parts.module,
            parts.source_context,
        ));
        custom_types.extend(parts.custom_types);
        constant_templates.push(parts.constants);
        templates.push_module(parts.functions, parts.anonymous_functions);
    }

    let main_return_shape = templates
        .get(entry)
        .signature()
        .shape()
        .return_shape()
        .clone();
    let main_key = SpecializationKey::monomorphic(entry);
    let initial = SpecializationState {
        constant_templates: ProgramConstantTemplates {
            modules: constant_templates,
        },
        representations: RepresentationContext::new(custom_types),
        erased_specializations: HashSet::new(),
    };

    let (main, lowered, host_functions) =
        try_resolve_specialization_fixed_point(initial, |state| {
            let SpecializationState {
                constant_templates,
                representations,
                erased_specializations,
            } = state;
            let main_value_shape =
                SpecializedValueShape::instantiate(&main_return_shape, main_key.substitution());
            let main_return_shape = representations.inhabitation(&main_value_shape);
            let mut context = LoweringContext::new(
                templates.entry_templates(),
                representations,
                constant_templates,
                main_key.clone(),
                erased_specializations,
            );
            let main = context.reserve_main(main_key.clone(), main_return_shape);
            let mut host_functions = implementations.lowering();

            while let Some(key) = context.pending.pop_front() {
                context.begin(&key);
                match templates.get(key.template()) {
                    HostLoweringTemplate::Gleam(template) => {
                        function::lower_specialized(template, &key, &mut context);
                    }
                    HostLoweringTemplate::Host(template) => {
                        host_functions.lower_specialized(template, &key, &mut context)?;
                    }
                }
            }

            let (completion, host_functions) = host_functions.finish(context);
            let (constant_templates, representations, lowered) = completion;
            let outcome = super::SpecializationOutcome::Complete(main)
                .zip_with(lowered, |main, lowered| (main, lowered, host_functions));
            let erased_specializations = outcome.erased_specializations();
            Ok(outcome.into_fixed_point(SpecializationState {
                constant_templates,
                representations,
                erased_specializations,
            }))
        })?;

    Ok((
        ExecutionProgram {
            common: ExecutionProgramCommon {
                root,
                modules: module_contexts.into_boxed_slice(),
                main,
                constants: lowered.constants,
                list_types: lowered.list_types,
                custom_types: lowered.custom_types,
                external_types: lowered.external_types,
                value_shapes: lowered.value_shapes,
            },
            functions: lowered.functions,
        },
        host_functions,
    ))
}
