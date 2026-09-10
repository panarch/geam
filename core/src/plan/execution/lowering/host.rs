mod native;
mod parameter;
mod return_;
mod sealing;
mod table;
mod template;

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

impl LoweringContext {
    fn finish_hosted(
        self,
        additional: function::ProfiledFunctionEntries<HostedExecutionProfile>,
    ) -> LoweringCompletion<LoweredExecution<HostedExecutionProfile>> {
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

type LoweredHostedEntries<Entries, Profile> = (
    ExecutionProgram<HostedExecutionProfile>,
    HostFunctionTables<Profile>,
    <Entries as HostedEntries>::Output,
);

fn lower_hosted_entries<Entries, Profile>(
    input: HostedLoweringInput,
    entries: Entries,
    implementations: HostFunctionRegistry<Profile>,
) -> Result<LoweredHostedEntries<Entries, Profile>, HostSpecializationError>
where
    Entries: HostedEntries,
    Profile: HostProfile,
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

    let programs = assemble_hosted_program(root, module_contexts.into_boxed_slice(), main, lowered);
    Ok((programs, host_functions, entry_output))
}

fn assemble_hosted_program(
    root: ModuleId,
    modules: Box<[ExecutionModuleContext]>,
    main: RuntimeFunctionId,
    lowered: Box<LoweredExecution<HostedExecutionProfile>>,
) -> ExecutionProgram<HostedExecutionProfile> {
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
            list_types: std::sync::Arc::new(list_types),
            custom_types: std::sync::Arc::new(custom_types),
            external_types: std::sync::Arc::new(external_types),
            value_shapes,
        }),
        functions,
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
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::frontend::HostedTypedProgram;
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostCallable, HostComponentProfile,
        HostCustomConstructorListEnd, HostCustomSchema, HostCustomType, HostFunctionType,
        HostFutureStore, HostProfile, HostProvider, HostProviderModule, HostType, HostTypeList,
        HostTypeListEnd, HostTypeParameter,
    };
    use crate::plan::{LibraryEntry, LibraryValueType};
    use crate::work_fixture::WorkComponent;
    use crate::{HostFailure, HostSpecializationErrorReason};
    use crate::{
        HostModule, HostProviderSet, ModuleSource, PackageSource, compile_typed_host_program,
    };

    use num_bigint::BigInt;
    use std::convert::Infallible;

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
        impl crate::host::HostComponentProfile<crate::work_fixture::WorkComponent> for Profile {
            fn component_stores(stores: &Self::ExternalStores) -> &Self::ExternalStores {
                stores
            }
            fn component_state(state: &mut ()) -> &mut () {
                state
            }
        }
        let mut state = ();
        let stores = crate::host::HostFutureStore::default();
        assert!(
            std::ptr::eq(
                <Profile as crate::host::HostComponentProfile<
                    crate::work_fixture::WorkComponent,
                >>::component_stores(&stores),
                &stores,
            )
        );
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
                vec![crate::plan::LibraryVariant::new(
                    StandardVariant::Result,
                    vec![ValueType::Int, ValueType::String],
                )],
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
            let program = crate::frontend::compile_typed_host_program(
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
                crate::host::HostProviderSet::from_providers(
                    crate::work_fixture::WorkComponent::providers::<Profile>()
                        .expect("Future provider"),
                )
                .expect("providers"),
            )
            .expect("source identity");
            let plan = crate::planner::plan_host_library_program(program).expect("typed library");
            let template = plan
                .functions()
                .iter()
                .find(|function| function.name() == "identity")
                .expect("identity")
                .signature()
                .id();
            let (_, _, entries) = super::lower_hosted_library(
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

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
    }
    impl crate::host::HostWorkProfile for Profile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }
    struct Provider;
    impl HostProvider<Profile> for Provider {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    fn program(source: &str, provider: HostProviderModule<Profile>) -> HostedTypedProgram<Profile> {
        compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            HostProviderSet::from_providers([provider]).expect("provider set"),
        )
        .expect("valid source")
    }

    struct NeverSchema;
    impl HostCustomSchema for NeverSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Never";
        const PARAMETER_COUNT: usize = 0;
        type Constructors = HostCustomConstructorListEnd;
    }
    type Never = HostCustomType<NeverSchema>;
    type Generic = HostTypeParameter<0>;
    type CallbackArgs = HostTypeList<Generic, HostTypeListEnd>;
    type Callback = HostFunctionType<CallbackArgs, BigInt>;

    fn accept_never<'call, Value: HostType>(
        mut call: HostCall<'call, Profile, Provider, BigInt>,
        _: Value::Value<'call>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        assert_eq!(call.state(), &());
        Ok(call.return_value(1.into()))
    }

    fn produce<'call>(
        _: HostCall<'call, Profile, Provider, Generic>,
    ) -> Result<HostCallCompletion<'call, Generic>, HostCallError> {
        Err(HostFailure::new("native producer failed").into())
    }

    fn accept_callback<'call>(
        call: HostCall<'call, Profile, Provider, BigInt>,
        _: HostCallable<'call, CallbackArgs, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        Ok(call.return_value(1.into()))
    }

    fn diverge_callback<'call>(
        _: HostCall<'call, Profile, Provider, BigInt>,
        _: HostCallable<'call, CallbackArgs, BigInt>,
    ) -> Result<Infallible, HostCallError> {
        Err(HostFailure::new("native callback failed").into())
    }

    fn diverge<'call, Return: HostType>(
        _: HostCall<'call, Profile, Provider, Return>,
    ) -> Result<Infallible, HostCallError> {
        Err(HostFailure::new("native execution stopped").into())
    }

    #[test]
    fn erases_an_uninhabited_provider_specialization_before_direct_execution() {
        let host = HostProviderModule::new("application", "library")
            .expect("provider")
            .with_scoped_function::<Provider, (Never,), BigInt, _>(
                "accept_never",
                accept_never::<Never>,
            )
            .expect("Never argument");
        let source = r#"
pub type Never
@external(erlang, "native", "accept_never")
fn accept_never(value: Never) -> Int
pub fn run() { let _ = accept_never 42 }
"#;
        let (bindings, run) = HostedModuleBuilder::new(program(source, host))
            .expect("plan")
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .expect("binding");
        let mut module = bindings
            .seal()
            .expect("uninhabited specialization is erased");
        let mut state = ();
        let mut echo = drop;
        let execution_host = crate::execution_fixture::TestHost::default();
        execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    assert_eq!(
                        scope.call(&run, ()).await.expect("direct entry"),
                        BigInt::from(42)
                    );
                },
            ))
            .expect("hosted call completes");
    }

    #[test]
    fn rejects_a_value_producer_with_unresolved_return_storage_at_sealing() {
        let host = HostProviderModule::new("application", "library")
            .expect("provider")
            .with_scoped_function::<Provider, (), Generic, _>("produce", produce)
            .expect("generic producer");
        let source = r#"
@external(erlang, "native", "produce")
fn produce() -> value
pub fn run() { let _ = produce 42 }
"#;
        let (bindings, _) = HostedModuleBuilder::new(program(source, host))
            .expect("valid plan")
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .expect("binding");
        let error = bindings
            .seal()
            .err()
            .expect("unresolved producer cannot seal");
        assert_eq!(error.function(), "produce");
        assert_eq!(
            error.reason(),
            &HostSpecializationErrorReason::UndeterminedReturnStorage
        );
    }

    #[test]
    fn native_rules_reject_overlap_only_after_generic_specialization() {
        use crate::work_fixture::WorkSchema;
        type Other = HostTypeParameter<1>;
        type FirstArguments = HostTypeList<Generic, HostTypeListEnd>;
        type SecondArguments = HostTypeList<Other, HostTypeListEnd>;
        fn ready<'call>(
            call: crate::host::native::NativeCall<'call, Profile, Provider, bool, HostTypeListEnd>,
            _: <Generic as HostType>::Value<'call>,
            _: <Other as HostType>::Value<'call>,
        ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
            Ok(call.finish(true))
        }
        for (source, overlaps) in [
            (
                r#"
@external(erlang, "native", "ready")
fn ready(left: a, right: b) -> Bool
pub fn run() { ready(1, 2) }
"#,
                true,
            ),
            (
                r#"
@external(erlang, "native", "ready")
fn ready(left: a, right: b) -> Bool
pub fn run() { ready(1, "two") }
"#,
                false,
            ),
        ] {
            let host = HostProviderModule::new("application", "library")
                .unwrap()
                .with_native_function::<Provider, (Generic, Other), bool, HostTypeListEnd, _>(
                    "ready",
                    crate::host::native::NativeRules::default()
                        .external::<WorkSchema, FirstArguments>(|_, _, _| None)
                        .external::<WorkSchema, SecondArguments>(|_, _, _| None),
                    ready,
                )
                .unwrap();
            let work = HostProviderModule::new("work_fixture", "fixture/work")
                .unwrap()
                .with_external_type::<WorkComponent, WorkSchema>()
                .unwrap();
            let typed = compile_typed_host_program(
                "application",
                "library",
                [
                    PackageSource::new(
                        "application",
                        ["work_fixture"],
                        [ModuleSource::new("library", "src/library.gleam", source)],
                    ),
                    PackageSource::new(
                        "work_fixture",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "fixture/work",
                            "src/fixture/work.gleam",
                            "pub type Work(a)",
                        )],
                    ),
                ],
                HostProviderSet::from_providers([host, work]).unwrap(),
            )
            .unwrap();
            let (bindings, run) = HostedModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(), bool>::new("run"))
                .unwrap();
            if overlaps {
                let error = bindings
                    .seal()
                    .err()
                    .expect("same specialization must conflict");
                assert_eq!(error.function(), "ready");
                assert_eq!(
                    error.reason(),
                    &HostSpecializationErrorReason::ConflictingNativeConversions {
                        type_: crate::ValueType::External(crate::ExternalType::new(
                            crate::ExternalTypeName::new(
                                "work_fixture".into(),
                                "fixture/work".into(),
                                "Work".into()
                            ),
                            vec![crate::ValueType::Int],
                        )),
                    }
                );
            } else {
                let mut module = bindings.seal().expect("distinct specialized rules");
                let execution_host = crate::execution_fixture::TestHost::default();
                execution_host
                    .block_on(module.with_execution(
                        &execution_host,
                        &mut (),
                        &mut drop,
                        async |scope| {
                            assert!(scope.call(&run, ()).await.unwrap());
                        },
                    ))
                    .expect("direct entry");
            }
        }
    }

    #[test]
    fn rejects_symbolic_callback_arguments_for_value_and_diverging_providers() {
        let source = r#"
@external(erlang, "native", "accept")
fn accept(callback: fn(value) -> Int) -> Int
fn generic(_value) { 1 }
pub fn run() { accept(generic) }
"#;
        let value = HostProviderModule::new("application", "library")
            .expect("provider")
            .with_scoped_function::<Provider, (Callback,), BigInt, _>("accept", accept_callback)
            .expect("value callback");
        let diverging = HostProviderModule::new("application", "library")
            .expect("provider")
            .with_scoped_diverging_function::<Provider, (Callback,), BigInt, _>(
                "accept",
                diverge_callback,
            )
            .expect("diverging callback");
        for host in [value, diverging] {
            let (bindings, _) = HostedModuleBuilder::new(program(source, host))
                .expect("valid plan")
                .function(FunctionDeclaration::<(), BigInt>::new("run"))
                .expect("binding");
            let error = bindings
                .seal()
                .err()
                .expect("symbolic callback cannot be invoked");
            assert_eq!(error.function(), "accept");
            assert_eq!(
                error.reason(),
                &HostSpecializationErrorReason::UninhabitedCallbackArguments {
                    callback: crate::FunctionType::new(
                        vec![crate::ValueType::Parameter(crate::plan::TypeParameterId(0))],
                        crate::ValueType::Int,
                    ),
                }
            );
        }
    }

    #[test]
    fn inhabited_specializations_execute_the_registered_value_and_diverging_callbacks() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let host = HostProviderModule::new("application", "library")
            .expect("provider")
            .with_scoped_function::<Provider, (BigInt,), BigInt, _>(
                "accept_value",
                accept_never::<BigInt>,
            )
            .expect("inhabited value")
            .with_scoped_function::<Provider, (), Generic, _>("produce", produce)
            .expect("generic producer")
            .with_scoped_function::<Provider, (Callback,), BigInt, _>("accept", accept_callback)
            .expect("inhabited callback")
            .with_scoped_diverging_function::<Provider, (Callback,), BigInt, _>(
                "stop",
                diverge_callback,
            )
            .expect("inhabited diverging callback");
        let source = r#"
