use super::function as function_lowering;
use super::graph;
use super::library;
use super::specialization::{self, RepresentationContext, SpecializationKey};
use super::{
    LoweringContext, ProgramConstantTemplates, SpecializationOutcome, SpecializationState,
    resolve_specialization_fixed_point,
};
use crate::plan::execution::function as execution_function;
use crate::plan::execution::{
    ExecutionModuleContext, ExecutionProgram, ExecutionProgramCommon, LibraryFunctionEntries,
};
use crate::plan::{
    FunctionTemplate, FunctionTemplateId, LibraryEntry, LibraryModulePlan, ModuleId, ModulePlan,
    PlannedModule,
};
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;

pub(in crate::plan::execution) fn lower(module_plan: ModulePlan) -> ExecutionProgram<Infallible> {
    let parts = module_plan.into_parts();
    lower_plain(
        parts.root,
        parts.modules,
        PlainEntry::Main(parts.entry),
        Vec::new(),
    )
    .0
}

pub(in crate::plan::execution) fn lower_library(
    module_plan: LibraryModulePlan,
    first: LibraryEntry,
    remaining: Vec<LibraryEntry>,
) -> (ExecutionProgram<Infallible>, LibraryFunctionEntries) {
    let parts = module_plan.into_parts();
    let (program, entries) = lower_plain(
        parts.root,
        parts.modules,
        PlainEntry::from(first),
        remaining.into_iter().map(PlainEntry::from).collect(),
    );
    (program, entries.finish())
}

fn lower_plain(
    root: ModuleId,
    modules: Vec<PlannedModule>,
    first_entry: PlainEntry,
    remaining_entries: Vec<PlainEntry>,
) -> (ExecutionProgram<Infallible>, library::EntryIds) {
    let mut module_contexts = Vec::with_capacity(modules.len());
    let mut module_templates = Vec::with_capacity(modules.len());
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
        let mut templates = parts.functions;
        templates.extend(parts.anonymous_functions);
        templates.sort_by_key(|template| template.id().index());
        module_templates.push(templates);
    }

    let templates = FunctionTemplates::new(module_templates);
    let initial = SpecializationState {
        constant_templates: ProgramConstantTemplates {
            modules: constant_templates,
        },
        representations: RepresentationContext::new(custom_types),
        erased_specializations: HashSet::new(),
    };

    // Function indices remain provisional until a pass produces no new erasures.
    let ((first_entry_id, remaining_entry_ids), lowered) =
        resolve_specialization_fixed_point(initial, |state| {
            let SpecializationState {
                constant_templates,
                representations,
                erased_specializations,
            } = state;
            let mut context = LoweringContext::new(
                templates.entry_templates(),
                representations,
                constant_templates,
                SpecializationKey::monomorphic(first_entry.template()),
                erased_specializations,
            );

            let first = reserve_plain_entry(first_entry, &templates, &mut context);
            let remaining = remaining_entries
                .iter()
                .copied()
                .map(|entry| reserve_plain_entry(entry, &templates, &mut context))
                .collect::<Vec<_>>();

            while let Some(key) = context.pending.pop_front() {
                context.begin(&key);
                function_lowering::lower_specialized(
                    templates.get(key.template()),
                    &key,
                    &mut context,
                );
            }

            let (constant_templates, representations, lowered) = context.finish();
            let (first_key, first) = first.seal();
            let entries = remaining.into_iter().fold(
                SpecializationOutcome::from_representability(first, first_key)
                    .map(|first| (first, Vec::with_capacity(remaining_entries.len()))),
                |entries, entry| {
                    let (key, entry) = entry.seal();
                    entries.zip_with(
                        SpecializationOutcome::from_representability(entry, key),
                        |(first, mut remaining), entry| {
                            remaining.push(entry);
                            (first, remaining)
                        },
                    )
                },
            );
            let outcome = entries.zip_with(lowered, |entries, lowered| (entries, lowered));
            let erased_specializations = outcome.erased_specializations();
            outcome.into_fixed_point(SpecializationState {
                constant_templates,
                representations,
                erased_specializations,
            })
        });

    let main = first_entry_id.runtime_id();
    let mut entry_ids = library::EntryIds::default();
    first_entry_id.push(&mut entry_ids);
    for entry in remaining_entry_ids {
        entry.push(&mut entry_ids);
    }
    let program = ExecutionProgram {
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
    };
    (program, entry_ids)
}

#[derive(Clone, Copy)]
enum PlainEntry {
    Main(FunctionTemplateId),
    Library(library::Entry),
}

enum ReservedPlainEntry {
    Main {
        key: SpecializationKey,
        id: execution_function::RuntimeFunctionId,
    },
    Library(library::ReservedEntry),
}

enum SealedPlainEntry {
    Main(execution_function::ProfiledRuntimeFunctionId<Infallible>),
    Library(library::SealedEntry),
}

struct FunctionTemplates {
    templates: Vec<Vec<FunctionTemplate>>,
}

impl From<LibraryEntry> for PlainEntry {
    fn from(entry: LibraryEntry) -> Self {
        Self::Library(entry.into())
    }
}

impl PlainEntry {
    fn template(self) -> FunctionTemplateId {
        match self {
            Self::Main(template) => template,
            Self::Library(entry) => entry.template(),
        }
    }
}

