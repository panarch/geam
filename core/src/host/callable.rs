use super::{
    HostAbiType, HostAbiTypeSequence, HostCompletion, HostType, HostTypeDescriptor,
    HostTypeSequence,
};
use ecow::EcoString;

/// The static contract of a Rust-created Gleam closure.
///
/// A generic Rust schema can be instantiated with host type parameters when its
/// body is registered and with the constructing function's types when it is used.
/// Its nominal identity always names the same body. Capture values are supplied
/// separately during execution and never become part of the declaration.
pub trait HostCallableSchema: Send + Sync + 'static {
    const PACKAGE: &'static str;
    const MODULE: &'static str;
    const NAME: &'static str;

    type Arguments: HostTypeSequence;
    type Return: HostType;
    type Captures: HostTypeSequence;
    type Constructions: HostTypeSequence;
    type Completion: HostCompletion;
}

/// A scoped Rust implementation of one statically declared native callable body.
pub trait ScopedHostCallable<Profile, Provider, Schema, Arguments>:
    super::function::HostCallableAdapter<Profile, Provider, Schema, Arguments, Schema::Completion>
where
    Profile: crate::HostProfile,
    Provider: crate::HostProvider<Profile>,
    Schema: HostCallableSchema,
{
}

impl<Profile, Provider, Schema, Arguments, Function>
    ScopedHostCallable<Profile, Provider, Schema, Arguments> for Function
where
    Profile: crate::HostProfile,
    Provider: crate::HostProvider<Profile>,
    Schema: HostCallableSchema,
    Function: super::function::HostCallableAdapter<
            Profile,
            Provider,
            Schema,
            Arguments,
            Schema::Completion,
        >,
{
}

/// A native callable body that can suspend and resume on its original execution.
pub trait ResumableHostCallable<Profile, Provider, Schema, Arguments>:
    super::function::ResumableHostCallableAdapter<
        Profile,
        Provider,
        Schema,
        Arguments,
        Schema::Completion,
    >
where
    Profile: crate::HostProfile,
    Provider: crate::HostProvider<Profile>,
    Schema: HostCallableSchema,
{
}

impl<Profile, Provider, Schema, Arguments, Function>
    ResumableHostCallable<Profile, Provider, Schema, Arguments> for Function
where
    Profile: crate::HostProfile,
    Provider: crate::HostProvider<Profile>,
    Schema: HostCallableSchema,
    Function: super::function::ResumableHostCallableAdapter<
            Profile,
            Provider,
            Schema,
            Arguments,
            Schema::Completion,
        >,
{
}

/// Permission to read this invocation's immutable, statically declared captures.
pub struct HostCaptures<'call, Types: HostTypeSequence> {
    marker: CaptureMarker<'call, Types>,
}

