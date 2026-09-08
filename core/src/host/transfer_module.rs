use super::module::{
    HostModuleIdentity, RegisteredExternalTypes, RegisteredHostFunction,
    RegisteredHostImplementationId, RegisteredHostProviderModule, validate_function_name,
    validate_module_identities,
};
use super::{
    AsyncHostExternalBinding, HostExternalSchema, HostFunctionSchema, HostProfile, HostProvider,
    HostRegistrationError, HostTypeSequence, StatelessHostProfile, TransferHostFunctionDefinition,
    TransferHostFunctionImplementation, TransferScopedConstructingHostFunction,
    TransferScopedDivergingHostFunction, TransferScopedHostFunction,
};
use ecow::EcoString;
use std::collections::BTreeSet;
use std::sync::Arc;

/// Immediate Rust implementations for an ordinary Gleam module using transferable values.
///
/// Functions returning explicit work construct that work during this call. The
/// host drives the returned work separately; ordinary results remain immediate.
pub struct TransferHostProviderModule<Profile: HostProfile = StatelessHostProfile> {
    identity: HostModuleIdentity,
    functions: Vec<TransferHostFunctionDefinition<Profile>>,
    external_types: RegisteredExternalTypes,
}

/// Statically selected providers for one transferable Gleam program.
pub struct TransferHostProviderSet<Profile: HostProfile = StatelessHostProfile> {
    providers: Vec<TransferHostProviderModule<Profile>>,
}

pub(crate) struct RegisteredTransferHostImplementations<Profile: HostProfile> {
    functions: Vec<Arc<TransferHostFunctionImplementation<Profile>>>,
}

impl TransferHostProviderModule<StatelessHostProfile> {
    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
    ) -> Result<Self, HostRegistrationError> {
        Self::new_for_profile(package, module)
    }
}

impl<Profile: HostProfile> TransferHostProviderModule<Profile> {
    pub fn package(&self) -> &EcoString {
        &self.identity.package
    }

    pub fn module(&self) -> &EcoString {
        &self.identity.module
    }

    pub fn new_for_profile(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
    ) -> Result<Self, HostRegistrationError> {
        HostModuleIdentity::new(package.into(), module.into()).map(|identity| Self {
            identity,
            functions: Vec::new(),
            external_types: RegisteredExternalTypes::new(),
        })
    }

    pub fn with_scoped_function<Provider, Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: TransferScopedHostFunction<Profile, Provider, Arguments, Return>,
    {
        self.register(name.into(), |name| {
            TransferHostFunctionDefinition::new_scoped(name, function)
        })?;
        Ok(self)
    }

    pub fn with_scoped_function_and_constructions<
        Provider,
        Arguments,
        Return,
        Constructions,
        Function,
    >(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Constructions: HostTypeSequence,
        Function: TransferScopedConstructingHostFunction<
                Profile,
                Provider,
                Arguments,
                Return,
                Constructions,
            >,
    {
        self.register(name.into(), |name| {
            TransferHostFunctionDefinition::new_scoped_with_constructions::<
                Provider,
                Arguments,
                Return,
                Constructions,
                Function,
            >(name, function)
        })?;
        Ok(self)
    }

    pub fn with_scoped_diverging_function<Provider, Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: TransferScopedDivergingHostFunction<Profile, Provider, Arguments, Return>,
    {
        self.register(name.into(), |name| {
            TransferHostFunctionDefinition::new_scoped_diverging(name, function)
        })?;
        Ok(self)
    }

    pub fn with_external_type<Provider, Schema>(mut self) -> Result<Self, HostRegistrationError>
    where
        Schema: HostExternalSchema,
        Provider: AsyncHostExternalBinding<Profile, Schema>,
    {
        self.external_types.register(
            &self.identity.module,
            super::HostExternalTypeSchema::of::<Schema>(),
        )?;
        Ok(self)
    }

    pub fn schemas(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions
            .iter()
            .map(TransferHostFunctionDefinition::schema)
    }

    fn register(
        &mut self,
        name: EcoString,
        definition: impl FnOnce(
            EcoString,
        ) -> Result<
            TransferHostFunctionDefinition<Profile>,
            HostRegistrationError,
        >,
    ) -> Result<(), HostRegistrationError> {
        validate_function_name(&self.identity.module, &name)?;
        if self
            .functions
            .iter()
            .any(|function| function.schema().name() == &name)
        {
            return Err(HostRegistrationError::DuplicateFunction {
                module: self.identity.module.clone(),
                function: name,
            });
        }
        self.functions.push(definition(name)?);
        Ok(())
    }
}

impl<Profile: HostProfile> TransferHostProviderSet<Profile> {
    pub fn new(
        providers: impl IntoIterator<Item = TransferHostProviderModule<Profile>>,
    ) -> Result<Self, HostRegistrationError> {
        let providers: Vec<_> = providers.into_iter().collect();
        let identities: Vec<_> = providers
            .iter()
            .map(|provider| (&provider.identity.package, &provider.identity.module))
            .collect();
        validate_module_identities(&identities)?;
        Ok(Self { providers })
    }

