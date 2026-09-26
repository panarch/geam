mod callable;
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
use crate::host::{HostFunctionBinding, HostProfile};
use crate::plan::execution::function::RuntimeFunctionId;
use crate::plan::execution::host::{
    CallableRegistration, HostBindingTables, HostFunctionTables, HostSpecializationError,
    HostedExecutionProfile,
};
use crate::plan::execution::storage::Table;
use crate::plan::execution::{ExecutionModuleContext, ExecutionProgram, ExecutionProgramCommon};
use crate::plan::execution::{LibraryFunctionEntries, LibraryNativeConstruction};
use crate::plan::{
    FunctionTemplateId, FunctionType, HostedModulePlan, HostedModulePlanParts, HostedPlannedModule,
    LibraryEntry, LibraryNativeCallable, ModuleId, ProfiledHostedLibraryModulePlan,
    ProfiledHostedLibraryModulePlanParts,
};
use std::collections::HashSet;
use std::sync::Arc;
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

type LoweredHostedLibrary<Value, Never> = (
    ExecutionProgram<HostedExecutionProfile>,
    HostBindingTables<Value, Never>,
    LibraryFunctionEntries,
    Table<LibraryNativeConstruction>,
);

pub(in crate::plan::execution) fn lower_hosted_library<Value: Clone, Never: Clone + From<Value>>(
    module_plan: ProfiledHostedLibraryModulePlan<HostFunctionBinding<Value, Never>>,
    first: LibraryEntry,
    remaining: Vec<LibraryEntry>,
) -> Result<LoweredHostedLibrary<Value, Never>, HostSpecializationError> {
    let ProfiledHostedLibraryModulePlanParts {
        root,
        modules,
        implementation_bindings,
        callables,
    } = module_plan.into_parts();
    let implementations = HostFunctionRegistry::new(implementation_bindings);
    lower_hosted_entries(
        HostedLoweringInput { root, modules },
        LibraryEntries {
            functions: library::Entries::new(first, remaining),
            callables,
        },
        implementations,
    )
    .map(|(program, functions, (entries, callables))| (program, functions, entries, callables))
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
                        types.into_tables(&representations);
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
    template: FunctionTemplateId,
}

struct LibraryEntries {
    functions: library::Entries,
    callables: Vec<LibraryNativeCallable>,
}

trait HostedEntries {
    type Reserved;
    type Output;

    fn initial_key(&self) -> SpecializationKey;

    fn invalid_callback(
        &self,
        _context: &LoweringContext,
    ) -> Option<(FunctionTemplateId, FunctionType)> {
        None
    }

    fn reserve(
        &self,
        templates: &HostTemplateCatalog,
        context: &mut LoweringContext,
    ) -> Result<Self::Reserved, HostSpecializationError>;

    fn seal(reserved: Self::Reserved) -> SpecializationOutcome<(RuntimeFunctionId, Self::Output)>;
}

type LoweredHostedEntries<Entries, Value, Never> = (
    ExecutionProgram<HostedExecutionProfile>,
    HostBindingTables<Value, Never>,
    <Entries as HostedEntries>::Output,
);

