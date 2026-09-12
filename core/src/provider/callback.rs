use super::{ProviderConstructionRequirements, ProviderConstructions};
use crate::host::{
    HostExecutionContext, HostExecutionError, HostOwnedCallable, HostProfile, HostProvider,
    HostType, HostTypeListEnd, HostTypeSequence,
};
use crate::{HostCall, HostCallable};
use std::future::Future;
use std::marker::PhantomData;

/// An owned Gleam function invoked through a resumable or Future provider call.
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

/// Owned Rust and transferable-host conversion selected by one generated
/// callback declaration.
#[doc(hidden)]
pub trait ProviderCallbackCodec<Profile, Provider, Return>
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

    fn into_host_arguments<'call>(
        arguments: Self::Arguments,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> <Self::HostArguments as HostTypeSequence>::Values<'call>;

    fn from_host_return<'call>(
        value: <Self::HostReturn as HostType>::Value<'call>,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> Self::Returned;
}

/// Owned callable and construction proof for a native operation.
#[doc(hidden)]
pub struct ProviderOwnedCallbackContext<Profile, Provider, Codec>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Codec: ProviderCallbackCodec<Profile, Provider, ()>,
{
    callable: OwnedCallback<Profile, Provider, Codec>,
}

type OwnedCallback<Profile, Provider, Codec> = HostOwnedCallable<
    Profile,
    Provider,
    <Codec as ProviderCallbackCodec<Profile, Provider, ()>>::HostArguments,
    <Codec as ProviderCallbackCodec<Profile, Provider, ()>>::HostReturn,
    <<Codec as ProviderCallbackCodec<Profile, Provider, ()>>::Requirements as
        ProviderConstructionRequirements>::Types<HostTypeListEnd>,
>;

impl<Profile, Provider, Codec> Clone for ProviderOwnedCallbackContext<Profile, Provider, Codec>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Codec: ProviderCallbackCodec<Profile, Provider, ()>,
{
    fn clone(&self) -> Self {
        Self {
            callable: self.callable.clone(),
        }
    }
}

impl<Signature, Profile, Provider, Codec>
    Callback<Signature, ProviderOwnedCallbackContext<Profile, Provider, Codec>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Codec: ProviderCallbackCodec<Profile, Provider, ()>,
{
    #[doc(hidden)]
    pub fn from_owned_host<'call, Return: HostType>(
        call: &HostCall<'call, Profile, Provider, Return>,
        callable: HostCallable<'call, Codec::HostArguments, Codec::HostReturn>,
        constructions: ProviderConstructions<'call, Codec::Requirements>,
    ) -> Self {
        Self {
            context: ProviderOwnedCallbackContext {
                callable: call.owned_callable(callable, &constructions.host()),
            },
            signature: PhantomData,
        }
    }

    pub(crate) fn invoke_owned<'request>(
        &'request self,
        context: &'request HostExecutionContext<'_, Profile, Provider, HostTypeListEnd>,
        arguments: Codec::Arguments,
    ) -> impl Future<Output = Result<Codec::Returned, HostExecutionError>> + Send + 'request
    where
        Codec: 'static,
        Codec::Arguments: Send + 'static,
        Codec::Returned: Send + 'static,
    {
        self.context.callable.invoke(
            context,
            move |mut call, constructions| {
                Codec::into_host_arguments(
                    arguments,
                    &mut call,
                    &ProviderConstructions::new(&constructions),
                )
            },
            |mut call, _, value| Ok(Codec::from_host_return(value, &mut call)),
        )
    }
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
