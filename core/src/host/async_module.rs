use super::async_function::{AsyncHostFunctionDefinition, AsyncHostFunctionImplementation};
use super::function::{OwnedHostFunctionDefinition, OwnedHostFunctionImplementation};
use super::module::{
    HostModuleIdentity, RegisteredExternalTypes, RegisteredHostFunction,
    RegisteredHostImplementationId, RegisteredHostModule, RegisteredHostProviderModule,
    validate_function_name, validate_module_identities,
};
use super::{
    AsyncHostFunction, FallibleAsyncHostFunction, FallibleHostFunction,
    FallibleScopedAsyncHostFunction, HostFunction, HostFunctionSchema, HostProfile, HostProvider,
    HostRegistrationError, ScopedAsyncHostFunction, StatelessHostProfile,
};
use ecow::EcoString;
use std::collections::BTreeSet;
use std::future::Future;
use std::sync::Arc;

use super::{AsyncHostExternalBinding, HostExternalSchema, HostExternalTypeSchema};

/// A source-less host module whose Rust functions may suspend.
///
/// Async host modules are accepted only by the resumable embedding path. They
/// are statically separate from [`crate::HostModule`] and cannot be sealed into
/// an immediate hosted execution.
///
/// ```compile_fail
/// use geam_core::{AsyncHostProviderSet, HostProviderSet};
/// fn immediate(hosts: AsyncHostProviderSet) -> HostProviderSet {
///     hosts
/// }
/// ```
pub struct AsyncHostModule<Profile: HostProfile = StatelessHostProfile> {
    identity: HostModuleIdentity,
    functions: RegisteredAsyncFunctions<Profile>,
}

/// Async implementations attached to an ordinary Gleam source module.
pub struct AsyncHostProviderModule<Profile: HostProfile = StatelessHostProfile> {
    identity: HostModuleIdentity,
    functions: RegisteredAsyncFunctions<Profile>,
    external_types: RegisteredExternalTypes,
}

/// A validated set of async host modules for one resumable embedding program.
pub struct AsyncHostProviderSet<Profile: HostProfile = StatelessHostProfile> {
    modules: Vec<AsyncHostModule<Profile>>,
    providers: Vec<AsyncHostProviderModule<Profile>>,
}

struct RegisteredAsyncFunctions<Profile: HostProfile> {
    functions: Vec<ResumableHostFunctionDefinition<Profile>>,
}

pub(crate) struct RegisteredAsyncHostImplementations<Profile: HostProfile> {
    functions: Vec<Arc<ResumableHostFunctionImplementation<Profile>>>,
}

enum ResumableHostFunctionDefinition<Profile: HostProfile> {
    Immediate(OwnedHostFunctionDefinition<Profile>),
    Async(AsyncHostFunctionDefinition<Profile>),
}

pub(crate) enum ResumableHostFunctionImplementation<Profile: HostProfile> {
    Immediate(OwnedHostFunctionImplementation<Profile>),
    Async(AsyncHostFunctionImplementation<Profile>),
}

impl AsyncHostModule<StatelessHostProfile> {
    /// Creates an empty stateless host module for the given package and module.
    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
    ) -> Result<Self, HostRegistrationError> {
        Self::new_for_profile(package, module)
    }
}

impl<Profile: HostProfile> AsyncHostModule<Profile> {
    /// Creates an empty host module using the selected host profile.
    pub fn new_for_profile(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
    ) -> Result<Self, HostRegistrationError> {
        HostModuleIdentity::new(package.into(), module.into()).map(|identity| Self {
            identity,
            functions: RegisteredAsyncFunctions::new(),
        })
    }

