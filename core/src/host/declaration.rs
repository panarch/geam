use super::function::{HostSignature, RegisteredHostDefinition};
use super::module::{HostModuleIdentity, RegisteredExternalTypes, RegisteredFunctionSet};
use super::{
    HostExternalSchema, HostExternalTypeSchema, HostFunctionBinding, HostFunctionSchema,
    HostRegistrationError, HostType, HostTypeListEnd, HostTypeSequence, RegisteredHostBindings,
    RegisteredHostConstructions, RegisteredHostModule, RegisteredHostProviderModule,
};
use ecow::EcoString;
use std::marker::PhantomData;

/// A native signature and its permitted intermediate constructions, without a body.
///
/// Keep this declaration in a module shared by the application and its preparation
/// helper. The application supplies an implementation using the same declaration.
/// Arguments are host type tuples with arity zero through seven. `Completion`
/// distinguishes a value-producing body from a diverging body before specialization.
pub struct HostFunctionDeclaration<
    Arguments,
    Return,
    Constructions = HostTypeListEnd,
    Completion = HostReturns,
> {
    name: &'static str,
    marker: PhantomData<fn(Arguments, Constructions, Completion) -> Return>,
}

/// The native body may complete with a value of its declared result type.
pub struct HostReturns;

/// The native body cannot complete with a value, even if its result type is inhabited.
pub struct HostDiverges;

/// A sealed choice of value-producing or diverging native completion.
#[allow(private_bounds)]
pub trait HostCompletion: CompletionKind {}

impl HostCompletion for HostReturns {}
impl HostCompletion for HostDiverges {}

/// A bodyless host module used only when preparing immutable execution data.
pub struct HostModuleDeclaration {
    pub(super) identity: HostModuleIdentity,
    pub(super) functions: RegisteredFunctionSet<HostFunctionBinding<(), ()>>,
}

/// Declarations implementing the native functions of an existing Gleam source module.
pub struct HostProviderModuleDeclaration {
    pub(super) identity: HostModuleIdentity,
    pub(super) functions: RegisteredFunctionSet<HostFunctionBinding<(), ()>>,
    pub(super) external_types: RegisteredExternalTypes,
    pub(super) shared_custom_types: super::shared_custom::RegisteredSharedCustomTypes,
    pub(super) callables: super::callable::RegisteredCallableSet<HostFunctionBinding<(), ()>>,
}

/// Complete native contracts for preparation, without callable Rust implementations.
pub struct HostDeclarations {
    pub(super) modules: Vec<HostModuleDeclaration>,
    pub(super) providers: Vec<HostProviderModuleDeclaration>,
    pub(super) callables: super::callable::RegisteredCallableSet<HostFunctionBinding<(), ()>>,
}

pub(super) trait CompletionKind {
    fn binding() -> HostFunctionBinding<(), ()>;
}

#[allow(private_bounds)]
impl<Arguments, Return, Constructions, Completion>
    HostFunctionDeclaration<Arguments, Return, Constructions, Completion>
where
    Arguments: HostSignature<Return>,
    Return: HostType,
    Constructions: HostTypeSequence,
    Completion: HostCompletion,
{
    /// Declares the source-facing name; registration validates it within its module.
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            marker: PhantomData,
        }
    }

    pub(super) fn definition(
        self,
        name: EcoString,
    ) -> Result<RegisteredHostDefinition<HostFunctionBinding<(), ()>>, HostRegistrationError> {
        let (schema, _) = Arguments::signature();
        RegisteredHostDefinition::from_parts(
            name,
            schema,
            RegisteredHostConstructions::for_sequence::<Constructions>(),
            Completion::binding(),
        )
    }
}

impl<Arguments, Return, Constructions, Completion>
    HostFunctionDeclaration<Arguments, Return, Constructions, Completion>
{
    /// The native function name shared by declaration and implementation registration.
    pub const fn name(&self) -> &'static str {
        self.name
    }
}

impl<Arguments, Return, Constructions, Completion> Copy
    for HostFunctionDeclaration<Arguments, Return, Constructions, Completion>
{
}

