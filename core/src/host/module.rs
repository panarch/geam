use super::{
    FallibleHostFunction, HostExternalBinding, HostExternalSchema, HostExternalTypeSchema,
    HostFunction, HostFunctionDefinition, HostFunctionImplementation, HostFunctionSchema,
    HostProfile, HostProvider, HostRegistrationError, ScopedConstructingHostFunction,
    ScopedDivergingHostFunction, ScopedHostFunction, StatelessHostProfile,
};
use ecow::EcoString;
use gleam_compiler_core::analyse::name::check_name_case;
use gleam_compiler_core::ast::SrcSpan;
use gleam_compiler_core::parse::lexer::string_to_keyword;
use gleam_compiler_core::type_::PRELUDE_MODULE_NAME;
use gleam_compiler_core::type_::error::Named;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub struct HostModule<Profile: HostProfile = StatelessHostProfile> {
    identity: HostModuleIdentity,
    functions: RegisteredFunctions<Profile>,
}

pub struct HostProviderModule<Profile: HostProfile = StatelessHostProfile> {
    identity: HostModuleIdentity,
    functions: RegisteredFunctions<Profile>,
    external_types: RegisteredExternalTypes,
}

pub struct HostProviderSet<Profile: HostProfile = StatelessHostProfile> {
    modules: Vec<HostModule<Profile>>,
    providers: Vec<HostProviderModule<Profile>>,
}

pub(super) struct HostModuleIdentity {
    pub(super) package: EcoString,
    pub(super) module: EcoString,
}

pub(crate) struct RegisteredHostModule {
    pub(crate) package: EcoString,
    pub(crate) module: EcoString,
    pub(crate) functions: Vec<RegisteredHostFunction>,
}

pub(crate) struct RegisteredHostProviderModule {
    pub(crate) package: EcoString,
    pub(crate) module: EcoString,
    pub(crate) functions: Vec<RegisteredHostFunction>,
    pub(crate) external_types: Vec<HostExternalTypeSchema>,
}

pub(crate) struct RegisteredHostFunction {
    schema: HostFunctionSchema,
    constructions: super::RegisteredHostConstructions,
    implementation: RegisteredHostImplementationId,
}

#[derive(Clone, Copy)]
pub(crate) struct RegisteredHostImplementationId(usize);

pub(crate) struct RegisteredHostImplementations<Profile: HostProfile> {
    functions: Vec<Arc<HostFunctionImplementation<Profile>>>,
}

struct RegisteredFunctions<Profile: HostProfile> {
    functions: Vec<HostFunctionDefinition<Profile>>,
}

pub(super) struct RegisteredExternalTypes {
    types: Vec<HostExternalTypeSchema>,
}

impl HostModule<StatelessHostProfile> {
    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
    ) -> Result<Self, HostRegistrationError> {
        Self::new_for_profile(package, module)
    }
}

impl<Profile: HostProfile> HostModule<Profile> {
    pub fn new_for_profile(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
    ) -> Result<Self, HostRegistrationError> {
        HostModuleIdentity::new(package.into(), module.into()).map(|identity| Self {
            identity,
            functions: RegisteredFunctions::new(),
        })
    }

    pub fn with_function<Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Function: HostFunction<Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                HostFunctionDefinition::new(name, function)
            })
            .map(|()| self)
    }

    pub fn with_fallible_function<Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Function: FallibleHostFunction<Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                HostFunctionDefinition::new_fallible(name, function)
            })
            .map(|()| self)
    }

    pub fn with_scoped_function<Provider, Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: ScopedHostFunction<Profile, Provider, Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                HostFunctionDefinition::new_scoped::<Provider, _, _, _>(name, function)
            })
            .map(|()| self)
    }

    pub fn with_scoped_diverging_function<Provider, Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: ScopedDivergingHostFunction<Profile, Provider, Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                HostFunctionDefinition::new_scoped_diverging::<Provider, _, _, _>(name, function)
            })
            .map(|()| self)
    }

    pub fn package(&self) -> &EcoString {
        &self.identity.package
    }

    pub fn module(&self) -> &EcoString {
        &self.identity.module
    }

    pub fn functions(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions.schemas()
    }
}