    /// Registers an infallible function that completes immediately.
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
                OwnedHostFunctionDefinition::new(name, function)
                    .map(ResumableHostFunctionDefinition::Immediate)
            })
            .map(|()| self)
    }

    /// Registers a fallible function that completes immediately.
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
                OwnedHostFunctionDefinition::new_fallible(name, function)
                    .map(ResumableHostFunctionDefinition::Immediate)
            })
            .map(|()| self)
    }

    /// Registers an infallible function whose owned Future may suspend.
    pub fn with_async_function<Arguments, Return, Function, HostFuture>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Function: AsyncHostFunction<Arguments, Return, HostFuture>,
        HostFuture: Future<Output = Return> + Send + 'static,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                AsyncHostFunctionDefinition::new(name, function)
                    .map(ResumableHostFunctionDefinition::Async)
            })
            .map(|()| self)
    }

    /// Registers a fallible function whose owned Future may suspend.
    pub fn with_fallible_async_function<Arguments, Return, Function, HostFuture>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Function: FallibleAsyncHostFunction<Arguments, Return, HostFuture>,
        HostFuture: Future<Output = Result<Return, super::HostFailure>> + Send + 'static,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                AsyncHostFunctionDefinition::new_fallible(name, function)
                    .map(ResumableHostFunctionDefinition::Async)
            })
            .map(|()| self)
    }

    /// Registers an async function with bounded state and callback access.
    pub fn with_scoped_async_function<Provider, Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: ScopedAsyncHostFunction<Profile, Provider, Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                AsyncHostFunctionDefinition::new_scoped::<Provider, _, _, _>(name, function)
                    .map(ResumableHostFunctionDefinition::Async)
            })
            .map(|()| self)
    }

    /// Registers a fallible async function with bounded state and callback access.
    pub fn with_fallible_scoped_async_function<Provider, Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: FallibleScopedAsyncHostFunction<Profile, Provider, Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                AsyncHostFunctionDefinition::new_fallible_scoped::<Provider, _, _, _>(
                    name, function,
                )
                .map(ResumableHostFunctionDefinition::Async)
            })
            .map(|()| self)
    }

    /// Returns the Gleam package implemented by this host module.
    pub fn package(&self) -> &EcoString {
        &self.identity.package
    }

    /// Returns the Gleam module implemented by this host module.
    pub fn module(&self) -> &EcoString {
        &self.identity.module
    }

    /// Iterates over the registered function schemas in registration order.
    pub fn functions(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions.schemas()
    }
}

impl<Profile: HostProfile> AsyncHostProviderModule<Profile> {
    /// Registers a source-declared external type with its transferable storage.
    pub fn with_external_type<Provider, Schema>(mut self) -> Result<Self, HostRegistrationError>
    where
        Schema: HostExternalSchema,
        Provider: AsyncHostExternalBinding<Profile, Schema>,
    {
        self.external_types
            .register(
                &self.identity.module,
                HostExternalTypeSchema::of::<Schema>(),
            )
            .map(|()| self)
    }