type CaptureMarker<'call, Types> = std::marker::PhantomData<fn(&'call ()) -> (&'call (), Types)>;

impl<Types: HostTypeSequence> HostCaptures<'_, Types> {
    pub(super) fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct HostCallableIdentity {
    pub(crate) package: EcoString,
    pub(crate) module: EcoString,
    pub(crate) name: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisteredCallableConstruction {
    pub(crate) identity: HostCallableIdentity,
    pub(crate) captures: Box<[HostTypeDescriptor]>,
    pub(crate) arguments: Box<[HostTypeDescriptor]>,
    pub(crate) return_: HostTypeDescriptor,
    pub(crate) completion: super::HostFunctionBinding<(), ()>,
}

pub(super) struct RegisteredCallableSet<Implementation> {
    functions: Vec<(
        HostCallableIdentity,
        super::HostFunctionBinding<(), ()>,
        super::function::RegisteredHostDefinition<Implementation>,
    )>,
}

pub(crate) struct RegisteredHostCallable {
    pub(crate) identity: HostCallableIdentity,
    pub(crate) completion: super::HostFunctionBinding<(), ()>,
    pub(crate) function: super::RegisteredHostFunction,
}

impl<Value, Never> RegisteredCallableSet<super::HostFunctionBinding<Value, Never>> {
    pub(super) fn into_declarations(
        self,
    ) -> RegisteredCallableSet<super::HostFunctionBinding<(), ()>> {
        RegisteredCallableSet {
            functions: self
                .functions
                .into_iter()
                .map(|(identity, completion, definition)| {
                    (identity, completion, definition.into_declaration())
                })
                .collect(),
        }
    }
}

impl<Implementation> RegisteredCallableSet<Implementation> {
    pub(super) fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub(super) fn register<Schema: HostCallableSchema>(
        &mut self,
        definition: impl FnOnce() -> Result<
            super::function::RegisteredHostDefinition<Implementation>,
            crate::HostRegistrationError,
        >,
    ) -> Result<(), crate::HostRegistrationError> {
        let identity = HostCallableIdentity::of::<Schema>();
        self.validate_registration(&identity)
            .and_then(|()| definition())
            .map(|definition| {
                self.functions.push((
                    identity,
                    <Schema::Completion as super::declaration::CompletionKind>::binding(),
                    definition,
                ));
            })
    }

    fn validate_registration(
        &self,
        identity: &HostCallableIdentity,
    ) -> Result<(), crate::HostRegistrationError> {
        super::module::validate_module_name(&identity.module)?;
        super::module::validate_function_name(&identity.module, &identity.name)?;
        self.check_identity_available(identity)
    }

    pub(super) fn check_available<Schema: HostCallableSchema>(
        &self,
    ) -> Result<(), crate::HostRegistrationError> {
        self.check_identity_available(&HostCallableIdentity::of::<Schema>())
    }

    fn check_identity_available(
        &self,
        identity: &HostCallableIdentity,
    ) -> Result<(), crate::HostRegistrationError> {
        if self
            .functions
            .iter()
            .any(|(registered, _, _)| registered == identity)
        {
            return Err(crate::HostRegistrationError::DuplicateFunction {
                module: identity.module.clone(),
                function: identity.name.clone(),
            });
        }
        Ok(())
    }

    pub(super) fn identities(&self) -> impl Iterator<Item = &HostCallableIdentity> {
        self.functions.iter().map(|(identity, _, _)| identity)
    }

    pub(super) fn into_registered(
        self,
        bindings: &mut super::RegisteredHostBindings<Implementation>,
    ) -> Vec<RegisteredHostCallable> {
        self.functions
            .into_iter()
            .map(
                |(identity, completion, definition)| RegisteredHostCallable {
                    identity,
                    completion,
                    function: bindings.register(definition),
                },
            )
            .collect()
    }
}

pub(super) fn validate_callable_identities<'a>(
    identities: impl Iterator<Item = &'a HostCallableIdentity>,
) -> Result<(), crate::HostRegistrationError> {
    let mut registered = std::collections::HashSet::new();
    for identity in identities {
        if !registered.insert(identity) {
            return Err(crate::HostRegistrationError::DuplicateFunction {
                module: identity.module.clone(),
                function: identity.name.clone(),
            });
        }
    }
    Ok(())
}

impl HostCallableIdentity {
    fn of<Schema: HostCallableSchema>() -> Self {
        Self {
            package: Schema::PACKAGE.into(),
            module: Schema::MODULE.into(),
            name: Schema::NAME.into(),
        }
    }
}

impl RegisteredCallableConstruction {
    pub(crate) fn of<Schema: HostCallableSchema>() -> Self {
        Self {
            identity: HostCallableIdentity::of::<Schema>(),
            captures: <Schema::Captures as HostAbiTypeSequence>::descriptors().into_boxed_slice(),
            arguments: <Schema::Arguments as HostAbiTypeSequence>::descriptors().into_boxed_slice(),
            return_: <Schema::Return as HostAbiType>::descriptor(),
            completion: <Schema::Completion as super::declaration::CompletionKind>::binding(),
        }
    }
}
