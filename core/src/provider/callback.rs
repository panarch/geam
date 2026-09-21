mod list;

pub use list::{ProviderCallbackListDecoder, ProviderOwnedCallbackListDecoder};

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
    ) -> Result<<Self::HostArguments as HostTypeSequence>::Values<'call>, crate::HostCallError>;

    fn from_host_return<'call>(
        value: <Self::HostReturn as HostType>::Value<'call>,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self::Returned;
}

/// A callback's concrete input/output views; its declaration codec remains private.
#[doc(hidden)]
pub type ProviderOwnedCallbackContext<Profile, Provider, Codec> = ProviderCallbackContext<
    Profile,
    <Codec as ProviderCallbackCodec<Profile, Provider, ()>>::Arguments,
    <Codec as ProviderCallbackCodec<Profile, Provider, ()>>::Returned,
    <Codec as ProviderCallbackCodec<Profile, Provider, ()>>::HostArguments,
    <Codec as ProviderCallbackCodec<Profile, Provider, ()>>::HostReturn,
>;

/// Owned callback storage shared by declarations with the same exact typed views.
///
/// The input/output codec pointers retain their originating declaration's proof.
/// Forwarding a callback never substitutes the receiving declaration's codec.
#[doc(hidden)]
pub struct ProviderCallbackContext<Profile, Arguments, Returned, HostArguments, HostReturn>
where
    Profile: HostProfile,
    HostArguments: HostTypeSequence,
    HostReturn: HostType,
{
    callable:
        HostOwnedCallable<Profile, CallbackProvider, HostArguments, HostReturn, HostTypeListEnd>,
    encode: ArgumentCodec<Profile, Arguments, HostArguments>,
    decode: ReturnCodec<Profile, Returned, HostReturn>,
}

// Stored callbacks use one neutral projection. Their codec function pointers
// retain the declaring provider without exposing it in a transferable view.
struct CallbackProvider;

impl<Profile: HostProfile> HostProvider<Profile> for CallbackProvider {
    type State = Profile::RunState;
    fn project(state: &mut Self::State) -> &mut Self::State {
        state
    }
}

type ArgumentCodec<Profile, Arguments, HostArguments> = for<'call> fn(
    Arguments,
    HostCall<'call, Profile, CallbackProvider, ()>,
    usize,
) -> Result<
    <HostArguments as HostTypeSequence>::Values<'call>,
    crate::HostCallError,
>;

type ReturnCodec<Profile, Returned, HostReturn> = for<'call> fn(
    <HostReturn as HostType>::Value<'call>,
    HostCall<'call, Profile, CallbackProvider, ()>,
    usize,
) -> Returned;

impl<Profile, Arguments, Returned, HostArguments, HostReturn> Clone
    for ProviderCallbackContext<Profile, Arguments, Returned, HostArguments, HostReturn>
where
    Profile: HostProfile,
    HostArguments: HostTypeSequence,
    HostReturn: HostType,
{
    fn clone(&self) -> Self {
        Self {
            callable: self.callable.clone(),
            encode: self.encode,
            decode: self.decode,
        }
    }
}

impl<Signature, Profile, Arguments, Returned, HostArguments, HostReturn>
    Callback<
        Signature,
        ProviderCallbackContext<Profile, Arguments, Returned, HostArguments, HostReturn>,
    >