    /// Creates an empty set of Rust implementations for one Gleam source module.
    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
    ) -> Result<Self, HostRegistrationError> {
        HostModuleIdentity::new(package.into(), module.into()).map(|identity| Self {
            identity,
            functions: RegisteredAsyncFunctions::new(),
            external_types: RegisteredExternalTypes::new(),
        })
    }

    #[cfg(test)]
    pub(crate) fn with_external_type_for_test<Schema>(mut self) -> Self
    where
        Schema: HostExternalSchema,
    {
        self.external_types
            .push_valid_for_test(HostExternalTypeSchema::of::<Schema>());
        self
    }

    /// Registers an infallible source-provider function that completes immediately.
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
                OwnedHostFunctionDefinition::new(name, function)
                    .map(ResumableHostFunctionDefinition::Immediate)
            })
            .map(|()| self)
    }

    /// Registers a fallible source-provider function that completes immediately.
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
                OwnedHostFunctionDefinition::new_fallible(name, function)
                    .map(ResumableHostFunctionDefinition::Immediate)
            })
            .map(|()| self)
    }

    /// Registers an infallible source-provider function whose Future may suspend.
    pub fn with_async_function<Arguments, Return, Function, HostFuture>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Function: AsyncHostFunction<Arguments, Return, HostFuture>,
        HostFuture: Future<Output = Return> + Send + 'static,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                AsyncHostFunctionDefinition::new(name, function)
                    .map(ResumableHostFunctionDefinition::Async)
            })
            .map(|()| self)
    }

    /// Registers a fallible source-provider function whose Future may suspend.
    pub fn with_fallible_async_function<Arguments, Return, Function, HostFuture>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Function: FallibleAsyncHostFunction<Arguments, Return, HostFuture>,
        HostFuture: Future<Output = Result<Return, super::HostFailure>> + Send + 'static,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                AsyncHostFunctionDefinition::new_fallible(name, function)
                    .map(ResumableHostFunctionDefinition::Async)
            })
            .map(|()| self)
    }

    /// Registers a source-provider async function with bounded state and callback access.
    pub fn with_scoped_async_function<Provider, Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: ScopedAsyncHostFunction<Profile, Provider, Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                AsyncHostFunctionDefinition::new_scoped::<Provider, _, _, _>(name, function)
                    .map(ResumableHostFunctionDefinition::Async)
            })
            .map(|()| self)
    }

    /// Registers a fallible source-provider async function with bounded state and callback access.
    pub fn with_fallible_scoped_async_function<Provider, Arguments, Return, Function>(
        mut self,
        name: impl Into<EcoString>,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: FallibleScopedAsyncHostFunction<Profile, Provider, Arguments, Return>,
    {
        self.functions
            .register(&self.identity.module, name.into(), |name| {
                AsyncHostFunctionDefinition::new_fallible_scoped::<Provider, _, _, _>(
                    name, function,
                )
                .map(ResumableHostFunctionDefinition::Async)
            })
            .map(|()| self)
    }

    /// Returns the Gleam package that owns the source module.
    pub fn package(&self) -> &EcoString {
        &self.identity.package
    }

    /// Returns the Gleam source module receiving these implementations.
    pub fn module(&self) -> &EcoString {
        &self.identity.module
    }

    /// Iterates over the registered function schemas in registration order.
    pub fn functions(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions.schemas()
    }
}

impl<Profile: HostProfile> AsyncHostProviderSet<Profile> {
    /// Validates a set containing only source-less async host modules.
    pub fn new(
        modules: impl IntoIterator<Item = AsyncHostModule<Profile>>,
    ) -> Result<Self, HostRegistrationError> {
        Self::with_providers(modules, Vec::<AsyncHostProviderModule<Profile>>::new())
    }

    /// Validates source-less host modules and source-provider modules together.
    pub fn with_providers(
        modules: impl IntoIterator<Item = AsyncHostModule<Profile>>,
        providers: impl IntoIterator<Item = AsyncHostProviderModule<Profile>>,
    ) -> Result<Self, HostRegistrationError> {
        let modules = modules.into_iter().collect::<Vec<_>>();
        let providers = providers.into_iter().collect::<Vec<_>>();
        let identities = modules
            .iter()
            .map(|module| (&module.identity.package, &module.identity.module))
            .chain(
                providers
                    .iter()
                    .map(|provider| (&provider.identity.package, &provider.identity.module)),
            )
            .collect::<Vec<_>>();
        validate_module_identities(&identities).map(|()| Self { modules, providers })
    }

    /// Iterates over the source-less host modules.
    pub fn modules(&self) -> impl ExactSizeIterator<Item = &AsyncHostModule<Profile>> {
        self.modules.iter()
    }

    /// Iterates over the source-provider modules.
    pub fn providers(&self) -> impl ExactSizeIterator<Item = &AsyncHostProviderModule<Profile>> {
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
        RegisteredAsyncHostImplementations<Profile>,
    ) {
        let mut implementations = RegisteredAsyncHostImplementations::new();
        let modules = self
            .modules
            .into_iter()
            .map(|module| RegisteredHostModule {
                package: module.identity.package,
                module: module.identity.module,
                functions: module.functions.into_registered(&mut implementations),
            })
            .collect();
        let providers = self
            .providers
            .into_iter()
            .map(|provider| RegisteredHostProviderModule {
                package: provider.identity.package,
                module: provider.identity.module,
                functions: provider.functions.into_registered(&mut implementations),
                external_types: provider.external_types.into_vec(),
            })
            .collect();
        (modules, providers, implementations)
    }
}