    pub(crate) fn select_source_providers(
        mut self,
        selected: &BTreeSet<(EcoString, EcoString)>,
    ) -> Self {
        self.providers.retain(|provider| {
            selected.contains(&(
                provider.identity.package.clone(),
                provider.identity.module.clone(),
            ))
        });
        self
    }

    pub(crate) fn into_registered(
        self,
    ) -> (
        Vec<RegisteredHostProviderModule>,
        RegisteredTransferHostImplementations<Profile>,
    ) {
        let mut functions = Vec::new();
        let providers = self
            .providers
            .into_iter()
            .map(|provider| {
                let registered = provider
                    .functions
                    .into_iter()
                    .map(|function| {
                        let (schema, constructions, implementation) = function.into_parts();
                        let id = RegisteredHostImplementationId::new(functions.len());
                        functions.push(Arc::new(implementation));
                        RegisteredHostFunction::new(schema, constructions, id)
                    })
                    .collect();
                RegisteredHostProviderModule {
                    package: provider.identity.package,
                    module: provider.identity.module,
                    functions: registered,
                    external_types: provider.external_types.into_vec(),
                }
            })
            .collect();
        (
            providers,
            RegisteredTransferHostImplementations { functions },
        )
    }
}

impl<Profile: HostProfile> RegisteredTransferHostImplementations<Profile> {
    pub(crate) fn implementation(
        &self,
        id: RegisteredHostImplementationId,
    ) -> Arc<TransferHostFunctionImplementation<Profile>> {
        Arc::clone(&self.functions[id.index()])
    }
}

#[cfg(test)]
mod tests {
    use super::{TransferHostProviderModule, TransferHostProviderSet};
    use crate::embedding::{FunctionDeclaration, WorkModuleBuilder, with_execution_scope};
    use crate::frontend::compile_typed_transfer_host_program;
    use crate::host::{
        AsyncHostComponentProfile, HostFutureStore, HostProfile, HostRegistrationError, HostType,
        HostTypeParameter, TransferHostCall,
    };
    use crate::work_fixture::WorkType;
    use crate::work_fixture::{WorkComponent, WorkSchema};
    use crate::{AsyncHostCallError, HostCallCompletion, ModuleSource, PackageSource};
    use futures_util::FutureExt;
    use num_bigint::BigInt;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
    }
    impl crate::host::HostWorkProfile for Profile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl AsyncHostComponentProfile<WorkComponent> for Profile {
        fn component_async_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }

