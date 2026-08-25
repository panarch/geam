use crate::host::{HostExternalPayloadView, HostStoredType, HostStoredValue, HostTypeSequence};
use crate::provider::advanced::Retained;
use std::marker::PhantomData;

/// Generated identity for one external declaration that may own retained values.
#[doc(hidden)]
pub trait ProviderStoredOwner: 'static {}

/// One generic Gleam value retained by a macro-authored external payload.
///
/// Providers create stored values through [`super::Call::store`] and restore
/// fields exposed by a generated external input through [`super::Call::restore`].
/// A stored value is neither cloneable nor copyable and cannot be constructed
/// independently of its generated external owner.
pub struct Stored<Type, Context = MissingStoredContext> {
    context: Context,
    type_: PhantomData<fn() -> Type>,
}

#[doc(hidden)]
pub struct MissingStoredContext;

/// A retained value being assembled into one generated external payload.
#[doc(hidden)]
pub struct ProviderStoredOutput<'call, Owner, Index, Host> {
    value: HostStoredValue<HostStoredType<Index>>,
    call: PhantomData<&'call ()>,
    owner: PhantomData<fn() -> (Owner, Host)>,
}

/// A borrowed retained field selected from one generated external input.
#[doc(hidden)]
pub struct ProviderStoredInput<'value, Owner, Index, Host> {
    value: &'value HostStoredValue<HostStoredType<Index>>,
    context: PhantomData<fn() -> (Owner, Host)>,
}

/// The exact retained payload view inserted into one generated external input.
#[doc(hidden)]
pub struct ProviderExternalInputContext<'call, Payload, Arguments>
where
    Arguments: HostTypeSequence,
{
    payload: HostExternalPayloadView<'call, Payload, Arguments>,
}

#[doc(hidden)]
pub struct MissingExternalInputContext;

#[doc(hidden)]
pub struct ProviderExternalOutput<Payload> {
    payload: Payload,
}

#[doc(hidden)]
pub struct MissingExternalOutputContext;

impl<Type, Owner, Index, Host> Stored<Type, ProviderStoredOutput<'_, Owner, Index, Host>>
where
    Owner: ProviderStoredOwner,
{
    #[doc(hidden)]
    pub fn from_output(value: HostStoredValue<HostStoredType<Index>>) -> Self {
        Self {
            context: ProviderStoredOutput {
                value,
                call: PhantomData,
                owner: PhantomData,
            },
            type_: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn into_host(self) -> HostStoredValue<HostStoredType<Index>> {
        self.context.value
    }

    /// Moves this newly stored value into an advanced persistent payload.
    pub fn into_retained(self) -> Retained<Owner, Index> {
        Retained::new(self.context.value)
    }
}

impl<'value, Type, Owner, Index, Host> Stored<Type, ProviderStoredInput<'value, Owner, Index, Host>>
where
    Owner: ProviderStoredOwner,
{
    #[doc(hidden)]
    pub fn from_input(value: &'value HostStoredValue<HostStoredType<Index>>) -> Self {
        Self {
            context: ProviderStoredInput {
                value,
                context: PhantomData,
            },
            type_: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn from_retained(value: &'value Retained<Owner, Index>) -> Self {
        Self::from_input(value.host())
    }

    pub(crate) fn host(&self) -> &'value HostStoredValue<HostStoredType<Index>> {
        self.context.value
    }
}

impl<Payload> ProviderExternalOutput<Payload> {
    #[doc(hidden)]
    pub fn new(payload: Payload) -> Self {
        Self { payload }
    }

    #[doc(hidden)]
    pub fn into_payload(self) -> Payload {
        self.payload
    }
}

impl<'call, Payload, Arguments> ProviderExternalInputContext<'call, Payload, Arguments>
where
    Arguments: HostTypeSequence,
{
    #[doc(hidden)]
    pub fn from_host(payload: HostExternalPayloadView<'call, Payload, Arguments>) -> Self {
        Self { payload }
    }

    #[doc(hidden)]
    pub fn payload(&self) -> &Payload {
        &self.payload
    }
}