impl<Arguments, Return, Constructions, Completion> Clone
    for HostFunctionDeclaration<Arguments, Return, Constructions, Completion>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl HostModuleDeclaration {
    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
    ) -> Result<Self, HostRegistrationError> {
        HostModuleIdentity::new(package.into(), module.into()).map(|identity| Self {
            identity,
            functions: RegisteredFunctionSet::new(),
        })
    }

    #[allow(private_bounds)]
    pub fn with_function<Arguments, Return, Constructions, Completion>(
        mut self,
        declaration: HostFunctionDeclaration<Arguments, Return, Constructions, Completion>,
    ) -> Result<Self, HostRegistrationError>
    where
        Arguments: HostSignature<Return>,
        Return: HostType,
        Constructions: HostTypeSequence,
        Completion: HostCompletion,
    {
        self.functions
            .register(&self.identity.module, declaration.name.into(), |name| {
                declaration.definition(name)
            })?;
        Ok(self)
    }

    pub fn functions(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions.schemas()
    }
}

impl HostProviderModuleDeclaration {
    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
    ) -> Result<Self, HostRegistrationError> {
        HostModuleIdentity::new(package.into(), module.into()).map(|identity| Self {
            identity,
            functions: RegisteredFunctionSet::new(),
            external_types: RegisteredExternalTypes::new(),
            shared_custom_types: super::shared_custom::RegisteredSharedCustomTypes::new(),
            callables: super::callable::RegisteredCallableSet::new(),
        })
    }

    #[allow(private_bounds)]
    pub fn with_function<Arguments, Return, Constructions, Completion>(
        mut self,
        declaration: HostFunctionDeclaration<Arguments, Return, Constructions, Completion>,
    ) -> Result<Self, HostRegistrationError>
    where
        Arguments: HostSignature<Return>,
        Return: HostType,
        Constructions: HostTypeSequence,
        Completion: HostCompletion,
    {
        self.functions
            .register(&self.identity.module, declaration.name.into(), |name| {
                declaration.definition(name)
            })?;
        Ok(self)
    }

    /// Declares a private callable body, including its captures and completion kind.
    #[allow(private_bounds)]
    pub fn with_callable<Schema: super::HostCallableSchema>(
        mut self,
    ) -> Result<Self, HostRegistrationError>
    where
        Schema::Arguments: super::function::HostCallableSignature<Schema::Return>,
    {
        self.callables.register::<Schema>(|| {
            let (signature, _) = <Schema::Arguments as super::function::HostCallableSignature<
                Schema::Return,
            >>::signature();
            RegisteredHostDefinition::from_parts(
                Schema::NAME.into(),
                signature.with_captures::<Schema::Captures>(),
                RegisteredHostConstructions::for_sequence::<Schema::Constructions>(),
                <Schema::Completion as CompletionKind>::binding(),
            )
        })?;
        Ok(self)
    }

    /// Declares the source owner's sharing grant without a Rust implementation.
    pub fn with_shared_custom_type<Schema: super::HostCustomSchema>(
        mut self,
    ) -> Result<Self, HostRegistrationError> {
        self.shared_custom_types
            .register(&self.identity, super::HostCustomTypeSchema::of::<Schema>())?;
        Ok(self)
    }

    /// Declares a nominal external type without selecting an application's payload store.
    pub fn with_external_type<Schema: HostExternalSchema>(
        mut self,
    ) -> Result<Self, HostRegistrationError> {
        self.external_types.register(
            &self.identity.module,
            HostExternalTypeSchema::of::<Schema>(),
        )?;
        Ok(self)
    }

    pub fn functions(&self) -> impl ExactSizeIterator<Item = &HostFunctionSchema> {
        self.functions.schemas()
    }
}

type RegisteredDeclarations = (
    Vec<RegisteredHostModule>,
    Vec<RegisteredHostProviderModule>,
    Vec<super::RegisteredHostCallable>,
    RegisteredHostBindings<HostFunctionBinding<(), ()>>,
);

impl HostDeclarations {
    pub(crate) fn select_source_providers(
        mut self,
        source_modules: &std::collections::BTreeSet<(EcoString, EcoString)>,
    ) -> Self {
        self.providers
            .retain(|provider| provider.identity.is_selected(source_modules));
        self
    }

    pub fn new(
        modules: impl IntoIterator<Item = HostModuleDeclaration>,
    ) -> Result<Self, HostRegistrationError> {
        Self::with_providers(modules, [])
    }

    pub fn from_providers(
        providers: impl IntoIterator<Item = HostProviderModuleDeclaration>,
    ) -> Result<Self, HostRegistrationError> {
        Self::with_providers([], providers)
    }

