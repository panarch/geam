mod parameter;
mod return_;
mod sealing;
mod table;
mod template;
mod transfer;

use super::function;
use super::library;
use super::specialization::{RepresentationContext, SpecializationKey, SpecializedValueShape};
use super::{
    LoweredExecution, LoweringCompletion, LoweringContext, ProgramConstantTemplates,
    SpecializationOutcome, SpecializationState, try_resolve_specialization_fixed_point,
};
use crate::host::HostProfile;
use crate::plan::execution::LibraryFunctionEntries;
use crate::plan::execution::function::RuntimeFunctionId;
use crate::plan::execution::host::{
    HostFunctionTables, HostSpecializationError, HostedExecutionProfile,
    TransferHostFunctionTables, TransferHostedExecutionProfile,
};
use crate::plan::execution::{ExecutionModuleContext, ExecutionProgram, ExecutionProgramCommon};
use crate::plan::{
    HostedLibraryModulePlan, HostedLibraryModulePlanParts, HostedModulePlan, HostedModulePlanParts,
    HostedPlannedModule, LibraryEntry, ModuleId,
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

pub(in crate::plan::execution) fn lower_transfer_hosted_library<Profile: HostProfile>(
    module_plan: crate::plan::TransferHostedLibraryModulePlan<Profile>,
    first: LibraryEntry,
    remaining: Vec<LibraryEntry>,
) -> Result<
    (
        ExecutionProgram<TransferHostedExecutionProfile>,
        TransferHostFunctionTables<Profile>,
        LibraryFunctionEntries,
    ),
    HostSpecializationError,
> {
    let crate::plan::TransferHostedLibraryModulePlanParts {
        root,
        modules,
        implementation_bindings,
    } = module_plan.into_parts();
    lower_hosted_entries(
        HostedLoweringInput { root, modules },
        library::Entries::new(first, remaining),
        transfer::TransferHostFunctionRegistry::new(implementation_bindings),
    )
}

impl LoweringContext {
    fn finish_hosted<Execution>(
        self,
        additional: function::ProfiledFunctionEntries<Execution>,
    ) -> LoweringCompletion<LoweredExecution<Execution>>
    where
        Execution: crate::plan::execution::function::DirectHostedExecutionProfile,
    {
        let mut this = self;
        let function_parameters = this.function_parameter_catalog();
        let Self {
            constant_templates,
            constants,
            types,
            representations,
            functions,
            erased_specializations,
            ..
        } = this;
        let outcome = functions
            .finish_hosted(additional)
            .zip_with(
                SpecializationOutcome::Complete(constants.finish_hosted()),
                |functions, constants| {
                    let (list_types, custom_types, external_types, value_shapes) =
                        types.into_tables();
                    Box::new(LoweredExecution {
                        constants,
                        functions: *functions,
                        function_parameters,
                        list_types,
                        custom_types,
                        external_types,
                        value_shapes,
                    })
                },
            )
            .include_prior_erasure(erased_specializations);
        (constant_templates, representations, outcome)
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
    type Programs;
    type Lowered;
    type Error;
    type Lowering<'registry>: SealedHostFunctionLowering<
            Execution = Self::Execution,
            Tables = Self::Tables,
            Lowered = Self::Lowered,
            Error = Self::Error,
        >
    where
        Self: 'registry;

    fn lowering(&self) -> Self::Lowering<'_>;

    fn assemble(
        &self,
        root: ModuleId,
        modules: Box<[ExecutionModuleContext]>,
        main: RuntimeFunctionId,
        lowered: Box<Self::Lowered>,
    ) -> Self::Programs;
}

trait SealedHostFunctionLowering {
    type Execution: crate::plan::execution::function::DirectHostedExecutionProfile;
    type Tables;
    type Lowered;
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
    ) -> (super::LoweringCompletion<Self::Lowered>, Self::Tables);
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
    <Registry as SealedHostFunctionRegistry>::Programs,
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

    let programs =
        implementations.assemble(root, module_contexts.into_boxed_slice(), main, lowered);
    Ok((programs, host_functions, entry_output))
}

impl<Profile: HostProfile> SealedHostFunctionRegistry for HostFunctionRegistry<Profile> {
    type Execution = HostedExecutionProfile;
    type Tables = HostFunctionTables<Profile>;
    type Programs = ExecutionProgram<HostedExecutionProfile>;
    type Lowered = super::LoweredExecution<HostedExecutionProfile>;
    type Error = HostSpecializationError;
    type Lowering<'registry>
        = table::HostFunctionLowering<'registry, Profile>
    where
        Self: 'registry;

    fn lowering(&self) -> Self::Lowering<'_> {
        HostFunctionRegistry::lowering(self)
    }

    fn assemble(
        &self,
        root: ModuleId,
        modules: Box<[ExecutionModuleContext]>,
        main: RuntimeFunctionId,
        lowered: Box<Self::Lowered>,
    ) -> Self::Programs {
        let super::LoweredExecution {
            constants,
            functions,
            function_parameters,
            list_types,
            custom_types,
            external_types,
            value_shapes,
        } = *lowered;
        ExecutionProgram {
            common: std::sync::Arc::new(ExecutionProgramCommon {
                root,
                modules,
                main,
                constants,
                function_parameters: std::sync::Arc::new(function_parameters),
                list_types,
                custom_types,
                external_types,
                value_shapes,
            }),
            functions,
        }
    }
}