impl<Profile: HostProfile> HostProviderModule<Profile> {
    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
    ) -> Result<Self, HostRegistrationError> {
        HostModuleIdentity::new(package.into(), module.into()).map(|identity| Self {
            identity,
            functions: RegisteredFunctions::new(),
            external_types: RegisteredExternalTypes::new(),
        })
    }

    pub fn with_function<Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Function: HostFunction<Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                HostFunctionDefinition::new(name, function)
            })
            .map(|()| self)
    }

    pub fn with_fallible_function<Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Function: FallibleHostFunction<Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                HostFunctionDefinition::new_fallible(name, function)
            })
            .map(|()| self)
    }

    pub fn with_scoped_function<Provider, Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: ScopedHostFunction<Profile, Provider, Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                HostFunctionDefinition::new_scoped::<Provider, _, _, _>(name, function)
            })
            .map(|()| self)
    }

    /// Registers a scoped callback and the exact intermediate types it may construct.
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
        Constructions: crate::host::HostTypeSequence,
        Function:
            ScopedConstructingHostFunction<Profile, Provider, Arguments, Return, Constructions>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                HostFunctionDefinition::new_scoped_with_constructions::<
                    Provider,
                    Arguments,
                    Return,
                    Constructions,
                    Function,
                >(name, function)
            })
            .map(|()| self)
    }

    pub fn with_scoped_diverging_function<Provider, Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: ScopedDivergingHostFunction<Profile, Provider, Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                HostFunctionDefinition::new_scoped_diverging::<Provider, _, _, _>(name, function)
            })
            .map(|()| self)
    }

    pub fn with_external_type<Provider, Schema>(mut self) -> Result<Self, HostRegistrationError>
    where
        Schema: HostExternalSchema,
        Provider: HostExternalBinding<Profile, Schema>,
    {
        let schema = HostExternalTypeSchema::of::<Schema>();
        self.external_types
            .register(&self.identity.module, schema)
            .map(|()| self)
    }

    pub fn package(&self) -> &EcoString {
        &self.identity.package
    }

    pub fn module(&self) -> &EcoString {
        &self.identity.module
    }

    pub fn functions(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions.schemas()
    }

    pub fn external_types(&self) -> impl ExactSizeIterator<Item = &HostExternalTypeSchema> {
        self.external_types.schemas()
    }
}

impl<Profile: HostProfile> HostProviderSet<Profile> {
    pub fn from_providers(
        providers: impl IntoIterator<Item = HostProviderModule<Profile>>,
    ) -> Result<Self, HostRegistrationError> {
        Self::with_providers([], providers)
    }

    pub fn new(
        modules: impl IntoIterator<Item = HostModule<Profile>>,
    ) -> Result<Self, HostRegistrationError> {
        Self::with_providers(modules, Vec::<HostProviderModule<Profile>>::new())
    }

    pub fn with_providers(
        modules: impl IntoIterator<Item = HostModule<Profile>>,
        providers: impl IntoIterator<Item = HostProviderModule<Profile>>,
    ) -> Result<Self, HostRegistrationError> {
        let modules = modules.into_iter().collect::<Vec<_>>();
        let providers = providers.into_iter().collect::<Vec<_>>();
        let identities = modules
            .iter()
            .map(|module| (&module.identity.package, &module.identity.module))
            .chain(
                providers
                    .iter()
                    .map(|module| (&module.identity.package, &module.identity.module)),
            )
            .collect::<Vec<_>>();
        validate_module_identities(&identities).map(|()| Self { modules, providers })
    }

    pub fn modules(&self) -> impl ExactSizeIterator<Item = &HostModule<Profile>> {
        self.modules.iter()
    }

    pub fn providers(&self) -> impl ExactSizeIterator<Item = &HostProviderModule<Profile>> {
        self.providers.iter()
    }

    pub(crate) fn select_source_providers(
        mut self,
        source_modules: &BTreeSet<(EcoString, EcoString)>,
    ) -> Self {
        self.providers.retain(|provider| {
            source_modules.contains(&(
                provider.identity.package.clone(),
                provider.identity.module.clone(),
            ))
        });
        self
    }

    pub(crate) fn into_registered(
        self,
    ) -> (
        Vec<RegisteredHostModule>,
        Vec<RegisteredHostProviderModule>,
        RegisteredHostImplementations<Profile>,
    ) {
        let mut implementations = RegisteredHostImplementations::new();
        let mut modules = Vec::with_capacity(self.modules.len());
        for module in self.modules {
            modules.push(RegisteredHostModule {
                package: module.identity.package,
                module: module.identity.module,
                functions: module.functions.into_registered(&mut implementations),
            });
        }
        let mut providers = Vec::with_capacity(self.providers.len());
        for provider in self.providers {
            providers.push(RegisteredHostProviderModule {
                package: provider.identity.package,
                module: provider.identity.module,
                functions: provider.functions.into_registered(&mut implementations),
                external_types: provider.external_types.into_vec(),
            });
        }
        (modules, providers, implementations)
    }
}

impl HostModuleIdentity {
    pub(super) fn new(
        package: EcoString,
        module: EcoString,
    ) -> Result<Self, HostRegistrationError> {
        validate_module_name(&module)?;
        Ok(Self { package, module })
    }
}

fn validate_module_name(module: &EcoString) -> Result<(), HostRegistrationError> {
    let valid = module != PRELUDE_MODULE_NAME
        && !module.is_empty()
        && module.split('/').all(|segment| {
            !segment.is_empty()
                && string_to_keyword(segment).is_none()
                && check_name_case(
                    SrcSpan::new(0, 0),
                    &EcoString::from(segment),
                    Named::Function,
                )
                .is_ok()
        });
    if valid {
        Ok(())
    } else {
        Err(HostRegistrationError::InvalidModuleName {
            module: module.clone(),
        })
    }
}

pub(super) fn validate_module_identities(
    identities: &[(&EcoString, &EcoString)],
) -> Result<(), HostRegistrationError> {
    let mut modules = BTreeMap::new();
    for (package, module) in identities {
        if let Some(first_package) = modules.insert((*module).clone(), (*package).clone()) {
            return Err(HostRegistrationError::DuplicateModule {
                module: (*module).clone(),
                first_package,
                second_package: (*package).clone(),
            });
        }
    }
    Ok(())
}

