use super::{
    FallibleHostFunction, HostFunction, HostFunctionDefinition, HostFunctionImplementation,
    HostFunctionSchema, HostProfile, HostProvider, HostRegistrationError, ScopedHostFunction,
    StatelessHostProfile,
};
use ecow::EcoString;
use gleam_core::analyse::name::check_name_case;
use gleam_core::ast::SrcSpan;
use gleam_core::parse::lexer::string_to_keyword;
use gleam_core::type_::PRELUDE_MODULE_NAME;
use gleam_core::type_::error::Named;
use std::collections::BTreeMap;

pub struct HostModule<Profile: HostProfile = StatelessHostProfile> {
    identity: HostModuleIdentity,
    functions: RegisteredFunctions<Profile>,
}

pub struct HostProviderModule<Profile: HostProfile> {
    identity: HostModuleIdentity,
    functions: RegisteredFunctions<Profile>,
}

struct HostModuleIdentity {
    package: EcoString,
    module: EcoString,
}

pub struct HostProviderSet<Profile: HostProfile = StatelessHostProfile> {
    modules: Vec<HostModule<Profile>>,
    providers: Vec<HostProviderModule<Profile>>,
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
}

pub(crate) struct RegisteredHostFunction {
    schema: HostFunctionSchema,
    implementation: RegisteredHostImplementationId,
}

#[derive(Clone, Copy)]
pub(crate) struct RegisteredHostImplementationId(usize);

pub(crate) struct RegisteredHostImplementations<Profile: HostProfile> {
    functions: Vec<HostFunctionImplementation<Profile>>,
}

struct RegisteredFunctions<Profile: HostProfile> {
    functions: Vec<HostFunctionDefinition<Profile>>,
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
            .register(
                &self.identity.module,
                HostFunctionDefinition::new(name.into(), function),
            )
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
            .register(
                &self.identity.module,
                HostFunctionDefinition::new_fallible(name.into(), function),
            )
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
            .register(
                &self.identity.module,
                HostFunctionDefinition::new_scoped::<Provider, _, _, _>(name.into(), function),
            )
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
            .register(
                &self.identity.module,
                HostFunctionDefinition::new(name.into(), function),
            )
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
            .register(
                &self.identity.module,
                HostFunctionDefinition::new_fallible(name.into(), function),
            )
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
            .register(
                &self.identity.module,
                HostFunctionDefinition::new_scoped::<Provider, _, _, _>(name.into(), function),
            )
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

impl<Profile: HostProfile> HostProviderSet<Profile> {
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
            });
        }
        (modules, providers, implementations)
    }
}

impl HostModuleIdentity {
    fn new(package: EcoString, module: EcoString) -> Result<Self, HostRegistrationError> {
        validate_module_name(&module)?;
        Ok(Self { package, module })
    }
}

fn validate_module_identities(
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
        function: HostFunctionDefinition<Profile>,
    ) -> Result<(), HostRegistrationError> {
        let name = function.schema().name().clone();
        if string_to_keyword(&name).is_some()
            || check_name_case(SrcSpan::new(0, 0), &name, Named::Function).is_err()
        {
            return Err(HostRegistrationError::InvalidFunctionName {
                module: module.clone(),
                function: name,
            });
        }
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
        self.functions.push(function);
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
    pub(crate) fn into_parts(self) -> (EcoString, EcoString, Vec<RegisteredHostFunction>) {
        (self.package, self.module, self.functions)
    }
}

impl RegisteredHostFunction {
    pub(crate) fn schema(&self) -> &HostFunctionSchema {
        &self.schema
    }

    pub(crate) fn into_parts(self) -> (HostFunctionSchema, RegisteredHostImplementationId) {
        (self.schema, self.implementation)
    }
}

impl<Profile: HostProfile> RegisteredHostImplementations<Profile> {
    fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    fn register(&mut self, definition: HostFunctionDefinition<Profile>) -> RegisteredHostFunction {
        let (schema, implementation) = definition.into_parts();
        let id = RegisteredHostImplementationId(self.functions.len());
        self.functions.push(implementation);
        RegisteredHostFunction {
            schema,
            implementation: id,
        }
    }

