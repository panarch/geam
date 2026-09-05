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
    AsyncHostFunctionTables, AsyncHostedExecutionProfile, HostFunctionTables,
    HostSpecializationError, HostedExecutionProfile,
};
use crate::plan::execution::{ExecutionModuleContext, ExecutionProgram, ExecutionProgramCommon};
use crate::plan::{
    AsyncHostedLibraryModulePlan, AsyncHostedLibraryModulePlanParts, HostedLibraryModulePlan,
    HostedLibraryModulePlanParts, HostedModulePlan, HostedModulePlanParts, HostedPlannedModule,
    LibraryEntry, ModuleId,
};
use std::collections::HashSet;
use table::{AsyncHostFunctionRegistry, HostFunctionRegistry};
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
    lower_hosted_entries(
        HostedLoweringInput { root, modules },
        MainEntry { template: entry },
        implementations,
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
    let implementations = HostFunctionRegistry::new(implementation_bindings);
    lower_hosted_entries(
        HostedLoweringInput { root, modules },
        library::Entries::new(first, remaining),
        implementations,
    )
}

pub(in crate::plan::execution) fn lower_async_hosted_library<Profile: HostProfile>(
    module_plan: AsyncHostedLibraryModulePlan<Profile>,
    first: LibraryEntry,
    remaining: Vec<LibraryEntry>,
) -> (
    ExecutionProgram<AsyncHostedExecutionProfile>,
    AsyncHostFunctionTables<Profile>,
    LibraryFunctionEntries,
) {
    let AsyncHostedLibraryModulePlanParts {
        root,
        modules,
        implementation_bindings,
    } = module_plan.into_parts();
    let implementations = AsyncHostFunctionRegistry::new(implementation_bindings);
    match lower_hosted_entries(
        HostedLoweringInput { root, modules },
        library::Entries::new(first, remaining),
        implementations,
    ) {
        Ok(output) => output,
        Err(error) => match error {},
    }
}

struct HostedLoweringInput {
    root: ModuleId,
    modules: Vec<HostedPlannedModule>,
}

struct MainEntry {
    template: crate::plan::FunctionTemplateId,
}

trait SealedHostFunctionRegistry {
    type Execution: crate::plan::execution::function::DirectHostedExecutionProfile;
    type Tables;
    type Error;
    type Lowering<'registry>: SealedHostFunctionLowering<
            Execution = Self::Execution,
            Tables = Self::Tables,
            Error = Self::Error,
        >
    where
        Self: 'registry;

    fn lowering(&self) -> Self::Lowering<'_>;
}

trait SealedHostFunctionLowering {
    type Execution: crate::plan::execution::function::DirectHostedExecutionProfile;
    type Tables;
    type Error;

    fn lower_specialized(
        &mut self,
        template: &crate::plan::HostFunctionTemplate,
        key: &SpecializationKey,
        context: &mut LoweringContext,
    ) -> Result<(), Self::Error>;

    fn finish(
        self,
        context: LoweringContext,
    ) -> (
        super::LoweringCompletion<super::LoweredExecution<Self::Execution>>,
        Self::Tables,
    );
}

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

type LoweredHostedEntries<Entries, Registry> = (
    ExecutionProgram<<Registry as SealedHostFunctionRegistry>::Execution>,
    <Registry as SealedHostFunctionRegistry>::Tables,
    <Entries as HostedEntries>::Output,
);

fn lower_hosted_entries<Entries, Registry>(
    input: HostedLoweringInput,
    entries: Entries,
    implementations: Registry,
) -> Result<LoweredHostedEntries<Entries, Registry>, Registry::Error>
where
    Entries: HostedEntries,
    Registry: SealedHostFunctionRegistry,
{
    let HostedLoweringInput { root, modules } = input;
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

impl<Profile: HostProfile> SealedHostFunctionRegistry for HostFunctionRegistry<Profile> {
    type Execution = HostedExecutionProfile;
    type Tables = HostFunctionTables<Profile>;
    type Error = HostSpecializationError;
    type Lowering<'registry>
        = table::HostFunctionLowering<'registry, Profile>
    where
        Self: 'registry;

    fn lowering(&self) -> Self::Lowering<'_> {
        HostFunctionRegistry::lowering(self)
    }
}

impl<Profile: HostProfile> SealedHostFunctionRegistry for AsyncHostFunctionRegistry<Profile> {
    type Execution = AsyncHostedExecutionProfile;
    type Tables = AsyncHostFunctionTables<Profile>;
    type Error = std::convert::Infallible;
    type Lowering<'registry>
        = table::AsyncHostFunctionLowering<'registry, Profile>
    where
        Self: 'registry;

    fn lowering(&self) -> Self::Lowering<'_> {
        AsyncHostFunctionRegistry::lowering(self)
    }
}

impl<Profile: HostProfile> SealedHostFunctionLowering for table::HostFunctionLowering<'_, Profile> {
    type Execution = HostedExecutionProfile;
    type Tables = HostFunctionTables<Profile>;
    type Error = HostSpecializationError;

    fn lower_specialized(
        &mut self,
        template: &crate::plan::HostFunctionTemplate,
        key: &SpecializationKey,
        context: &mut LoweringContext,
    ) -> Result<(), Self::Error> {
        table::HostFunctionLowering::lower_specialized(self, template, key, context)
    }

    fn finish(
        self,
        context: LoweringContext,
    ) -> (
        super::LoweringCompletion<super::LoweredExecution<HostedExecutionProfile>>,
        Self::Tables,
    ) {
        table::HostFunctionLowering::finish(self, context)
    }
}

impl<Profile: HostProfile> SealedHostFunctionLowering
    for table::AsyncHostFunctionLowering<'_, Profile>
{
    type Execution = AsyncHostedExecutionProfile;
    type Tables = AsyncHostFunctionTables<Profile>;
    type Error = std::convert::Infallible;

    fn lower_specialized(
        &mut self,
        template: &crate::plan::HostFunctionTemplate,
        key: &SpecializationKey,
        context: &mut LoweringContext,
    ) -> Result<(), Self::Error> {
        table::AsyncHostFunctionLowering::lower_specialized(self, template, key, context)
    }

    fn finish(
        self,
        context: LoweringContext,
    ) -> (
        super::LoweringCompletion<super::LoweredExecution<AsyncHostedExecutionProfile>>,
        Self::Tables,
    ) {
        table::AsyncHostFunctionLowering::finish(self, context)
    }
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
    use crate::plan::{LibraryEntry, LibraryValueType};
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
            LibraryEntry::new(
                template.signature().id(),
                LibraryValueType::Int,
                Vec::new(),
                Vec::new(),
            )
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