impl ReservedPlainEntry {
    fn seal(
        self,
    ) -> (
        SpecializationKey,
        specialization::Representability<SealedPlainEntry>,
    ) {
        match self {
            Self::Main { key, id } => (
                key,
                graph::seal_plain_runtime_function_id(id).map(SealedPlainEntry::Main),
            ),
            Self::Library(entry) => {
                let (key, entry) = entry.seal();
                (key, entry.map(SealedPlainEntry::Library))
            }
        }
    }
}

impl SealedPlainEntry {
    fn runtime_id(&self) -> execution_function::ProfiledRuntimeFunctionId<Infallible> {
        match self {
            Self::Main(function) => function.clone(),
            Self::Library(entry) => entry.runtime_id(),
        }
    }

    fn push(self, entries: &mut library::EntryIds) {
        if let Self::Library(entry) = self {
            entries.push(entry);
        }
    }
}

impl FunctionTemplates {
    fn new(templates: Vec<Vec<FunctionTemplate>>) -> Self {
        Self { templates }
    }

    fn get(&self, id: FunctionTemplateId) -> &FunctionTemplate {
        &self.templates[id.module().index()][id.index()]
    }

    fn entry_templates(&self) -> HashMap<FunctionTemplateId, super::local::FunctionEntryTemplate> {
        self.templates
            .iter()
            .flatten()
            .map(|template| {
                (
                    template.id(),
                    super::local::FunctionEntryTemplate::new(template),
                )
            })
            .collect()
    }
}

fn reserve_plain_entry(
    entry: PlainEntry,
    templates: &FunctionTemplates,
    context: &mut LoweringContext,
) -> ReservedPlainEntry {
    let key = SpecializationKey::monomorphic(entry.template());
    match entry {
        PlainEntry::Main(_) => {
            let return_shape = templates
                .get(key.template())
                .signature()
                .shape()
                .return_shape();
            let value_shape = specialization::SpecializedValueShape::instantiate(
                return_shape,
                key.substitution(),
            );
            let return_ = context.representations.inhabitation(&value_shape);
            let id = context.reserve_main(key.clone(), return_);
            ReservedPlainEntry::Main { key, id }
        }
        PlainEntry::Library(entry) => ReservedPlainEntry::Library(entry.reserve(context)),
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::function::{
        IntFunctionId, ProfiledCoreRuntimeFunctionId, ProfiledRuntimeFunctionId,
    };

    #[test]
    fn preserves_public_module_and_source_context() {
        let source = "pub fn main() { 1 }";
        let typed = crate::compile_typed_module("sample", "sample.gleam", source)
            .expect("source should compile");
        let context = crate::SourceContext::new("sample.gleam", source);
        let module =
            crate::plan_module_with_source(typed, context.clone()).expect("source should plan");
        let execution = crate::ExecutionPlan::from_module_plan(module);

        assert_eq!(execution.program.common.root, crate::plan::ModuleId::new(0));
        assert_eq!(execution.program.common.modules.len(), 1);
        assert_eq!(execution.program.common.modules[0].module, "sample");
        assert_eq!(
            execution.program.common.modules[0].source_context,
            Some(context),
        );
    }

    #[test]
    fn seeds_only_the_root_entry_and_preserves_module_sources() {
        let root_source = "pub fn main() { 7 }";
        let dependency_source = "pub fn main(value: Int) { value }";
        let typed = crate::compile_typed_program(
            "root",
            [
                crate::ModuleSource::new("root", "root.gleam", root_source),
                crate::ModuleSource::new("alpha", "alpha.gleam", dependency_source),
            ],
        )
        .expect("program should compile");
        let module = crate::plan_program(typed).expect("program should plan");
        let execution = crate::ExecutionPlan::from_module_plan(module);

        assert_eq!(execution.program.common.root, crate::plan::ModuleId::new(1));
        assert_eq!(
            execution.program.common.main,
            ProfiledRuntimeFunctionId::Core(ProfiledCoreRuntimeFunctionId::Int(IntFunctionId(0))),
        );
        assert_eq!(
            execution
                .program
                .common
                .modules
                .iter()
                .map(|module| {
                    (
                        module.module.as_str(),
                        module
                            .source_context
                            .as_ref()
                            .map(crate::SourceContext::source),
                    )
                })
                .collect::<Vec<_>>(),
            vec![
                ("alpha", Some(dependency_source)),
                ("root", Some(root_source))
            ],
        );
        assert_eq!(
            execution
                .program
                .functions
                .value_returns
                .int_functions
                .len(),
            1,
        );
    }

    #[test]
    fn deduplicates_cross_module_generic_specializations() {
        let typed = crate::compile_typed_program(
            "main",
            [
                crate::ModuleSource::new(
                    "generic",
                    "generic.gleam",
                    "pub fn identity(value: value) { value }",
                ),
                crate::ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
import generic

pub fn main() {
  #(
    generic.identity(1),
    generic.identity(2),
    generic.identity("three"),
  )
}
"#,
                ),
            ],
        )
        .expect("generic module program should compile");
        let module = crate::plan_program(typed).expect("generic module program should plan");
        let execution = crate::ExecutionPlan::from_module_plan(module);

        assert_eq!(
            execution
                .program
                .functions
                .value_returns
                .int_functions
                .len(),
            1,
        );
        assert_eq!(
            execution
                .program
                .functions
                .value_returns
                .string_functions
                .len(),
            1,
        );
        assert_eq!(
            execution
                .program
                .functions
                .value_returns
                .tuple_functions
                .len(),
            1,
        );
    }
}