    fn identity<'call, Value: HostType>(
        mut call: TransferHostCall<'call, Profile, WorkComponent, Value>,
        value: Value::Value<'call>,
    ) -> Result<HostCallCompletion<'call, Value>, AsyncHostCallError> {
        assert_eq!(call.state(), &());
        Ok(call.return_value(value))
    }

    #[test]
    fn module_identity_and_schemas_describe_the_functions_that_execute() {
        let mut providers = WorkComponent::providers::<Profile>().expect("Future module");
        assert_eq!(providers[0].package(), "work_fixture");
        assert_eq!(providers[0].module(), "fixture/work");
        assert_eq!(providers[0].schemas().len(), 4);
        assert_eq!(
            providers[0]
                .schemas()
                .map(|schema| schema.name().as_str())
                .collect::<Vec<_>>(),
            ["ready", "map", "flatten", "all"]
        );
        providers.push(TransferHostProviderModule::new_for_profile("application", "library")
            .expect("module")
            .with_scoped_function::<WorkComponent, (HostTypeParameter<0>,), HostTypeParameter<0>, _>(
                "identity", identity::<HostTypeParameter<0>>,
            ).expect("generic identity"));
        let program = compile_typed_transfer_host_program(
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
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        r#"
import fixture/work as future
@external(erlang, "native", "identity")
fn identity(value: a) -> a
pub fn answer() { future.ready(identity(42)) }
"#,
                    )],
                ),
            ],
            TransferHostProviderSet::new(providers).expect("selected modules"),
        )
        .expect("source types");
        let (bindings, answer) = WorkModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("answer"))
            .expect("entry");
        let mut module = bindings.seal().expect("sealed execution");
        with_execution_scope(async |guard| {
            let mut state = ();
            let mut echo = drop;
            let mut execution = module.attach(guard, &mut state, &mut echo);
            let work = execution.call(&answer, ()).expect("constructed work");
            execution
                .observe(&work)
                .await
                .expect("completion")
                .read(|value| assert_eq!(value, &BigInt::from(42)));
        })
        .now_or_never()
        .expect("ready work");
    }

    #[test]
    fn constructing_and_never_functions_validate_names_before_their_bodies_run() {
        use crate::host::{HostConstructions, HostTypeListEnd};
        fn construct<'call>(
            call: TransferHostCall<'call, Profile, WorkComponent, BigInt>,
            _: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallCompletion<'call, BigInt>, AsyncHostCallError> {
            Ok(call.return_value(BigInt::from(42)))
        }
        fn stop(
            _: TransferHostCall<'_, Profile, WorkComponent, BigInt>,
        ) -> Result<std::convert::Infallible, AsyncHostCallError> {
            Err(crate::HostFailure::new("native stopped").into())
        }
        for name in ["Bad", "construct"] {
            let result = TransferHostProviderModule::new_for_profile("application", "library")
                .expect("module")
                .with_scoped_function_and_constructions::<WorkComponent, (), BigInt, HostTypeListEnd, _>(name, construct);
            assert_eq!(
                result.err(),
                if name == "Bad" {
                    Some(HostRegistrationError::InvalidFunctionName {
                        module: "library".into(),
                        function: name.into(),
                    })
                } else {
                    None
                }
            );
        }
        for name in ["Bad", "stop"] {
            let result = TransferHostProviderModule::new_for_profile("application", "library")
                .expect("module")
                .with_scoped_diverging_function::<WorkComponent, (), BigInt, _>(name, stop);
            assert_eq!(
                result.err(),
                if name == "Bad" {
                    Some(HostRegistrationError::InvalidFunctionName {
                        module: "library".into(),
                        function: name.into(),
                    })
                } else {
                    None
                }
            );
        }
        let provider = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("module")
            .with_scoped_function_and_constructions::<WorkComponent, (), BigInt, HostTypeListEnd, _>("construct", construct).expect("construct")
            .with_scoped_diverging_function::<WorkComponent, (), BigInt, _>("stop", stop).expect("stop");
        let program = compile_typed_transfer_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