fn lower_hosted_entries<Entries, Value, Never>(
    input: HostedLoweringInput,
    entries: Entries,
    implementations: HostFunctionRegistry<Value, Never>,
) -> Result<LoweredHostedEntries<Entries, Value, Never>, HostSpecializationError>
where
    Entries: HostedEntries,
    Value: Clone,
    Never: Clone + From<Value>,
{
    let HostedLoweringInput { root, modules } = input;
    let mut module_contexts = Vec::with_capacity(modules.len());
    let mut module_packages = Vec::with_capacity(modules.len());
    let mut templates = HostTemplateCatalog::new();
    let mut constant_templates = Vec::with_capacity(modules.len());
    let mut custom_types = Vec::new();

    for module in modules {
        let parts = module.into_parts();
        module_packages.push(parts.package);
        module_contexts.push(ExecutionModuleContext::new(
            parts.module,
            parts.source_context,
        ));
        custom_types.extend(parts.custom_types);
        constant_templates.push(parts.constants);
        templates.push_module(
            parts.functions,
            parts.anonymous_functions,
            parts.native_callables,
        );
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
            if let Some((id, callback)) = entries.invalid_callback(&context) {
                let template = templates.get(id);
                let name = match template {
                    HostLoweringTemplate::Gleam(template) => template.name().to_string(),
                    HostLoweringTemplate::Host(template) => template.name().to_string(),
                };
                return Err(HostSpecializationError::uninhabited_callback_arguments(
                    module_packages[id.module().index()].clone(),
                    module_contexts[id.module().index()].module.as_str().into(),
                    name.into(),
                    template.signature().shape().type_(),
                    callback,
                ));
            }
            let reserved_entries = entries.reserve(&templates, &mut context)?;
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
    let LoweredExecution {
        constants,
        functions,
        function_parameters,
        list_types,
        custom_types,
        external_types,
        value_shapes,
    } = *lowered;
    ExecutionProgram {
        common: Arc::new(ExecutionProgramCommon {
            root,
            modules: modules.into(),
            main,
            constants: Box::new(constants).into(),
            function_parameters: Arc::new(function_parameters),
            list_types: Arc::new(list_types),
            custom_types: Arc::new(custom_types),
            external_types: Arc::new(external_types),
            value_shapes: Box::new(value_shapes).into(),
        }),
        functions: Box::new(functions).into(),
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
    ) -> Result<Self::Reserved, HostSpecializationError> {
        let key = self.initial_key();
        let return_shape = templates
            .get(self.template)
            .signature()
            .shape()
            .return_shape();
        let value_shape = SpecializedValueShape::instantiate(return_shape, key.substitution());
        let return_ = context.representations.inhabitation(&value_shape);
        Ok(context.reserve_main(key, return_))
    }

    fn seal(reserved: Self::Reserved) -> SpecializationOutcome<(RuntimeFunctionId, Self::Output)> {
        SpecializationOutcome::Complete((reserved, ()))
    }
}

impl HostedEntries for LibraryEntries {
    type Reserved = (library::ReservedEntries, Vec<LibraryNativeConstruction>);
    type Output = (LibraryFunctionEntries, Table<LibraryNativeConstruction>);

    fn initial_key(&self) -> SpecializationKey {
        self.functions.initial_key()
    }

    fn invalid_callback(
        &self,
        context: &LoweringContext,
    ) -> Option<(FunctionTemplateId, FunctionType)> {
        self.functions.invalid_callback(context).or_else(|| {
            let key = self.initial_key();
            self.callables.iter().find_map(|entry| {
                library::invalid_callback(
                    std::slice::from_ref(&entry.signature.invocation),
                    &key,
                    context,
                )
                .map(|callback| (entry.template.id(), callback))
            })
        })
    }

    fn reserve(
        &self,
        _templates: &HostTemplateCatalog,
        context: &mut LoweringContext,
    ) -> Result<Self::Reserved, HostSpecializationError> {
        let functions = self.functions.reserve(context);
        let key = self.initial_key();
        let callables = self
            .callables
            .iter()
            .map(|entry| {
                let construction = callable::seal_construction(
                    &entry.template,
                    &entry.declaration,
                    &entry.target,
                    key.substitution(),
                    context,
                )?;
                Ok(LibraryNativeConstruction {
                    declaration: CallableRegistration::from_registered(&entry.declaration),
                    construction,
                    invocation: context.library_callable(&key, &entry.signature.invocation),
                    captures: context.library_input_constructions(
                        &key,
                        &entry.signature.capture_variants,
                        &entry.signature.capture_lists,
                    ),
                })
            })
            .collect::<Result<Vec<_>, HostSpecializationError>>()?;
        Ok((functions, callables))
    }

    fn seal(reserved: Self::Reserved) -> SpecializationOutcome<(RuntimeFunctionId, Self::Output)> {
        reserved
            .0
            .seal()
            .map(library::SealedEntries::finish)
            .map(|(main, entries)| (main, (entries, reserved.1.into())))
    }
}

#[cfg(test)]
mod tests {
    use super::lower_hosted_library;
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::execution_fixture::TestHost;
    use crate::frontend::HostedTypedProgram;
    use crate::host::native::{NativeCall, NativeRules};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostCallable, HostComponentProfile,
        HostCustomConstructorListEnd, HostCustomSchema, HostCustomType, HostFunctionType,
        HostFutureStore, HostProfile, HostProvider, HostProviderModule, HostType, HostTypeList,
        HostTypeListEnd, HostTypeParameter, HostWorkProfile,
    };
    use crate::plan::{
        ExternalType, ExternalTypeName, FunctionType, LibraryEntry, LibraryValueType,
        LibraryVariant, TypeParameterId, ValueType,
    };
    use crate::planner::plan_host_library_program;
    use crate::work_fixture::WorkComponent;
    use crate::{HostFailure, HostSpecializationErrorReason};
    use crate::{
        HostModule, HostProviderSet, ModuleSource, PackageSource, compile_typed_host_program,
    };

    use num_bigint::BigInt;
    use std::convert::Infallible;

    #[test]
    fn every_hosted_library_family_can_own_the_first_entry() {
        use crate::plan::StandardVariant;
        struct Profile;
        impl HostProfile for Profile {
            type RunState = ();
            type ExternalStores = HostFutureStore;
            type ExecutionState = ();
        }
        impl HostWorkProfile for Profile {
            type Work = WorkComponent;
        }
        impl HostComponentProfile<WorkComponent> for Profile {
            fn component_stores(stores: &Self::ExternalStores) -> &Self::ExternalStores {
                stores
            }
            fn component_state(state: &mut ()) -> &mut () {
                state
            }
        }
        let mut state = ();
        let stores = HostFutureStore::default();
        assert!(std::ptr::eq(
            <Profile as HostComponentProfile<WorkComponent>>::component_stores(&stores),
            &stores,
        ));
        assert_eq!(
            <WorkComponent as HostProvider<Profile>>::project(&mut state),
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
                vec![LibraryVariant::new(
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
            let program = compile_typed_host_program(
                "application",
                "library",
                [
                    PackageSource::new(
                        "work_fixture",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "fixture/work",
                            "src/fixture/work.gleam",
                            WorkComponent::SOURCE,
                        )],
                    ),
                    PackageSource::new(
                        "application",
                        ["work_fixture"],
                        [ModuleSource::new("library", "src/library.gleam", source)],
                    ),
                ],
                HostProviderSet::from_providers(
                    WorkComponent::providers::<Profile>().expect("Future provider"),
                )
                .expect("providers"),
            )
            .expect("source identity");
            let plan = plan_host_library_program(program).expect("typed library");
            let template = plan
                .functions()
                .iter()
                .find(|function| function.name() == "identity")
                .expect("identity")
                .signature()
                .id();
            let (_, _, entries, _) = lower_hosted_library(
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
        let plan = plan_host_library_program(program).expect("hosted library should plan");
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

        let (_, host_functions, entries, _) =
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
        type ExecutionState = ();
    }
    impl HostWorkProfile for Profile {
        type Work = WorkComponent;
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
        let execution_host = TestHost::default();
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
    fn generic_producers_seal_and_only_execute_when_called() {
        for (body, invoked) in [
            ("let _ = produce 42", false),
            ("let _ = produce() 42", true),
            ("produce() + 1", true),
        ] {
            let host = HostProviderModule::new("application", "library")
                .expect("provider")
                .with_scoped_function::<Provider, (), Generic, _>("produce", produce)
                .expect("generic producer");
            let source = format!(
                r#"
@external(erlang, "native", "produce")
fn produce() -> value
pub fn run() {{ {body} }}
"#
            );
            let (bindings, run) = HostedModuleBuilder::new(program(&source, host))
                .expect("valid plan")
                .function(FunctionDeclaration::<(), BigInt>::new("run"))
                .expect("binding");
            let mut module = bindings.seal().expect("generic producer should seal");
            let execution_host = TestHost::default();
            let returned = execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut (),
                    &mut drop,
                    async |scope| scope.call(&run, ()).await,
                ))
                .unwrap();
            if invoked {
                assert_eq!(
                    returned.unwrap_err().to_string(),
                    "host function application::library.produce failed: native producer failed"
                );
            } else {
                assert_eq!(returned.unwrap(), BigInt::from(42));
            }
        }
    }

    #[test]
    fn generic_return_specializations_follow_existing_inhabitation() {
        use crate::plan::execution::host::HostFunctionCompletion;
        for (return_type, completion) in [
            ("a", HostFunctionCompletion::Uninhabited),
            ("#(a, Int)", HostFunctionCompletion::Uninhabited),
            ("Required(a)", HostFunctionCompletion::Uninhabited),
            ("Never", HostFunctionCompletion::Uninhabited),
            ("List(a)", HostFunctionCompletion::Value),
            ("Optional(a)", HostFunctionCompletion::Value),
            ("fn() -> a", HostFunctionCompletion::Value),
        ] {
            let source = format!(
                r#"
pub type Never
pub type Required(a) {{ Required(a) }}
pub type Optional(a) {{ Absent Present(a) }}
@external(erlang, "native", "produce")
fn produce() -> value
fn selected() -> {return_type} {{ produce() }}
pub fn run() {{ let _ = selected() 42 }}
"#
            );
            let provider = HostProviderModule::new("application", "library")
                .unwrap()
                .with_scoped_function::<Provider, (), Generic, _>("produce", produce)
                .unwrap();
            let plan = plan_host_library_program(program(&source, provider)).unwrap();
            let selected = plan
                .functions()
                .iter()
                .find(|function| function.name() == "run")
                .unwrap();
            let entry = LibraryEntry::new(
                selected.signature().id(),
                LibraryValueType::Int,
                Vec::new(),
                Vec::new(),
            );
            let (_, functions, _, _) = lower_hosted_library(plan, entry, Vec::new()).unwrap();
            let metadata = if completion == HostFunctionCompletion::Value {
                assert!(functions.never_functions().is_empty(), "{return_type}");
                assert_eq!(functions.value_functions().len(), 1, "{return_type}");
                functions.value_functions()[0].metadata()
            } else {
                assert!(functions.value_functions().is_empty(), "{return_type}");
                assert_eq!(functions.never_functions().len(), 1, "{return_type}");
                functions.never_functions()[0].metadata()
            };
            assert_eq!(metadata.completion, completion, "{return_type}");
        }
    }

    #[test]
    fn native_rules_reject_overlap_only_after_generic_specialization() {
        use crate::work_fixture::WorkSchema;
        type Other = HostTypeParameter<1>;
        type FirstArguments = HostTypeList<Generic, HostTypeListEnd>;
        type SecondArguments = HostTypeList<Other, HostTypeListEnd>;
        fn ready<'call>(
            call: NativeCall<'call, Profile, Provider, bool, HostTypeListEnd>,
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
                    NativeRules::default()
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
                        type_: ValueType::External(ExternalType::new(
                            ExternalTypeName::new(
                                "work_fixture".into(),
                                "fixture/work".into(),
                                "Work".into()
                            ),
                            vec![ValueType::Int],
                        )),
                    }
                );
            } else {
                let mut module = bindings.seal().expect("distinct specialized rules");
                let execution_host = TestHost::default();
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
                    callback: FunctionType::new(
                        vec![ValueType::Parameter(TypeParameterId(0))],
                        ValueType::Int,
                    ),
                }
            );
        }
    }

    #[test]
    fn inhabited_specializations_execute_the_registered_value_and_diverging_callbacks() {
        let execution_host = TestHost::default();

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
        let execution_host = TestHost::default();
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
