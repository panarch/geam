use crate::host::{
    HostCall, HostCallCompletion, HostCallError, HostCallRuntime, HostConstructions, HostProfile,
    HostProvider, HostType, HostTypeSequence, HostValueToken,
};
use std::marker::PhantomData;

type CompletionCodec<Profile> = dyn FnOnce(&mut dyn HostCallRuntime<Profile>, usize) -> Result<HostValueToken, HostCallError>
    + Send;

/// An owned result and its exact codec, waiting for the originating execution.
///
/// The result may cross await points. Its codec receives fresh call-scoped views
/// only when the Rust host services the operation's completion.
pub struct HostOwnedCompletion<Profile, Provider, Output, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Output: HostType,
    Constructions: HostTypeSequence,
{
    codec: Box<CompletionCodec<Profile>>,
    signature: PhantomData<fn(Provider, Constructions) -> Output>,
}

impl<Profile, Provider, Output, Constructions>
    HostOwnedCompletion<Profile, Provider, Output, Constructions>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Output: HostType,
    Constructions: HostTypeSequence,
{
    /// Moves the native result into a codec for the registered output type.
    pub fn new(
        complete: impl for<'call> FnOnce(
            HostCall<'call, Profile, Provider, Output>,
            HostConstructions<'call, Constructions>,
        )
            -> Result<HostCallCompletion<'call, Output>, HostCallError>
        + Send
        + 'static,
    ) -> Self {
        Self {
            codec: Box::new(move |runtime, callable_base| {
                complete(
                    HostCall::new(runtime),
                    HostConstructions::with_base(callable_base),
                )
                .map(|completion| completion.token)
            }),
            signature: PhantomData,
        }
    }

    pub(crate) fn complete(
        self,
        runtime: &mut dyn HostCallRuntime<Profile>,
        callable_base: usize,
    ) -> Result<HostValueToken, HostCallError> {
        (self.codec)(runtime, callable_base)
    }
}