@external(erlang, "native", "construct")
fn construct() -> Int
@external(erlang, "native", "stop")
fn stop() -> Int
pub fn run(fails: Bool) { case fails { True -> stop() False -> construct() } }
"#,
                )],
            )],
            TransferHostProviderSet::new([provider]).expect("providers"),
        )
        .expect("ordinary source");
        let (bindings, run) = WorkModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(bool,), BigInt>::new("run"))
            .expect("entry");
        let mut module = bindings.seal().expect("host specializations");
        let mut state = ();
        let mut echo = drop;
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            assert_eq!(
                scope.call(&run, (false,)).expect("direct result"),
                BigInt::from(42)
            );
            assert!(
                scope
                    .call(&run, (true,))
                    .expect_err("diverging result")
                    .to_string()
                    .contains("native stopped")
            );
        })
        .now_or_never()
        .expect("ordinary calls need no suspension");
    }

    #[test]
    fn function_validation_precedes_transfer_definition_assembly() {
        use crate::host::TransferHostFunctionDefinition;
        use std::cell::Cell;

        let mut module =
            TransferHostProviderModule::<Profile>::new_for_profile("application", "library")
                .expect("module");
        let assemblies = Cell::new(0);
        let assemble = |name: ecow::EcoString| {
            assemblies.set(assemblies.get() + 1);
            if name == "sparse" {
                TransferHostFunctionDefinition::new_scoped::<
                    WorkComponent,
                    (HostTypeParameter<1>,),
                    HostTypeParameter<1>,
                    _,
                >(name, identity::<HostTypeParameter<1>>)
            } else {
                TransferHostFunctionDefinition::new_scoped::<
                    WorkComponent,
                    (HostTypeParameter<0>,),
                    HostTypeParameter<0>,
                    _,
                >(name, identity::<HostTypeParameter<0>>)
            }
        };
        assert_eq!(
            module.register("Bad".into(), assemble).err(),
            Some(HostRegistrationError::InvalidFunctionName {
                module: "library".into(),
                function: "Bad".into(),
            })
        );
        assert_eq!(assemblies.get(), 0);
        module
            .register("identity".into(), assemble)
            .expect("valid definition");
        assert_eq!(assemblies.get(), 1);
        assert_eq!(
            module.register("identity".into(), assemble).err(),
            Some(HostRegistrationError::DuplicateFunction {
                module: "library".into(),
                function: "identity".into(),
            })
        );
        assert_eq!(assemblies.get(), 1);
        assert_eq!(
            module.register("sparse".into(), assemble).err(),
            Some(HostRegistrationError::NonContiguousTypeParameters {
                function: "sparse".into(),
                parameters: Box::new([1]),
            })
        );
        assert_eq!(assemblies.get(), 2);
        assert_eq!(module.schemas().len(), 1);
    }

    #[test]
    fn external_return_callbacks_preserve_nested_source_failures() {
        use crate::host::{HostCallable, HostFunctionType, HostTypeListEnd};
        use crate::work_fixture::WorkHostType;

        fn bridge<'call>(
            mut call: TransferHostCall<'call, Profile, WorkComponent, WorkHostType<BigInt>>,
            callback: HostCallable<'call, HostTypeListEnd, WorkHostType<BigInt>>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, AsyncHostCallError> {
            let value = call.invoke(callback, ())?;
            Ok(call.return_value(value))
        }
        fn reject<'call>(
            _: TransferHostCall<'call, Profile, WorkComponent, WorkHostType<BigInt>>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, AsyncHostCallError> {
            Err(crate::HostFailure::new("native rejected").into())
        }
        let mut providers = WorkComponent::providers::<Profile>().expect("Future module");
        providers.push(TransferHostProviderModule::new_for_profile("application", "library").expect("native module")
            .with_scoped_function::<WorkComponent, (HostFunctionType<HostTypeListEnd, WorkHostType<BigInt>>,), WorkHostType<BigInt>, _>("bridge", bridge).expect("callback")
            .with_scoped_function::<WorkComponent, (), WorkHostType<BigInt>, _>("reject", reject).expect("failure"));
        let source = r#"import fixture/work as future