    pub fn with_providers(
        modules: impl IntoIterator<Item = HostModuleDeclaration>,
        providers: impl IntoIterator<Item = HostProviderModuleDeclaration>,
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
        super::module::validate_module_identities(&identities)?;
        super::callable::validate_callable_identities(
            providers
                .iter()
                .flat_map(|provider| provider.callables.identities()),
        )?;
        Ok(Self {
            modules,
            providers,
            callables: super::callable::RegisteredCallableSet::new(),
        })
    }

    /// Declares a private callable body, including its captures and completion kind.
    #[allow(private_bounds)]
    pub fn with_callable<Schema: super::HostCallableSchema>(
        mut self,
    ) -> Result<Self, HostRegistrationError>
    where
        Schema::Arguments: super::function::HostCallableSignature<Schema::Return>,
    {
        for provider in &self.providers {
            provider.callables.check_available::<Schema>()?;
        }
        self.callables.register::<Schema>(|| {
            let (signature, _) = <Schema::Arguments as super::function::HostCallableSignature<
                Schema::Return,
            >>::signature();
            RegisteredHostDefinition::from_parts(
                Schema::NAME.into(),
                signature.with_captures::<Schema::Captures>(),
                RegisteredHostConstructions::for_sequence::<Schema::Constructions>(),
                <Schema::Completion as CompletionKind>::binding(),
            )
        })?;
        Ok(self)
    }

    pub(crate) fn into_registered(self) -> RegisteredDeclarations {
        let mut bindings = RegisteredHostBindings::new();
        let modules = self
            .modules
            .into_iter()
            .map(|module| RegisteredHostModule {
                package: module.identity.package,
                module: module.identity.module,
                functions: module.functions.into_registered(&mut bindings),
            })
            .collect();
        let mut providers = Vec::with_capacity(self.providers.len());
        let mut callables = Vec::new();
        for module in self.providers {
            providers.push(RegisteredHostProviderModule {
                package: module.identity.package,
                module: module.identity.module,
                functions: module.functions.into_registered(&mut bindings),
                external_types: module.external_types.into_vec(),
                shared_custom_types: module.shared_custom_types.into_vec(),
            });
            callables.extend(module.callables.into_registered(&mut bindings));
        }
        callables.extend(self.callables.into_registered(&mut bindings));
        (modules, providers, callables, bindings)
    }
}

impl CompletionKind for HostReturns {
    fn binding() -> HostFunctionBinding<(), ()> {
        HostFunctionBinding::Value(())
    }
}