@external(erlang, "native", "accept_value")
fn accept_value(value: Int) -> Int
@external(erlang, "native", "produce")
fn produce() -> value
@external(erlang, "native", "accept")
fn accept(callback: fn(value) -> Int) -> Int
@external(erlang, "native", "stop")
fn stop(callback: fn(value) -> Int) -> Int
fn concrete(value: Int) { value + 1 }
pub fn accepted() { accept_value(42) + accept(concrete) }
pub fn failed() -> Int { produce() }
pub fn stopped() { stop(concrete) }
"#;
        let (mut bindings, accepted) = HostedModuleBuilder::new(program(source, host))
            .expect("plan")
            .function(FunctionDeclaration::<(), BigInt>::new("accepted"))
            .expect("value binding");
        let failed = bindings
            .function(FunctionDeclaration::<(), BigInt>::new("failed"))
            .expect("concrete producer");
        let stopped = bindings
            .function(FunctionDeclaration::<(), BigInt>::new("stopped"))
            .expect("diverging provider");
        let mut module = bindings.seal().expect("inhabited source types");
        let mut state = ();
        let stores = HostFutureStore::default();
        assert!(std::ptr::eq(Profile::component_stores(&stores), &stores));
        assert!(std::ptr::eq(
            <WorkComponent as HostProvider<Profile>>::project(&mut state),
            &state
        ));
        let mut echo = drop;
        execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |execution| {
                    assert_eq!(
                        execution
                            .call(&accepted, ())
                            .await
                            .expect("accepted values"),
                        BigInt::from(2)
                    );
                    assert_eq!(
                        execution
                            .call(&failed, ())
                            .await
                            .expect_err("native producer fails")
                            .to_string(),
                        "host function application::library.produce failed: native producer failed"
                    );
                    assert_eq!(
                        execution
                            .call(&stopped, ())
                            .await
                            .expect_err("native callback fails")
                            .to_string(),
                        "host function application::library.stop failed: native callback failed"
                    );
                },
            ))
            .expect("direct executions");
    }

    #[test]
    fn a_diverging_native_never_return_keeps_the_host_failure_boundary() {
        let host = HostProviderModule::new("application", "library")
            .expect("provider")
            .with_scoped_diverging_function::<Provider, (), Never, _>("stop", diverge::<Never>)
            .expect("uninhabited return");
        let source = r#"
pub type Never
@external(erlang, "native", "stop")
fn stop() -> Never
pub fn run() { let _ = stop() 42 }
"#;
        let (bindings, run) = HostedModuleBuilder::new(program(source, host))
            .expect("plan")
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .expect("root");
        let mut module = bindings
            .seal()
            .expect("diverging target has no returned value");
        let mut state = ();
        let mut echo = drop;
        let execution_host = crate::execution_fixture::TestHost::default();
        execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    assert_eq!(
                        scope
                            .call(&run, ())
                            .await
                            .expect_err("no fabricated Never value")
                            .to_string(),
                        "host function application::library.stop failed: native execution stopped"
                    );
                },
            ))
            .expect("direct failure");
    }
}