impl<Profile: HostProfile> RegisteredFunctions<Profile> {
    fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    fn register(
        &mut self,
        module: &EcoString,
        name: EcoString,
        definition: impl FnOnce(
            EcoString,
        )
            -> Result<HostFunctionDefinition<Profile>, HostRegistrationError>,
    ) -> Result<(), HostRegistrationError> {
        validate_function_name(module, &name)?;
        if self
            .functions
            .iter()
            .any(|function| function.schema().name() == &name)
        {
            return Err(HostRegistrationError::DuplicateFunction {
                module: module.clone(),
                function: name,
            });
        }
        let definition = definition(name)?;
        self.functions.push(definition);
        Ok(())
    }

    fn schemas(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions.iter().map(HostFunctionDefinition::schema)
    }

    fn into_registered(
        self,
        implementations: &mut RegisteredHostImplementations<Profile>,
    ) -> Vec<RegisteredHostFunction> {
        let mut registered = Vec::with_capacity(self.functions.len());
        for function in self.functions {
            registered.push(implementations.register(function));
        }
        registered
    }
}

pub(super) fn validate_function_name(
    module: &EcoString,
    name: &EcoString,
) -> Result<(), HostRegistrationError> {
    if string_to_keyword(name).is_some()
        || check_name_case(SrcSpan::new(0, 0), name, Named::Function).is_err()
    {
        Err(HostRegistrationError::InvalidFunctionName {
            module: module.clone(),
            function: name.clone(),
        })
    } else {
        Ok(())
    }
}

impl RegisteredExternalTypes {
    pub(super) fn new() -> Self {
        Self { types: Vec::new() }
    }

    pub(super) fn register(
        &mut self,
        module: &EcoString,
        schema: HostExternalTypeSchema,
    ) -> Result<(), HostRegistrationError> {
        let name = schema.name().clone();
        if check_name_case(SrcSpan::new(0, 0), &name, Named::Type).is_err() {
            return Err(HostRegistrationError::InvalidExternalTypeName {
                module: module.clone(),
                type_: name,
            });
        }
        if self
            .types
            .iter()
            .any(|registered| registered.name() == &name)
        {
            return Err(HostRegistrationError::DuplicateExternalType {
                module: module.clone(),
                type_: name,
            });
        }
        self.types.push(schema);
        Ok(())
    }

    fn schemas(&self) -> impl ExactSizeIterator<Item = &HostExternalTypeSchema> {
        self.types.iter()
    }

    pub(super) fn into_vec(self) -> Vec<HostExternalTypeSchema> {
        self.types
    }
}

impl RegisteredHostModule {
    pub(crate) fn package(&self) -> &EcoString {
        &self.package
    }

    pub(crate) fn module(&self) -> &EcoString {
        &self.module
    }

    pub(crate) fn functions(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions.iter().map(RegisteredHostFunction::schema)
    }

    pub(crate) fn into_parts(self) -> (EcoString, EcoString, Vec<RegisteredHostFunction>) {
        (self.package, self.module, self.functions)
    }
}

impl RegisteredHostProviderModule {
    pub(crate) fn into_parts(
        self,
    ) -> (
        EcoString,
        EcoString,
        Vec<RegisteredHostFunction>,
        Vec<HostExternalTypeSchema>,
    ) {
        (
            self.package,
            self.module,
            self.functions,
            self.external_types,
        )
    }
}

impl RegisteredHostFunction {
    pub(crate) fn schema(&self) -> &HostFunctionSchema {
        &self.schema
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        HostFunctionSchema,
        super::RegisteredHostConstructions,
        RegisteredHostImplementationId,
    ) {
        (self.schema, self.constructions, self.implementation)
    }
}

impl<Profile: HostProfile> RegisteredHostImplementations<Profile> {
    fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    fn register(&mut self, definition: HostFunctionDefinition<Profile>) -> RegisteredHostFunction {
        let (schema, constructions, implementation) = definition.into_parts();
        let id = RegisteredHostImplementationId(self.functions.len());
        self.functions.push(Arc::new(implementation));
        RegisteredHostFunction {
            schema,
            constructions,
            implementation: id,
        }
    }

    pub(crate) fn implementation(
        &self,
        id: RegisteredHostImplementationId,
    ) -> Arc<HostFunctionImplementation<Profile>> {
        Arc::clone(&self.functions[id.0])
    }
}