impl<Profile: HostProfile> RegisteredAsyncFunctions<Profile> {
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
        ) -> Result<
            ResumableHostFunctionDefinition<Profile>,
            HostRegistrationError,
        >,
    ) -> Result<(), HostRegistrationError> {
        validate_function_name(module, &name)
            .and_then(|()| {
                if self
                    .functions
                    .iter()
                    .any(|function| function.schema().name() == &name)
                {
                    Err(HostRegistrationError::DuplicateFunction {
                        module: module.clone(),
                        function: name.clone(),
                    })
                } else {
                    Ok(name)
                }
            })
            .and_then(definition)
            .map(|function| self.functions.push(function))
    }

    fn schemas(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions
            .iter()
            .map(ResumableHostFunctionDefinition::schema)
    }

    fn into_registered(
        self,
        implementations: &mut RegisteredAsyncHostImplementations<Profile>,
    ) -> Vec<RegisteredHostFunction> {
        self.functions
            .into_iter()
            .map(|function| implementations.register(function))
            .collect()
    }
}

impl<Profile: HostProfile> RegisteredAsyncHostImplementations<Profile> {
    fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    fn register(
        &mut self,
        definition: ResumableHostFunctionDefinition<Profile>,
    ) -> RegisteredHostFunction {
        let (schema, constructions, implementation) = definition.into_parts();
        let id = RegisteredHostImplementationId::new(self.functions.len());
        self.functions.push(Arc::new(implementation));
        RegisteredHostFunction::new(schema, constructions, id)
    }

    pub(crate) fn implementation(
        &self,
        id: RegisteredHostImplementationId,
    ) -> Arc<ResumableHostFunctionImplementation<Profile>> {
        Arc::clone(&self.functions[id.index()])
    }
}

impl<Profile: HostProfile> ResumableHostFunctionDefinition<Profile> {
    fn schema(&self) -> &HostFunctionSchema {
        match self {
            Self::Immediate(definition) => definition.schema(),
            Self::Async(definition) => definition.schema(),
        }
    }

