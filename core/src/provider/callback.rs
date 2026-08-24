use super::{ProviderConstructionRequirements, ProviderConstructions};
use crate::{
    HostCall, HostCallError, HostCallable, HostProfile, HostProvider, HostType, HostTypeSequence,
};
use std::marker::PhantomData;

type CallbackContextMarker<Profile, Provider, Return> =
    PhantomData<fn() -> (Profile, Provider, Return)>;

/// A call-scoped Gleam function that may be invoked by an active provider call.
///
/// The macro replaces the placeholder context with one exact static callback
/// codec. Opaque function values use [`super::Value`] instead when they only
/// need to pass through without invocation.
pub struct Callback<Signature, Context = MissingCallbackContext> {
    context: Context,
    signature: PhantomData<fn() -> Signature>,
}

#[doc(hidden)]
pub struct MissingCallbackContext;

/// Directional Rust and host conversion selected by one generated callback
/// declaration.
#[doc(hidden)]
pub trait ProviderCallbackCodec<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type HostArguments: HostTypeSequence;
    type HostReturn: HostType;
    type Arguments;
    type Returned;
    type Requirements: ProviderConstructionRequirements;

    fn into_host_arguments(
        arguments: Self::Arguments,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> <Self::HostArguments as HostTypeSequence>::Values<'call>;

    fn from_host_return(
        value: <Self::HostReturn as HostType>::Value<'call>,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> Self::Returned;
}

/// Exact callable and construction proof generated for one callback argument.
#[doc(hidden)]
pub struct ProviderCallbackContext<'call, Profile, Provider, Return, Codec>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Codec: ProviderCallbackCodec<'call, Profile, Provider, Return>,
{
    callable: HostCallable<'call, Codec::HostArguments, Codec::HostReturn>,
    constructions: ProviderConstructions<'call, Codec::Requirements>,
    context: CallbackContextMarker<Profile, Provider, Return>,
}

impl<'call, Signature, Profile, Provider, Return, Codec>
    Callback<Signature, ProviderCallbackContext<'call, Profile, Provider, Return, Codec>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Codec: ProviderCallbackCodec<'call, Profile, Provider, Return>,
{
    #[doc(hidden)]
    pub fn from_host(
        callable: HostCallable<'call, Codec::HostArguments, Codec::HostReturn>,
        constructions: ProviderConstructions<'call, Codec::Requirements>,
    ) -> Self {
        Self {
            context: ProviderCallbackContext {
                callable,
                constructions,
                context: PhantomData,
            },
            signature: PhantomData,
        }
    }

    pub(crate) fn invoke(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        arguments: Codec::Arguments,
    ) -> Result<Codec::Returned, HostCallError> {
        let arguments = Codec::into_host_arguments(arguments, call, &self.context.constructions);
        let returned = call.invoke(self.context.callable, arguments)?;
        Ok(Codec::from_host_return(returned, call))
    }
}

impl<'call, Profile, Provider, Return, Codec> Clone
    for ProviderCallbackContext<'call, Profile, Provider, Return, Codec>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Codec: ProviderCallbackCodec<'call, Profile, Provider, Return>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'call, Profile, Provider, Return, Codec> Copy
    for ProviderCallbackContext<'call, Profile, Provider, Return, Codec>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Codec: ProviderCallbackCodec<'call, Profile, Provider, Return>,
{
}

impl<Signature, Context> Clone for Callback<Signature, Context>
where
    Context: Clone,
{
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            signature: PhantomData,
        }
    }
}

impl<Signature, Context> Copy for Callback<Signature, Context> where Context: Copy {}