#[cfg(test)]
mod tests {
    use super::{HostModule, HostProviderModule, HostProviderSet, RegisteredFunctions};
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder, with_execution_scope};
    use crate::frontend::compile_typed_host_program;
    use crate::host::function::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        ExternalTestProfile, ExternalTestRunState, ExternalTestStores, HostCall,
        HostCallCompletion, HostCallError, HostExternalBinding, HostExternalSchema,
        HostExternalStorage, HostExternalStore, HostFailure, HostFunctionDefinition, HostProvider,
        HostRegistrationError, HostScopedValue, HostStoredValue, StatelessHostProfile,
        expect_never_implementation, expect_value_implementation,
    };
    use crate::host::{
        HostComponentProfile, HostFutureStore, HostProfile, HostType, HostTypeParameter,
    };
    use crate::plan::ValueType;
    use crate::work_fixture::{WorkComponent, WorkSchema, WorkType};
    use crate::{ModuleSource, PackageSource};
    use ecow::EcoString;
    use futures_util::FutureExt;
    use num_bigint::BigInt;
    use std::cell::Cell;
    use std::collections::BTreeSet;
    use std::convert::Infallible;

    struct Counter;

    struct CounterSchema;

    struct InvalidCounterSchema;

    struct CounterStorage;

    struct InvalidCounterStorage;

    impl HostProvider<TestHostProfile> for Counter {
        type State = usize;

        fn project(state: &mut TestRunState) -> &mut Self::State {
            &mut state.counter
        }
    }

    impl HostProvider<ExternalTestProfile> for Counter {
        type State = ();

        fn project(state: &mut ExternalTestRunState) -> &mut Self::State {
            &mut state.provider
        }
    }

    impl HostExternalSchema for CounterSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Counter";
        const PARAMETER_COUNT: usize = 1;
    }

    impl HostExternalStorage<ExternalTestProfile, CounterSchema> for CounterStorage {
        type Payload = usize;

        fn store(stores: &ExternalTestStores) -> &HostExternalStore<Self::Payload> {
            &stores.indices
        }

        fn source_equal(
            _: &crate::host::HostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            left == right
        }

        fn source_hash(_: &crate::host::HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
            *value as u64
        }

        fn inspect(
            _: &crate::host::HostExternalInspection<'_>,
            value: &Self::Payload,
        ) -> EcoString {
            value.to_string().into()
        }
    }

    impl HostExternalBinding<ExternalTestProfile, CounterSchema> for Counter {
        type Storage = CounterStorage;
    }

    impl HostExternalSchema for InvalidCounterSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "counter";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalStorage<ExternalTestProfile, InvalidCounterSchema> for InvalidCounterStorage {
        type Payload = usize;

        fn store(stores: &ExternalTestStores) -> &HostExternalStore<Self::Payload> {
            &stores.indices
        }

        fn source_equal(
            _: &crate::host::HostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            left == right
        }

        fn source_hash(_: &crate::host::HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
            *value as u64
        }

        fn inspect(
            _: &crate::host::HostExternalInspection<'_>,
            value: &Self::Payload,
        ) -> EcoString {
            value.to_string().into()
        }
    }

    impl HostExternalBinding<ExternalTestProfile, InvalidCounterSchema> for Counter {
        type Storage = InvalidCounterStorage;
    }

    fn increment<'call>(
        mut call: HostCall<'call, TestHostProfile, Counter, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        *call.state() += 1;
        let value = BigInt::from(*call.state());
        Ok(call.return_value(value))
    }

    fn stop<'call>(
        mut call: HostCall<'call, TestHostProfile, Counter, BigInt>,
    ) -> Result<Infallible, HostCallError> {
        *call.state() += 1;
        Err(HostFailure::new("stopped").into())
    }

    #[test]
    fn provider_set_exposes_source_less_and_source_backed_schemas() {
        let module = HostModule::new("host_support", "host/math")
            .expect("module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("function should be valid");
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("checked", BigInt::default)
            .expect("fallible function should be valid");
        let hosts = HostProviderSet::with_providers([module], [provider])
            .expect("host module identities should be unique");

        let module = hosts.modules().next().expect("module should exist");
        let provider = hosts.providers().next().expect("provider should exist");

        assert_eq!(module.package(), "host_support");
        assert_eq!(module.module(), "host/math");
        assert_eq!(module.functions().next().expect("function").name(), "add");
        assert_eq!(provider.package(), "application");
        assert_eq!(provider.module(), "main");
        assert_eq!(
            provider
                .functions()
                .next()
                .expect("function")
                .type_()
                .return_(),
            &ValueType::Int,
        );
    }

    #[test]
    fn source_provider_selection_precedes_compact_implementation_registration() {
        let module = HostModule::new("host_support", "host/math")
            .expect("source-less module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("source-less function should be valid");
        let unused = HostProviderModule::<StatelessHostProfile>::new("application", "unused")
            .expect("unused provider should be valid")
            .with_function("value", BigInt::default)
            .expect("unused provider function should be valid");
        let first = HostProviderModule::<StatelessHostProfile>::new("application", "first")
            .expect("first provider should be valid")
            .with_function("value", BigInt::default)
            .expect("first provider function should be valid");
        let second = HostProviderModule::<StatelessHostProfile>::new("dependency", "second")
            .expect("second provider should be valid")
            .with_function("value", BigInt::default)
            .expect("second provider function should be valid");
        let selected = BTreeSet::from([
            (EcoString::from("application"), EcoString::from("first")),
            (EcoString::from("dependency"), EcoString::from("second")),
        ]);
        let hosts = HostProviderSet::with_providers([module], [unused, first, second])
            .expect("host module identities should be unique")
            .select_source_providers(&selected);
        let (modules, providers, implementations) = hosts.into_registered();

        assert_eq!(modules.len(), 1);
        assert_eq!(providers.len(), 2);
        assert_eq!(providers[0].package, "application");
        assert_eq!(providers[0].module, "first");
        assert_eq!(providers[0].functions[0].implementation.0, 1);
        assert_eq!(providers[1].package, "dependency");
        assert_eq!(providers[1].module, "second");
        assert_eq!(providers[1].functions[0].implementation.0, 2);
        assert_eq!(implementations.functions.len(), 3);
    }

    #[test]
    fn provider_module_exposes_registered_external_type_schemas() {
        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_external_type::<Counter, CounterSchema>()
            .expect("external type should be valid");
        let schema = provider
            .external_types()
            .next()
            .expect("external type should be registered");

        assert_eq!(schema.package(), "application");
        assert_eq!(schema.module(), "main");
        assert_eq!(schema.name(), "Counter");
        assert_eq!(schema.parameter_count(), 1);

        let hosts = HostProviderSet::with_providers(
            Vec::<HostModule<ExternalTestProfile>>::new(),
            [provider],
        )
        .expect("provider module should be unique");
        let (_, mut providers, _) = hosts.into_registered();
        let (_, _, _, external_types) = providers
            .pop()
            .expect("provider module should be registered")
            .into_parts();

        assert_eq!(
            external_types,
            [crate::host::HostExternalTypeSchema::of::<CounterSchema>()]
        );
    }

    #[test]
    fn rejects_invalid_and_duplicate_external_type_registrations() {
        assert_eq!(
            HostProviderModule::<ExternalTestProfile>::new("application", "main")
                .expect("provider module should be valid")
                .with_external_type::<Counter, InvalidCounterSchema>()
                .err(),
            Some(HostRegistrationError::InvalidExternalTypeName {
                module: "main".into(),
                type_: "counter".into(),
            }),
        );
        assert_eq!(
            HostProviderModule::<ExternalTestProfile>::new("application", "main")
                .expect("provider module should be valid")
                .with_external_type::<Counter, CounterSchema>()
                .expect("first external type should be valid")
                .with_external_type::<Counter, CounterSchema>()
                .err(),
            Some(HostRegistrationError::DuplicateExternalType {
                module: "main".into(),
                type_: "Counter".into(),
            }),
        );
    }

    #[test]
    fn external_storage_protocol_projects_payload_semantics() {
        let stores = ExternalTestStores::default();
        let mut state = ExternalTestRunState::default();
        let equal =
            |_: &crate::runtime::RetainedValueRef, _: &crate::runtime::RetainedValueRef| false;
        let source_hash = |_: &crate::runtime::RetainedValueRef| 0;
        let inspect = |_: &crate::runtime::RetainedValueRef| EcoString::new();
        let raw_equality = crate::host::RetainedValueEquality::new(&equal);
        let equality = crate::host::HostExternalEquality(&raw_equality);
        let raw_hashing = crate::host::RetainedValueHashing::new(&source_hash);
        let hashing = crate::host::HostExternalHashing(&raw_hashing);
        let raw_inspection = crate::host::RetainedValueInspection::new(&inspect);
        let inspection = crate::host::HostExternalInspection(&raw_inspection);
        let stored = HostStoredValue::<BigInt>::new(crate::runtime::StoredRuntimeValue::test_int(
            BigInt::from(7),
        ));

        assert!(std::ptr::eq(
            <Counter as HostProvider<ExternalTestProfile>>::project(&mut state),
            &state.provider,
        ));
        assert_eq!(inspection.inspect_stored_value(&stored), "");
        assert!(std::ptr::eq(
            <CounterStorage as HostExternalStorage<ExternalTestProfile, CounterSchema>>::store(
                &stores,
            ),
            &stores.indices,
        ));
        assert!(<CounterStorage as HostExternalStorage<
            ExternalTestProfile,
            CounterSchema,
        >>::source_equal(&equality, &7, &7),);
        assert_eq!(
            <CounterStorage as HostExternalStorage<ExternalTestProfile, CounterSchema>>::source_hash(
                &hashing, &7,
            ),
            7,
        );
        assert_eq!(
            <CounterStorage as HostExternalStorage<ExternalTestProfile, CounterSchema>>::inspect(
                &inspection,
                &7,
            ),
            "7",
        );
        assert!(std::ptr::eq(
            <InvalidCounterStorage as HostExternalStorage<
                ExternalTestProfile,
                InvalidCounterSchema,
            >>::store(&stores),
            &stores.indices,
        ));
        assert!(<InvalidCounterStorage as HostExternalStorage<
            ExternalTestProfile,
            InvalidCounterSchema,
        >>::source_equal(&equality, &8, &8),);
        assert_eq!(
            <InvalidCounterStorage as HostExternalStorage<
                ExternalTestProfile,
                InvalidCounterSchema,
            >>::source_hash(&hashing, &8,),
            8,
        );
        assert_eq!(
            <InvalidCounterStorage as HostExternalStorage<
                ExternalTestProfile,
                InvalidCounterSchema,
            >>::inspect(&inspection, &8,),
            "8",
        );
    }

    #[test]
    fn scoped_registration_projects_provider_state() {
        let provider = HostProviderModule::<TestHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_scoped_function::<Counter, _, _, _>("increment", increment)
            .expect("scoped function should be valid");

        assert_eq!(provider.functions().len(), 1);
        let hosts =
            HostProviderSet::with_providers(Vec::<HostModule<TestHostProfile>>::new(), [provider])
                .expect("provider module should be unique");
        let (_, mut providers, implementations) = hosts.into_registered();
        let (_, _, mut definitions, _) = providers
            .pop()
            .expect("provider module should be registered")
            .into_parts();
        let (_, _, implementation) = definitions
            .pop()
            .expect("scoped function should be registered")
            .into_parts();
        let registered_implementation = implementations.implementation(implementation);
        let implementation = expect_value_implementation(registered_implementation.as_ref());
        let mut state = TestRunState {
            counter: 41,
            unrelated: true,
        };
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        let token = implementation
            .call(&mut runtime)
            .expect("scoped function should succeed");
        assert_eq!(token.family, crate::host::HostValueFamily::Int);
        drop(runtime);
        assert_eq!(state.counter, 42);
        assert!(state.unrelated);
    }

    #[test]
    fn scoped_diverging_provider_registration_preserves_the_source_return_type() {
        let provider = HostProviderModule::<TestHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_scoped_diverging_function::<Counter, (), BigInt, _>("stop", stop)
            .expect("scoped diverging function should be valid");

        let schema = provider
            .functions()
            .next()
            .expect("scoped diverging function should have a schema");
        assert_eq!(schema.name(), "stop");
        assert_eq!(schema.type_().argument_types(), []);
        assert_eq!(schema.type_().return_(), &ValueType::Int);

        let hosts =
            HostProviderSet::with_providers(Vec::<HostModule<TestHostProfile>>::new(), [provider])
                .expect("provider module should be unique");
        let (_, mut providers, implementations) = hosts.into_registered();
        let (_, _, mut definitions, _) = providers
            .pop()
            .expect("provider module should be registered")
            .into_parts();
        let (_, _, implementation) = definitions
            .pop()
            .expect("scoped diverging function should be registered")
            .into_parts();
        let registered = implementations.implementation(implementation);
        let implementation = expect_never_implementation(registered.as_ref());
        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        assert_eq!(
            implementation
                .call(&mut runtime)
                .expect_err("scoped diverging function should fail")
                .to_string(),
            "stopped",
        );
        drop(runtime);
        assert_eq!(state.counter, 1);
    }

    #[test]
    fn source_less_profile_registration_invokes_fallible_and_scoped_callbacks() {
        let module = HostModule::<TestHostProfile>::new_for_profile("host_support", "host/state")
            .expect("module should be valid")
            .with_fallible_function("checked", || {
                Result::<BigInt, HostFailure>::Ok(BigInt::from(7))
            })
            .expect("fallible function should be valid")
            .with_scoped_function::<Counter, _, _, _>("increment", increment)
            .expect("scoped function should be valid");
        let hosts = HostProviderSet::new([module]).expect("host module should be unique");
        let (mut modules, _, implementations) = hosts.into_registered();
        let (_, _, definitions) = modules
            .pop()
            .expect("host module should be registered")
            .into_parts();
        let mut definitions = definitions.into_iter();
        let (_, _, checked) = definitions
            .next()
            .expect("fallible function should be registered")
            .into_parts();
        let checked_implementation = implementations.implementation(checked);
        let checked = expect_value_implementation(checked_implementation.as_ref());
        let (_, _, increment) = definitions
            .next()
            .expect("scoped function should be registered")
            .into_parts();
        let increment_implementation = implementations.implementation(increment);
        let increment = expect_value_implementation(increment_implementation.as_ref());
        let mut state = TestRunState {
            counter: 9,
            unrelated: true,
        };

        let mut runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
        assert_eq!(
            checked
                .call(&mut runtime)
                .expect("fallible function should succeed")
                .family,
            crate::host::HostValueFamily::Int,
        );
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::Int(BigInt::from(7))),
        );
        drop(runtime);
        let mut runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
        let token = increment
            .call(&mut runtime)
            .expect("scoped function should succeed");
        assert_eq!(token.family, crate::host::HostValueFamily::Int);
        drop(runtime);
        assert_eq!(state.counter, 10);
        assert!(state.unrelated);
    }

    #[test]
    fn rejects_invalid_module_and_function_names() {
        assert_eq!(
            HostModule::<TestHostProfile>::new_for_profile("host_support", "").err(),
            Some(HostRegistrationError::InvalidModuleName { module: "".into() }),
        );
        assert_eq!(
            HostProviderModule::<TestHostProfile>::new("host_support", "gleam").err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "gleam".into(),
            }),
        );
        assert_eq!(
            HostModule::<TestHostProfile>::new_for_profile("host_support", "host/math")
                .expect("module should be valid")
                .with_function("Add", <BigInt as std::ops::Add>::add)
                .err(),
            Some(HostRegistrationError::InvalidFunctionName {
                module: "host/math".into(),
                function: "Add".into(),
            }),
        );
        assert_eq!(
            HostModule::<StatelessHostProfile>::new("host_support", "").err(),
            Some(HostRegistrationError::InvalidModuleName { module: "".into() }),
        );
        assert_eq!(
            HostProviderModule::<StatelessHostProfile>::new("host_support", "gleam").err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "gleam".into(),
            }),
        );
        assert_eq!(
            HostModule::<StatelessHostProfile>::new("host_support", "host/math")
                .expect("module should be valid")
                .with_function("Add", <BigInt as std::ops::Add>::add)
                .err(),
            Some(HostRegistrationError::InvalidFunctionName {
                module: "host/math".into(),
                function: "Add".into(),
            }),
        );
    }

    #[test]
    fn rejects_duplicate_functions_and_module_identities() {
        assert_eq!(
            HostModule::<TestHostProfile>::new_for_profile("host_support", "host/math")
                .expect("module should be valid")
                .with_function("add", <BigInt as std::ops::Add>::add)
                .expect("function should be valid")
                .with_function("add", <BigInt as std::ops::Add>::add)
                .err(),
            Some(HostRegistrationError::DuplicateFunction {
                module: "host/math".into(),
                function: "add".into(),
            }),
        );
        let module = HostModule::<StatelessHostProfile>::new("host_support", "host/math")
            .expect("module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("function should be valid");
        assert_eq!(
            module
                .with_function("add", <BigInt as std::ops::Add>::add)
                .err(),
            Some(HostRegistrationError::DuplicateFunction {
                module: "host/math".into(),
                function: "add".into(),
            }),
        );

        let module = HostModule::new("first", "host/math").expect("module should be valid");
        let provider =
            HostProviderModule::new("second", "host/math").expect("provider should be valid");
        assert_eq!(
            HostProviderSet::<StatelessHostProfile>::with_providers([module], [provider]).err(),
            Some(HostRegistrationError::DuplicateModule {
                module: "host/math".into(),
                first_package: "first".into(),
                second_package: "second".into(),
            }),
        );
    }

    #[test]
    fn function_name_and_duplicate_validation_precede_definition_assembly() {
        let module = EcoString::from("host/generic");
        let mut functions = RegisteredFunctions::<TestHostProfile>::new();
        let assembly_count = Cell::new(0);
        let assemble = |name| {
            assembly_count.set(assembly_count.get() + 1);
            if name == "sparse" {
                Err(HostRegistrationError::NonContiguousTypeParameters {
                    function: name,
                    parameters: vec![2].into_boxed_slice(),
                })
            } else {
                HostFunctionDefinition::new(name, || true)
            }
        };

        let invalid_name = functions.register(&module, "case".into(), assemble).err();
        assert_eq!(assembly_count.get(), 0);

        functions
            .register(&module, "identity".into(), assemble)
            .expect("the first valid definition should be assembled");
        assert_eq!(assembly_count.get(), 1);

        let duplicate = functions
            .register(&module, "identity".into(), assemble)
            .err();
        assert_eq!(assembly_count.get(), 1);

        let definition_error = functions.register(&module, "sparse".into(), assemble).err();
        assert_eq!(assembly_count.get(), 2);

        assert_eq!(
            invalid_name,
            Some(HostRegistrationError::InvalidFunctionName {
                module: "host/generic".into(),
                function: "case".into(),
            }),
        );
        assert_eq!(
            duplicate,
            Some(HostRegistrationError::DuplicateFunction {
                module: "host/generic".into(),
                function: "identity".into(),
            }),
        );
        assert_eq!(
            definition_error,
            Some(HostRegistrationError::NonContiguousTypeParameters {
                function: "sparse".into(),
                parameters: vec![2].into_boxed_slice(),
            }),
        );
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

    fn identity<'call, Value: HostType>(
        mut call: HostCall<'call, Profile, WorkComponent, Value>,
        value: Value::Value<'call>,
    ) -> Result<HostCallCompletion<'call, Value>, HostCallError> {
        assert_eq!(call.state(), &());
        Ok(call.return_value(value))
    }

    #[test]
    fn module_identity_and_schemas_describe_the_functions_that_execute() {
        let mut providers = WorkComponent::providers::<Profile>().expect("Future module");
        assert_eq!(providers[0].package(), "work_fixture");
        assert_eq!(providers[0].module(), "fixture/work");
        assert_eq!(providers[0].functions().len(), 4);
        assert_eq!(
            providers[0]
                .functions()
                .map(|schema| schema.name().as_str())
                .collect::<Vec<_>>(),
            ["ready", "map", "flatten", "all"]
        );
        providers.push(HostProviderModule::new("application", "library")
            .expect("module")
            .with_scoped_function::<WorkComponent, (HostTypeParameter<0>,), HostTypeParameter<0>, _>(
                "identity", identity::<HostTypeParameter<0>>,
            ).expect("generic identity"));
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
            HostProviderSet::from_providers(providers).expect("selected modules"),
        )
        .expect("source types");
        let (bindings, answer) = HostedModuleBuilder::new(program)
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
            call: HostCall<'call, Profile, WorkComponent, BigInt>,
            _: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            Ok(call.return_value(BigInt::from(42)))
        }
        fn stop(
            _: HostCall<'_, Profile, WorkComponent, BigInt>,
        ) -> Result<std::convert::Infallible, HostCallError> {
            Err(crate::HostFailure::new("native stopped").into())
        }
        for name in ["Bad", "construct"] {
            let result = HostProviderModule::new("application", "library")
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
            let result = HostProviderModule::new("application", "library")
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
        let provider = HostProviderModule::new("application", "library")
        .expect("module")
        .with_scoped_function_and_constructions::<WorkComponent, (), BigInt, HostTypeListEnd, _>(
            "construct",
            construct,
        )
        .expect("construct")
        .with_scoped_diverging_function::<WorkComponent, (), BigInt, _>("stop", stop)
        .expect("stop");
        let program = compile_typed_host_program(
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
            HostProviderSet::from_providers([provider]).expect("providers"),
        )
        .expect("ordinary source");
        let (bindings, run) = HostedModuleBuilder::new(program)
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
    fn function_validation_precedes_scoped_definition_assembly() {
        use crate::host::HostFunctionDefinition;
        use std::cell::Cell;

        let mut module =
            HostProviderModule::<Profile>::new("application", "library").expect("module");
        let assemblies = Cell::new(0);
        let assemble = |name: ecow::EcoString| {
            assemblies.set(assemblies.get() + 1);
            if name == "sparse" {
                HostFunctionDefinition::new_scoped::<
                    WorkComponent,
                    (HostTypeParameter<1>,),
                    HostTypeParameter<1>,
                    _,
                >(name, identity::<HostTypeParameter<1>>)
            } else {
                HostFunctionDefinition::new_scoped::<
                    WorkComponent,
                    (HostTypeParameter<0>,),
                    HostTypeParameter<0>,
                    _,
                >(name, identity::<HostTypeParameter<0>>)
            }
        };
        assert_eq!(
            module
                .functions
                .register(&module.identity.module, "Bad".into(), assemble)
                .err(),
            Some(HostRegistrationError::InvalidFunctionName {
                module: "library".into(),
                function: "Bad".into(),
            })
        );
        assert_eq!(assemblies.get(), 0);
        module
            .functions
            .register(&module.identity.module, "identity".into(), assemble)
            .expect("valid definition");
        assert_eq!(assemblies.get(), 1);
        assert_eq!(
            module
                .functions
                .register(&module.identity.module, "identity".into(), assemble)
                .err(),
            Some(HostRegistrationError::DuplicateFunction {
                module: "library".into(),
                function: "identity".into(),
            })
        );
        assert_eq!(assemblies.get(), 1);
        assert_eq!(
            module
                .functions
                .register(&module.identity.module, "sparse".into(), assemble)
                .err(),
            Some(HostRegistrationError::NonContiguousTypeParameters {
                function: "sparse".into(),
                parameters: Box::new([1]),
            })
        );
        assert_eq!(assemblies.get(), 2);
        assert_eq!(module.functions().len(), 1);
    }

    #[test]
    fn external_return_callbacks_preserve_nested_source_failures() {
        use crate::host::{HostCallable, HostFunctionType, HostTypeListEnd};
        use crate::work_fixture::WorkHostType;

        fn bridge<'call>(
            mut call: HostCall<'call, Profile, WorkComponent, WorkHostType<BigInt>>,
            callback: HostCallable<'call, HostTypeListEnd, WorkHostType<BigInt>>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
            let value = call.invoke(callback, ())?;
            Ok(call.return_value(value))
        }
        fn reject<'call>(
            _: HostCall<'call, Profile, WorkComponent, WorkHostType<BigInt>>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
            Err(crate::HostFailure::new("native rejected").into())
        }
        let mut providers = WorkComponent::providers::<Profile>().expect("Future module");
        providers.push(HostProviderModule::new("application", "library").expect("native module")
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
            HostProviderSet::from_providers(providers).expect("providers"),
        )
        .expect("ordinary callback source");
        let (bindings, run) = HostedModuleBuilder::new(program)
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
            HostProviderModule::<Profile>::new("application", "Bad").err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "Bad".into()
            })
        );
        for name in ["Bad", "identity"] {
            let error = HostProviderModule::new("application", "library")
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
        let error = HostProviderModule::new("application", "library")
        .expect("module")
        .with_scoped_function::<WorkComponent, (HostTypeParameter<1>,), HostTypeParameter<1>, _>(
            "identity",
            identity::<HostTypeParameter<1>>,
        )
        .err();
        assert_eq!(
            error,
            Some(HostRegistrationError::NonContiguousTypeParameters {
                function: "identity".into(),
                parameters: Box::new([1]),
            })
        );
        let error = HostProviderModule::<Profile>::new("work_fixture", "fixture/work")
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
        let error = HostProviderSet::from_providers([
            HostProviderModule::<Profile>::new("first", "library").expect("first"),
            HostProviderModule::new("second", "library").expect("second"),
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