where
    Profile: HostProfile,
    HostArguments: HostTypeSequence,
    HostReturn: HostType,
{
    #[doc(hidden)]
    pub fn from_owned_host<'call, Codec, Provider, Return: HostType>(
        call: &HostCall<'call, Profile, Provider, Return>,
        callable: HostCallable<'call, HostArguments, HostReturn>,
        constructions: ProviderConstructions<'call, Codec::Requirements>,
    ) -> Self
    where
        Provider: HostProvider<Profile>,
        Codec: ProviderCallbackCodec<
                Profile,
                Provider,
                (),
                HostArguments = HostArguments,
                HostReturn = HostReturn,
                Arguments = Arguments,
                Returned = Returned,
            >,
    {
        Self::from_owned_host_with::<Codec, Provider, Provider, Return>(
            call,
            callable,
            constructions,
        )
    }

    #[doc(hidden)]
    pub fn from_owned_host_with<'call, Codec, Provider, CallerProvider, Return>(
        call: &HostCall<'call, Profile, CallerProvider, Return>,
        callable: HostCallable<'call, HostArguments, HostReturn>,
        constructions: ProviderConstructions<'call, Codec::Requirements>,
    ) -> Self
    where
        Provider: HostProvider<Profile>,
        CallerProvider: HostProvider<Profile>,
        Return: HostType,
        Codec: ProviderCallbackCodec<
                Profile,
                Provider,
                (),
                HostArguments = HostArguments,
                HostReturn = HostReturn,
                Arguments = Arguments,
                Returned = Returned,
            >,
    {
        Self {
            context: ProviderCallbackContext {
                callable: call.owned_callable_with::<CallbackProvider, _, _, _>(
                    callable,
                    &crate::HostConstructions::with_base(constructions.host().callable_base()),
                ),
                encode: encode_arguments::<Profile, Provider, Codec>,
                decode: decode_return::<Profile, Provider, Codec>,
            },
            signature: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn into_host<'call, CallerProvider, Return>(
        self,
        call: &mut HostCall<'call, Profile, CallerProvider, Return>,
    ) -> Result<HostCallable<'call, HostArguments, HostReturn>, crate::HostCallError>
    where
        CallerProvider: HostProvider<Profile>,
        Return: HostType,
    {
        self.context.callable.restore(call)
    }

    pub(crate) fn invoke_owned<'request, CallerProvider, Constructions>(
        &'request self,
        context: &'request HostExecutionContext<'_, Profile, CallerProvider, Constructions>,
        arguments: Arguments,
    ) -> impl Future<Output = Result<Returned, HostExecutionError>> + Send + 'request
    where
        CallerProvider: HostProvider<Profile>,
        Constructions: HostTypeSequence,
        Arguments: Send + 'static,
        Returned: Send + 'static,
    {
        let encode = self.context.encode;
        let decode = self.context.decode;
        self.context.callable.try_invoke(
            context,
            move |call, constructions| encode(arguments, call, constructions.callable_base()),
            move |call, proof, value| Ok(decode(value, call, proof.callable_base())),
        )
    }
}

fn encode_arguments<'call, Profile, Provider, Codec>(
    arguments: Codec::Arguments,
    call: HostCall<'call, Profile, CallbackProvider, ()>,
    callable_base: usize,
) -> Result<<Codec::HostArguments as HostTypeSequence>::Values<'call>, crate::HostCallError>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Codec: ProviderCallbackCodec<Profile, Provider, ()>,
{
    let mut call = call.into_provider::<Provider>();
    let constructions = crate::HostConstructions::with_base(callable_base);
    Codec::into_host_arguments(
        arguments,
        &mut call,
        &ProviderConstructions::new(&constructions),
    )
}

fn decode_return<'call, Profile, Provider, Codec>(
    value: <Codec::HostReturn as HostType>::Value<'call>,
    call: HostCall<'call, Profile, CallbackProvider, ()>,
    callable_base: usize,
) -> Codec::Returned
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Codec: ProviderCallbackCodec<Profile, Provider, ()>,
{
    let mut call = call.into_provider::<Provider>();
    let constructions = crate::HostConstructions::with_base(callable_base);
    Codec::from_host_return(
        value,
        &mut call,
        &ProviderConstructions::new(&constructions),
    )
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

#[cfg(test)]
mod tests {

    #[test]
    fn neutral_projection_preserves_the_borrowed_application_state() {
        struct Profile;
        impl crate::HostProfile for Profile {
            type RunState = Vec<usize>;
            type ExternalStores = ();
            type ExecutionState = ();
        }
        let mut state = vec![3];
        let original = std::ptr::from_mut(&mut state);
        let projected =
            <super::CallbackProvider as crate::HostProvider<Profile>>::project(&mut state);
        assert_eq!(std::ptr::from_mut(projected), original);
        projected.push(5);
        assert_eq!(state, [3, 5]);
    }
}
