mod parameter;
mod return_;
mod sealing;
mod table;
mod template;

use super::function;
use super::library;
use super::specialization::{RepresentationContext, SpecializationKey, SpecializedValueShape};
use super::{
    LoweringContext, ProgramConstantTemplates, SpecializationOutcome, SpecializationState,
    try_resolve_specialization_fixed_point,
};
use crate::host::HostProfile;
use crate::plan::execution::LibraryFunctionEntries;
use crate::plan::execution::function::{HostedExecutionGraph, RuntimeFunctionId};
use crate::plan::execution::host::{
    HostFunctionTables, HostSpecializationError, HostedExecutionProfile,
};
use crate::plan::execution::{ExecutionModuleContext, ExecutionProgram, ExecutionProgramCommon};
use crate::plan::{
    HostImplementationBinding, HostedLibraryModulePlan, HostedLibraryModulePlanParts,
    HostedModulePlan, HostedModulePlanParts, HostedPlannedModule, LibraryEntry, ModuleId,
};
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
    lower_hosted_entries(
        HostedLoweringInput {
            root,
            modules,
            implementation_bindings,
        },
        MainEntry { template: entry },
    )
    .map(|(program, host_functions, ())| (program, host_functions))
}

pub(in crate::plan::execution) fn lower_hosted_library<Profile: HostProfile>(
    module_plan: HostedLibraryModulePlan<Profile>,
    first: LibraryEntry,
    remaining: Vec<LibraryEntry>,
) -> Result<
    (
        ExecutionProgram<HostedExecutionProfile>,
        HostFunctionTables<Profile>,
        LibraryFunctionEntries,
    ),
    HostSpecializationError,
> {
    let HostedLibraryModulePlanParts {
        root,
        modules,
        implementation_bindings,
    } = module_plan.into_parts();
    lower_hosted_entries(
        HostedLoweringInput {
            root,
            modules,
            implementation_bindings,
        },
        library::Entries::new(first, remaining),
    )
}

struct HostedLoweringInput<Profile: HostProfile> {
    root: ModuleId,
    modules: Vec<HostedPlannedModule>,
    implementation_bindings: Vec<HostImplementationBinding<Profile>>,
}

struct MainEntry {
    template: crate::plan::FunctionTemplateId,
}

type HostedLoweringResult<Profile, Output> = Result<
    (
        ExecutionProgram<HostedExecutionProfile>,
        HostFunctionTables<Profile>,
        Output,
    ),
    HostSpecializationError,
>;

trait HostedEntries {
    type Reserved;
    type Output;

    fn initial_key(&self) -> SpecializationKey;

    fn reserve(
        &self,
        templates: &HostTemplateCatalog,
        context: &mut LoweringContext,
    ) -> Self::Reserved;

    fn seal(reserved: Self::Reserved) -> SpecializationOutcome<(RuntimeFunctionId, Self::Output)>;
}

fn lower_hosted_entries<Profile, Entries>(
    input: HostedLoweringInput<Profile>,
    entries: Entries,
) -> HostedLoweringResult<Profile, Entries::Output>
where
    Profile: HostProfile,
    Entries: HostedEntries,
{
    let HostedLoweringInput {
        root,
        modules,
        implementation_bindings,
    } = input;
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

    let initial = SpecializationState {
        constant_templates: ProgramConstantTemplates {
            modules: constant_templates,
        },
        representations: RepresentationContext::new(custom_types),
        erased_specializations: HashSet::new(),
    };

    let (main, entry_output, lowered, host_functions) =
        try_resolve_specialization_fixed_point(initial, |state| {
            let SpecializationState {
                constant_templates,
                representations,
                erased_specializations,
            } = state;
            let mut context = LoweringContext::new(
                templates.entry_templates(),
                representations,
                constant_templates,
                entries.initial_key(),
                erased_specializations,
            );
            let reserved_entries = entries.reserve(&templates, &mut context);
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
            let outcome = Entries::seal(reserved_entries)
                .zip_with(lowered, |(main, entry_output), lowered| {
                    (main, entry_output, lowered, host_functions)
                });
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
        entry_output,
    ))
}

impl HostedEntries for MainEntry {
    type Reserved = RuntimeFunctionId;
    type Output = ();

    fn initial_key(&self) -> SpecializationKey {
        SpecializationKey::monomorphic(self.template)
    }

    fn reserve(
        &self,
        templates: &HostTemplateCatalog,
        context: &mut LoweringContext,
    ) -> Self::Reserved {
        let key = self.initial_key();
        let return_shape = templates
            .get(self.template)
            .signature()
            .shape()
            .return_shape();
        let value_shape = SpecializedValueShape::instantiate(return_shape, key.substitution());
        let return_ = context.representations.inhabitation(&value_shape);
        context.reserve_main(key, return_)
    }

    fn seal(reserved: Self::Reserved) -> SpecializationOutcome<(RuntimeFunctionId, Self::Output)> {
        SpecializationOutcome::Complete((reserved, ()))
    }
}

impl HostedEntries for library::Entries {
    type Reserved = library::ReservedEntries;
    type Output = LibraryFunctionEntries;

    fn initial_key(&self) -> SpecializationKey {
        library::Entries::initial_key(self)
    }

    fn reserve(
        &self,
        _templates: &HostTemplateCatalog,
        context: &mut LoweringContext,
    ) -> Self::Reserved {
        library::Entries::reserve(self, context)
    }

    fn seal(reserved: Self::Reserved) -> SpecializationOutcome<(RuntimeFunctionId, Self::Output)> {
        reserved
            .seal()
            .map(|entries| entries.finish::<HostedExecutionGraph>())
    }
}

#[cfg(test)]
mod tests {
    use super::lower_hosted_library;
    use crate::plan::LibraryEntry;
    use crate::{
        HostModule, HostProviderSet, ModuleSource, PackageSource, compile_typed_host_program,
    };
    use num_bigint::BigInt;

    #[test]
    fn shares_reachable_host_specializations_and_prunes_unused_providers() {
        let math = HostModule::new("host_support", "host/math")
            .expect("math module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("add should register")
            .with_function("unused", <BigInt as std::ops::Sub>::sub)
            .expect("unused should register");
        let hosts = HostProviderSet::new([math]).expect("math module should be unique");
        let program = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
import host/math

pub fn first(value: Int) { math.add(value, 1) }
pub fn second(value: Int) { math.add(value, 2) }
"#,
                )],
            )],
            hosts,
        )
        .expect("hosted library should compile");
        let plan =
            crate::planner::plan_host_library_program(program).expect("hosted library should plan");
        let entry = |name: &str| {
            let template = plan
                .functions()
                .iter()
                .find(|function| function.name() == name)
                .expect("selected root function should exist");
            LibraryEntry::Int(template.signature().id())
        };
        let first = entry("first");
        let second = entry("second");

        let (_, host_functions, entries) =
            lower_hosted_library(plan, first, vec![second]).expect("entries should seal");

        assert_eq!(entries.ints.len(), 2);
        assert_eq!(host_functions.value_functions().len(), 1);
        assert_eq!(host_functions.value_functions()[0].name(), "add");
        assert!(host_functions.never_functions().is_empty());
    }
}