    fn into_parts(
        self,
    ) -> (
        HostFunctionSchema,
        super::RegisteredHostConstructions,
        ResumableHostFunctionImplementation<Profile>,
    ) {
        match self {
            Self::Immediate(definition) => {
                let (schema, constructions, implementation) = definition.into_parts();
                (
                    schema,
                    constructions,
                    ResumableHostFunctionImplementation::Immediate(implementation),
                )
            }
            Self::Async(definition) => {
                let (schema, constructions, implementation) = definition.into_parts();
                (
                    schema,
                    constructions,
                    ResumableHostFunctionImplementation::Async(implementation),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AsyncHostModule, AsyncHostProviderModule, AsyncHostProviderSet, RegisteredAsyncFunctions,
    };
    use crate::{HostFailure, HostRegistrationError};
    use num_bigint::BigInt;
    use std::collections::BTreeSet;

    #[test]
    fn failed_definition_registration_does_not_mutate_the_function_set() {
        let mut functions = RegisteredAsyncFunctions::<crate::StatelessHostProfile>::new();
        let error = functions
            .register(&"host/math".into(), "identity".into(), |_| {
                Err(HostRegistrationError::NonContiguousTypeParameters {
                    function: "identity".into(),
                    parameters: vec![2].into_boxed_slice(),
                })
            })
            .expect_err("definition failure should be preserved");

        assert_eq!(
            error,
            HostRegistrationError::NonContiguousTypeParameters {
                function: "identity".into(),
                parameters: vec![2].into_boxed_slice(),
            },
        );
        assert_eq!(functions.schemas().len(), 0);
    }

    #[test]
    fn async_modules_validate_names_and_keep_implementations_separate() {
        let module = AsyncHostModule::new("host_support", "host/math")
            .expect("module should be valid")
            .with_async_function("identity", std::future::ready::<BigInt>)
            .expect("function should be valid");
        let hosts = AsyncHostProviderSet::new([module]).expect("set should be valid");
        assert_eq!(hosts.modules().len(), 1);
        assert!(hosts.providers().next().is_none());
        let module = hosts.modules().next().expect("source-less module");
        assert_eq!(module.package(), "host_support");
        assert_eq!(module.module(), "host/math");
        assert_eq!(
            module.functions().next().expect("function").name(),
            "identity"
        );
        let (modules, providers, implementations) = hosts.into_registered();

        assert_eq!(modules.len(), 1);
        assert!(providers.is_empty());
        assert_eq!(modules[0].functions().count(), 1);
        let (_, _, functions) = modules.into_iter().next().unwrap().into_parts();
        let (_, _, id) = functions.into_iter().next().unwrap().into_parts();
        let _implementation = implementations.implementation(id);
    }

    #[test]
    fn async_provider_modules_remain_distinct_from_source_less_modules() {
        let provider = AsyncHostProviderModule::new("application", "library")
            .expect("provider module should be valid")
            .with_fallible_function("checked", Result::<BigInt, HostFailure>::Ok)
            .expect("immediate provider function should be valid")
            .with_async_function("identity", std::future::ready::<BigInt>)
            .expect("provider function should be valid");
        assert_eq!(provider.package(), "application");
        assert_eq!(provider.module(), "library");
        assert_eq!(provider.functions().len(), 2);
        let hosts = AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule>::new(), [provider])
            .expect("provider set should be valid");
        let (modules, providers, implementations) = hosts.into_registered();

        assert!(modules.is_empty());
        assert_eq!(providers.len(), 1);
        let (_, _, functions, external_types) = providers
            .into_iter()
            .next()
            .expect("registered provider")
            .into_parts();
        assert!(external_types.is_empty());
        for function in functions {
            let (_, _, id) = function.into_parts();
            let _implementation = implementations.implementation(id);
        }
    }

    #[test]
    fn selects_only_source_providers_in_the_project_import_closure() {
        let unused = AsyncHostProviderModule::new("application", "unused")
            .expect("unused provider module should be valid");
        let selected = AsyncHostProviderModule::new("application", "selected")
            .expect("selected provider module should be valid");
        let hosts =
            AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule>::new(), [unused, selected])
                .expect("provider modules should be unique")
                .select_source_providers(&BTreeSet::from([(
                    "application".into(),
                    "selected".into(),
                )]));

        assert_eq!(hosts.providers().count(), 1);
        assert_eq!(
            hosts
                .providers()
                .next()
                .expect("selected source provider")
                .module(),
            "selected"
        );
    }

    #[test]
    fn rejects_invalid_and_duplicate_async_host_names() {
        assert_eq!(
            AsyncHostModule::new("host_support", "").err(),
            Some(HostRegistrationError::InvalidModuleName { module: "".into() }),
        );
        assert_eq!(
            AsyncHostProviderModule::<crate::StatelessHostProfile>::new("application", "gleam")
                .err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "gleam".into(),
            }),
        );
        assert_eq!(
            AsyncHostModule::new("host_support", "host/math")
                .expect("module should be valid")
                .with_async_function("Add", std::future::ready::<BigInt>)
                .err(),
            Some(HostRegistrationError::InvalidFunctionName {
                module: "host/math".into(),
                function: "Add".into(),
            }),
        );
        assert_eq!(
            AsyncHostProviderModule::<crate::StatelessHostProfile>::new("application", "library")
                .expect("provider module should be valid")
                .with_async_function("value", std::future::ready::<BigInt>)
                .expect("first function should be valid")
                .with_async_function("value", std::future::ready::<BigInt>)
                .err(),
            Some(HostRegistrationError::DuplicateFunction {
                module: "library".into(),
                function: "value".into(),
            }),
        );

        let module = AsyncHostModule::new("first", "host/math").expect("module should be valid");
        let provider =
            AsyncHostProviderModule::<crate::StatelessHostProfile>::new("second", "host/math")
                .expect("provider module should be valid");
        assert_eq!(
            AsyncHostProviderSet::with_providers([module], [provider]).err(),
            Some(HostRegistrationError::DuplicateModule {
                module: "host/math".into(),
                first_package: "first".into(),
                second_package: "second".into(),
            }),
        );
    }
}