@external(erlang, "native", "bridge")
fn bridge(callback: fn() -> future.Work(Int)) -> future.Work(Int)
@external(erlang, "native", "reject")
fn reject() -> future.Work(Int)
pub fn run(mode: Int) {
  case mode {
    2 -> reject()
    _ -> bridge(fn() {
      case mode {
        1 -> panic as "source rejected"
        _ -> future.ready(42)
      }
    })
  }
}
"#;
        let program = compile_typed_transfer_host_program(
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
            TransferHostProviderSet::new(providers).expect("providers"),
        )
        .expect("ordinary callback source");
        let (bindings, run) = WorkModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(BigInt,), WorkType<BigInt>>::new(
                "run",
            ))
            .expect("entry");
        let mut module = bindings.seal().expect("sealed functions");
        let mut state = ();
        let mut echo = drop;
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            for mode in 0..3 {
                match scope.call(&run, (mode.into(),)) {
                    Ok(work) => {
                        assert_eq!(mode, 0);
                        scope
                            .observe(&work)
                            .await
                            .expect("ready work")
                            .read(|value| assert_eq!(value, &BigInt::from(42)));
                    }
                    Err(error) => assert!(error.to_string().contains(if mode == 1 {
                        "source rejected"
                    } else {
                        "native rejected"
                    })),
                }
            }
        })
        .now_or_never()
        .expect("all source calls finish immediately");
    }

    #[test]
    fn registration_rejects_invalid_and_duplicate_names_before_execution() {
        assert_eq!(
            TransferHostProviderModule::new("application", "Bad").err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "Bad".into()
            })
        );
        for name in ["Bad", "identity"] {
            let error = TransferHostProviderModule::new_for_profile("application", "library")
                .expect("module")
                .with_scoped_function::<WorkComponent, (HostTypeParameter<0>,), HostTypeParameter<0>, _>(
                    "identity", identity::<HostTypeParameter<0>>,
                ).expect("first registration")
                .with_scoped_function::<WorkComponent, (HostTypeParameter<0>,), HostTypeParameter<0>, _>(
                    name, identity::<HostTypeParameter<0>>,
                ).err();
            let expected = if name == "Bad" {
                HostRegistrationError::InvalidFunctionName {
                    module: "library".into(),
                    function: name.into(),
                }
            } else {
                HostRegistrationError::DuplicateFunction {
                    module: "library".into(),
                    function: name.into(),
                }
            };
            assert_eq!(error, Some(expected));
        }
        let error = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("module")
            .with_scoped_function::<WorkComponent, (HostTypeParameter<1>,), HostTypeParameter<1>, _>(
                "identity", identity::<HostTypeParameter<1>>,
            ).err();
        assert_eq!(
            error,
            Some(HostRegistrationError::NonContiguousTypeParameters {
                function: "identity".into(),
                parameters: Box::new([1]),
            })
        );
        let error =
            TransferHostProviderModule::<Profile>::new_for_profile("work_fixture", "fixture/work")
                .expect("module")
                .with_external_type::<WorkComponent, WorkSchema>()
                .expect("first type")
                .with_external_type::<WorkComponent, WorkSchema>()
                .err();
        assert_eq!(
            error,
            Some(HostRegistrationError::DuplicateExternalType {
                module: "fixture/work".into(),
                type_: "Work".into(),
            })
        );
        let error = TransferHostProviderSet::new([
            TransferHostProviderModule::new("first", "library").expect("first"),
            TransferHostProviderModule::new("second", "library").expect("second"),
        ])
        .err();
        assert_eq!(
            error,
            Some(HostRegistrationError::DuplicateModule {
                module: "library".into(),
                first_package: "first".into(),
                second_package: "second".into(),
            })
        );
    }
}