impl<Profile: HostProfile> SealedHostFunctionLowering for table::HostFunctionLowering<'_, Profile> {
    type Execution = HostedExecutionProfile;
    type Tables = HostFunctionTables<Profile>;
    type Lowered = super::LoweredExecution<HostedExecutionProfile>;
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
        reserved.seal().map(library::SealedEntries::finish)
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
    fn every_hosted_library_family_can_own_the_first_entry() {
        use crate::plan::{ExternalType, ExternalTypeName, StandardVariant, ValueType};
        struct Profile;
        impl crate::HostProfile for Profile {
            type RunState = ();
            type ExternalStores = crate::host::HostFutureStore;
        }
        impl crate::host::HostWorkProfile for Profile {
            type Work = crate::work_fixture::WorkComponent;
        }
        impl crate::host::AsyncHostComponentProfile<crate::work_fixture::WorkComponent> for Profile {
            fn component_async_stores(stores: &Self::ExternalStores) -> &Self::ExternalStores {
                stores
            }
            fn component_state(state: &mut ()) -> &mut () {
                state
            }
        }
        let mut state = ();
        let stores = crate::host::HostFutureStore::default();
        assert!(std::ptr::eq(
            <Profile as crate::host::AsyncHostComponentProfile<
                crate::work_fixture::WorkComponent,
            >>::component_async_stores(&stores),
            &stores,
        ));
        assert_eq!(
            <crate::work_fixture::WorkComponent as crate::HostProvider<Profile>>::project(
                &mut state
            ),
            &()
        );
        let cases = [
            ("Int", LibraryValueType::Int, Vec::new(), Vec::new()),
            ("Float", LibraryValueType::Float, Vec::new(), Vec::new()),
            ("String", LibraryValueType::String, Vec::new(), Vec::new()),
            (
                "BitArray",
                LibraryValueType::BitArray,
                Vec::new(),
                Vec::new(),
            ),
            (
                "UtfCodepoint",
                LibraryValueType::UtfCodepoint,
                Vec::new(),
                Vec::new(),
            ),
            (
                "Result(Int, String)",
                LibraryValueType::Custom(
                    StandardVariant::Result.custom_type(vec![ValueType::Int, ValueType::String]),
                ),
                vec![StandardVariant::Result],
                Vec::new(),
            ),
            (
                "future.Work(Int)",
                LibraryValueType::External(ExternalType::new(
                    ExternalTypeName::new(
                        "work_fixture".into(),
                        "fixture/work".into(),
                        "Work".into(),
                    ),
                    vec![ValueType::Int],
                )),
                Vec::new(),
                Vec::new(),
            ),
            ("Bool", LibraryValueType::Bool, Vec::new(), Vec::new()),
            ("Nil", LibraryValueType::Nil, Vec::new(), Vec::new()),
            (
                "List(Int)",
                LibraryValueType::List(Box::new(LibraryValueType::Int)),
                Vec::new(),
                vec![LibraryValueType::Int],
            ),
            (
                "#(Int, Bool)",
                LibraryValueType::Tuple(vec![ValueType::Int, ValueType::Bool]),
                Vec::new(),
                Vec::new(),
            ),
        ];
        for (index, (source_type, return_type, variants, lists)) in cases.into_iter().enumerate() {
            let source = format!(
                "import fixture/work as future\npub fn identity(value: {source_type}) -> {source_type} {{ value }}"
            );
            let program = crate::frontend::compile_typed_transfer_host_program(
                "application",
                "library",
                [
                    PackageSource::new(
                        "work_fixture",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "fixture/work",
                            "src/fixture/work.gleam",
                            crate::work_fixture::WorkComponent::SOURCE,
                        )],
                    ),
                    PackageSource::new(
                        "application",
                        ["work_fixture"],
                        [ModuleSource::new("library", "src/library.gleam", source)],
                    ),
                ],
                crate::host::TransferHostProviderSet::new(
                    crate::work_fixture::WorkComponent::providers::<Profile>()
                        .expect("Future provider"),
                )
                .expect("providers"),
            )
            .expect("source identity");
            let plan =
                crate::planner::plan_transfer_host_library_program(program).expect("typed library");
            let template = plan
                .functions()
                .iter()
                .find(|function| function.name() == "identity")
                .expect("identity")
                .signature()
                .id();
            let (_, _, entries) = super::lower_transfer_hosted_library(
                plan,
                LibraryEntry::new(template, return_type, variants, lists),
                Vec::new(),
            )
            .expect("first family seals");
            let counts = [
                entries.ints.len(),
                entries.floats.len(),
                entries.strings.len(),
                entries.bit_arrays.len(),
                entries.utf_codepoints.len(),
                entries.customs.len(),
                entries.externals.len(),
                entries.bools.len(),
                entries.nils.len(),
                entries.lists.len(),
                entries.tuples.len(),
            ];
            assert_eq!(counts[index], 1, "{source_type}");
            assert_eq!(counts.into_iter().sum::<usize>(), 1);
        }
    }

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