impl CompletionKind for HostDiverges {
    fn binding() -> HostFunctionBinding<(), ()> {
        HostFunctionBinding::Never(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostDeclarations, HostFunctionDeclaration, HostModuleDeclaration,
        HostProviderModuleDeclaration,
    };
    use crate::plan::{FunctionType, ValueType};
    use crate::{
        HostCallableSchema, HostExternalSchema, HostRegistrationError, HostReturns, HostTypeList,
        HostTypeListEnd,
    };
    use num_bigint::BigInt;
    use std::collections::BTreeSet;

    struct Handle;
    impl HostExternalSchema for Handle {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "source";
        const NAME: &'static str = "Handle";
        const PARAMETER_COUNT: usize = 0;
    }

    struct Increment;
    impl HostCallableSchema for Increment {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "private/callbacks";
        const NAME: &'static str = "increment";
        type Arguments = HostTypeList<BigInt, HostTypeListEnd>;
        type Return = BigInt;
        type Captures = HostTypeList<BigInt, HostTypeListEnd>;
        type Constructions = HostTypeListEnd;
        type Completion = HostReturns;
    }

    #[test]
    fn private_callable_declarations_reject_invalid_identities_and_sparse_parameters() {
        macro_rules! declaration {
            ($schema:ident, $module:literal, $name:literal, $value:ty) => {
                struct $schema;
                impl HostCallableSchema for $schema {
                    const PACKAGE: &'static str = "app";
                    const MODULE: &'static str = $module;
                    const NAME: &'static str = $name;
                    type Arguments = HostTypeList<$value, HostTypeListEnd>;
                    type Return = $value;
                    type Captures = HostTypeListEnd;
                    type Constructions = HostTypeListEnd;
                    type Completion = HostReturns;
                }
            };
        }
        declaration!(BadModule, "Bad", "identity", BigInt);
        declaration!(BadFunction, "callbacks", "Bad", BigInt);
        declaration!(Sparse, "callbacks", "sparse", crate::HostTypeParameter<1>);
        assert_eq!(
            HostDeclarations::new([])
                .unwrap()
                .with_callable::<BadModule>()
                .err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "Bad".into()
            })
        );
        assert_eq!(
            HostDeclarations::new([])
                .unwrap()
                .with_callable::<BadFunction>()
                .err(),
            Some(HostRegistrationError::InvalidFunctionName {
                module: "callbacks".into(),
                function: "Bad".into()
            })
        );
        let expected = || HostRegistrationError::NonContiguousTypeParameters {
            function: "sparse".into(),
            parameters: Box::new([1]),
        };
        assert_eq!(
            HostDeclarations::new([])
                .unwrap()
                .with_callable::<Sparse>()
                .err(),
            Some(expected())
        );
        assert_eq!(
            HostProviderModuleDeclaration::new("app", "source")
                .unwrap()
                .with_callable::<Sparse>()
                .err(),
            Some(expected())
        );
    }

    #[test]
    fn source_less_and_source_backed_declarations_expose_the_same_exact_signature() {
        const IDENTITY: HostFunctionDeclaration<(BigInt,), BigInt> =
            HostFunctionDeclaration::new("identity");
        // Copying a declaration carries its contract without requiring its marker
        // types to implement Clone. Exercise Clone as part of that public protocol.
        let copied = <HostFunctionDeclaration<(BigInt,), BigInt> as Clone>::clone(&IDENTITY);
        assert_eq!(copied.name(), "identity");
        let module = HostModuleDeclaration::new("host", "native/math")
            .unwrap()
            .with_function(copied)
            .unwrap();
        let provider = HostProviderModuleDeclaration::new("application", "source")
            .unwrap()
            .with_function(IDENTITY)
            .unwrap()
            .with_external_type::<Handle>()
            .unwrap();
        for schemas in [
            module.functions().collect::<Vec<_>>(),
            provider.functions().collect(),
        ] {
            assert_eq!(schemas.len(), 1);
            assert_eq!(schemas[0].name(), "identity");
            assert_eq!(
                schemas[0].type_(),
                &FunctionType::new(vec![ValueType::Int], ValueType::Int)
            );
        }
        let declarations = HostDeclarations::with_providers([module], [provider]).unwrap();
        let (modules, providers, callables, _) = declarations.into_registered();
        assert_eq!(
            (&*modules[0].package, &*modules[0].module),
            ("host", "native/math")
        );
        assert_eq!(
            (&*providers[0].package, &*providers[0].module),
            ("application", "source")
        );
        assert_eq!(
            providers[0].external_types,
            [crate::HostExternalTypeSchema::of::<Handle>()]
        );
        assert!(callables.is_empty());
    }

    #[test]
    fn declaration_registration_rejects_names_duplicates_and_unbound_constructions() {
        const VALUE: HostFunctionDeclaration<(), BigInt> = HostFunctionDeclaration::new("value");
        const INVALID: HostFunctionDeclaration<(), BigInt> = HostFunctionDeclaration::new("Bad");
        const UNBOUND: HostFunctionDeclaration<
            (),
            BigInt,
            HostTypeList<crate::HostTypeParameter<0>, HostTypeListEnd>,
        > = HostFunctionDeclaration::new("unbound");
        let invalid_module = HostRegistrationError::InvalidModuleName {
            module: "Bad".into(),
        };
        assert_eq!(
            HostModuleDeclaration::new("app", "Bad").err(),
            Some(invalid_module)
        );
        assert_eq!(
            HostProviderModuleDeclaration::new("app", "Bad").err(),
            Some(HostRegistrationError::InvalidModuleName {
                module: "Bad".into()
            })
        );
        let invalid = || HostRegistrationError::InvalidFunctionName {
            module: "source".into(),
            function: "Bad".into(),
        };
        assert_eq!(
            HostModuleDeclaration::new("app", "source")
                .unwrap()
                .with_function(INVALID)
                .err(),
            Some(invalid())
        );
        assert_eq!(
            HostProviderModuleDeclaration::new("app", "source")
                .unwrap()
                .with_function(INVALID)
                .err(),
            Some(invalid())
        );
        let duplicate = || HostRegistrationError::DuplicateFunction {
            module: "source".into(),
            function: "value".into(),
        };
        assert_eq!(
            HostModuleDeclaration::new("app", "source")
                .unwrap()
                .with_function(VALUE)
                .unwrap()
                .with_function(VALUE)
                .err(),
            Some(duplicate())
        );
        assert_eq!(
            HostProviderModuleDeclaration::new("app", "source")
                .unwrap()
                .with_function(VALUE)
                .unwrap()
                .with_function(VALUE)
                .err(),
            Some(duplicate())
        );
        let unbound = || HostRegistrationError::UnboundConstructionTypeParameters {
            function: "unbound".into(),
            parameters: Box::new([0]),
        };
        assert_eq!(
            HostModuleDeclaration::new("app", "source")
                .unwrap()
                .with_function(UNBOUND)
                .err(),
            Some(unbound())
        );
        assert_eq!(
            HostProviderModuleDeclaration::new("app", "source")
                .unwrap()
                .with_function(UNBOUND)
                .err(),
            Some(unbound())
        );
        assert_eq!(
            HostProviderModuleDeclaration::new("application", "source")
                .unwrap()
                .with_external_type::<Handle>()
                .unwrap()
                .with_external_type::<Handle>()
                .err(),
            Some(HostRegistrationError::DuplicateExternalType {
                module: "source".into(),
                type_: "Handle".into()
            })
        );
        assert_eq!(
            HostDeclarations::with_providers(
                [HostModuleDeclaration::new("host", "source").unwrap()],
                [HostProviderModuleDeclaration::new("application", "source").unwrap()],
            )
            .err(),
            Some(HostRegistrationError::DuplicateModule {
                module: "source".into(),
                first_package: "host".into(),
                second_package: "application".into()
            })
        );
    }

    #[test]
    fn private_declarations_follow_source_selection_and_have_one_registration_owner() {
        let provider = |module| {
            HostProviderModuleDeclaration::new("application", module)
                .unwrap()
                .with_callable::<Increment>()
                .unwrap()
        };
        let duplicate = || HostRegistrationError::DuplicateFunction {
            module: "private/callbacks".into(),
            function: "increment".into(),
        };
        assert_eq!(
            provider("source").with_callable::<Increment>().err(),
            Some(duplicate())
        );
        assert_eq!(
            HostDeclarations::from_providers([provider("one"), provider("one")]).err(),
            Some(HostRegistrationError::DuplicateModule {
                module: "one".into(),
                first_package: "application".into(),
                second_package: "application".into()
            })
        );
        assert_eq!(
            HostDeclarations::from_providers([provider("one"), provider("two")]).err(),
            Some(duplicate())
        );
        assert_eq!(
            HostDeclarations::from_providers([provider("source")])
                .unwrap()
                .with_callable::<Increment>()
                .err(),
            Some(duplicate())
        );
        assert_eq!(
            HostDeclarations::new([])
                .unwrap()
                .with_callable::<Increment>()
                .unwrap()
                .with_callable::<Increment>()
                .err(),
            Some(duplicate())
        );
        for selected in [false, true] {
            let sources = if selected {
                BTreeSet::from([("application".into(), "source".into())])
            } else {
                BTreeSet::new()
            };
            let (_, providers, callables, _) =
                HostDeclarations::from_providers([provider("source")])
                    .unwrap()
                    .select_source_providers(&sources)
                    .into_registered();
            assert_eq!(providers.len(), usize::from(selected));
            assert_eq!(callables.len(), usize::from(selected));
            if selected {
                assert_eq!(
                    callables[0].identity,
                    crate::host::callable::HostCallableIdentity {
                        package: "application".into(),
                        module: "private/callbacks".into(),
                        name: "increment".into()
                    }
                );
                assert_eq!(
                    callables[0].completion,
                    crate::host::HostFunctionBinding::Value(())
                );
            }
        }
        // An application-owned declaration remains available even without a
        // source-provider selection; a different provider does not shadow it.
        let (_, providers, callables, _) =
            HostDeclarations::from_providers([HostProviderModuleDeclaration::new(
                "application",
                "source",
            )
            .unwrap()])
            .unwrap()
            .with_callable::<Increment>()
            .unwrap()
            .select_source_providers(&BTreeSet::new())
            .into_registered();
        assert!(providers.is_empty());
        assert_eq!(callables.len(), 1);
        assert_eq!(callables[0].identity.name, "increment");
    }
}