    pub(crate) fn implementation(
        &self,
        id: RegisteredHostImplementationId,
    ) -> HostFunctionImplementation<Profile> {
        self.functions[id.0].clone()
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

#[cfg(test)]
mod tests {
    use super::{HostModule, HostProviderModule, HostProviderSet};
    use crate::host::function::CallArguments;
    use crate::host::{
        HostCall, HostFailure, HostFunctionImplementation, HostIntFunction, HostProfile,
        HostProvider, HostRegistrationError, StatelessHostProfile,
    };
    use crate::plan::ValueType;
    use num_bigint::BigInt;

    struct Profile;

    struct Counter;

    impl HostProfile for Profile {
        type RunState = usize;
    }

    impl HostProvider<Profile> for Counter {
        type State = usize;

        fn project(state: &mut usize) -> &mut Self::State {
            state
        }
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
    fn scoped_registration_projects_provider_state() {
        let provider = HostProviderModule::<Profile>::new("application", "main")
            .expect("provider module should be valid")
            .with_scoped_function::<Counter, _, _, _>(
                "increment",
                |call: &mut HostCall<'_, Profile, Counter>| {
                    *call.state() += 1;
                    Ok(BigInt::from(*call.state()))
                },
            )
            .expect("scoped function should be valid");

        assert_eq!(provider.functions().len(), 1);
        let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), [provider])
            .expect("provider module should be unique");
        let (_, mut providers, implementations) = hosts.into_registered();
        let (_, _, mut definitions) = providers
            .pop()
            .expect("provider module should be registered")
            .into_parts();
        let (_, implementation) = definitions
            .pop()
            .expect("scoped function should be registered")
            .into_parts();
        let implementation = int_function(implementations.implementation(implementation));
        let mut state = 41;

        assert_eq!(
            implementation.call(&mut state, &CallArguments::new(Vec::new(), Vec::new()),),
            Ok(BigInt::from(42)),
        );
        assert_eq!(state, 42);
    }

    #[test]
    fn source_less_profile_registration_invokes_fallible_and_scoped_callbacks() {
        let module = HostModule::<Profile>::new_for_profile("host_support", "host/state")
            .expect("module should be valid")
            .with_fallible_function("checked", || {
                Result::<BigInt, HostFailure>::Ok(BigInt::from(7))
            })
            .expect("fallible function should be valid")
            .with_scoped_function::<Counter, _, _, _>(
                "increment",
                |call: &mut HostCall<'_, Profile, Counter>| {
                    *call.state() += 1;
                    Ok(BigInt::from(*call.state()))
                },
            )
            .expect("scoped function should be valid");
        let hosts = HostProviderSet::new([module]).expect("host module should be unique");
        let (mut modules, _, implementations) = hosts.into_registered();
        let (_, _, definitions) = modules
            .pop()
            .expect("host module should be registered")
            .into_parts();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut definitions = definitions.into_iter();
        let (_, checked) = definitions
            .next()
            .expect("fallible function should be registered")
            .into_parts();
        let checked = int_function(implementations.implementation(checked));
        let (_, increment) = definitions
            .next()
            .expect("scoped function should be registered")
            .into_parts();
        let increment = int_function(implementations.implementation(increment));
        let mut state = 9;

        assert_eq!(checked.call(&mut state, &arguments), Ok(BigInt::from(7)));
        assert_eq!(increment.call(&mut state, &arguments), Ok(BigInt::from(10)),);
        assert_eq!(state, 10);
    }

    #[test]
    #[should_panic(expected = "registered function should return Int")]
    fn registered_int_function_shape_guard_is_visible() {
        let module = HostModule::<Profile>::new_for_profile("host_support", "host/state")
            .expect("module should be valid")
            .with_function("ready", || true)
            .expect("function should be valid");
        let hosts = HostProviderSet::new([module]).expect("host module should be unique");
        let (mut modules, _, implementations) = hosts.into_registered();
        let (_, _, mut definitions) = modules
            .pop()
            .expect("host module should be registered")
            .into_parts();
        let (_, implementation) = definitions
            .pop()
            .expect("function should be registered")
            .into_parts();

        int_function(implementations.implementation(implementation));
    }

    #[test]
    fn rejects_invalid_module_and_function_names() {
        assert_eq!(
            HostModule::<Profile>::new_for_profile("host_support", "").err(),
            Some(HostRegistrationError::InvalidModuleName { module: "".into() }),
        );
        assert_eq!(
            HostProviderModule::<Profile>::new("host_support", "gleam").err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "gleam".into(),
            }),
        );
        assert_eq!(
            HostModule::<Profile>::new_for_profile("host_support", "host/math")
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
            HostModule::<Profile>::new_for_profile("host_support", "host/math")
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

    fn int_function<Host: HostProfile>(
        implementation: HostFunctionImplementation<Host>,
    ) -> HostIntFunction<Host> {
        let HostFunctionImplementation::Int(implementation) = implementation else {
            panic!("registered function should return Int");
        };
        implementation
    }
}
