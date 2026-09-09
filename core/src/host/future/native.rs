use super::context::NativeScope;
use super::{HostFutureContext, HostFutureError, HostFutureType, HostWorkProfile};
use crate::host::{
    HostCall, HostCallCompletion, HostCallError, HostCallRuntime, HostConstructions, HostProfile,
    HostProvider, HostType, HostTypeSequence, HostValueToken,
};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

type CompletionCodec<Profile> =
    dyn FnOnce(&mut dyn HostCallRuntime<Profile>) -> Result<HostValueToken, HostCallError> + Send;

/// An owned result and its exact codec, waiting for the originating execution.
///
/// The result may cross await points. Its codec receives fresh call-scoped views
/// only when the Rust host drives this Future's completion.
pub struct HostFutureCompletion<Profile, Provider, Output, Constructions>
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
    HostFutureCompletion<Profile, Provider, Output, Constructions>
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
            codec: Box::new(move |runtime| {
                complete(HostCall::new(runtime), HostConstructions::new())
                    .map(|completion| completion.token)
            }),
            signature: PhantomData,
        }
    }

    pub(crate) fn complete(
        self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<HostValueToken, HostCallError> {
        (self.codec)(runtime)
    }
}

impl<'call, Profile, Provider, Output>
    HostCall<'call, Profile, Provider, HostFutureType<Output, crate::host::HostWorkSchema<Profile>>>
where
    Profile: HostWorkProfile,
    Provider: HostProvider<Profile>,
    Output: HostType,
{
    /// Constructs work without polling it or borrowing the creating invocation.
    ///
    /// Construction permission is transferred from this registered call. The
    /// native Future's completion retains the same output and permission types.
    pub fn return_future<Constructions: HostTypeSequence>(
        self,
        _constructions: HostConstructions<'call, Constructions>,
        start: impl for<'work> FnOnce(
            HostFutureContext<'work, Profile, Provider, Constructions>,
        ) -> Pin<
            Box<
                dyn Future<
                        Output = Result<
                            HostFutureCompletion<Profile, Provider, Output, Constructions>,
                            HostFutureError,
                        >,
                    > + Send
                    + 'work,
            >,
        > + Send
        + 'static,
    ) -> HostCallCompletion<'call, HostFutureType<Output, crate::host::HostWorkSchema<Profile>>>
    {
        let context = self.runtime.work();
        let native = move |dependencies| async move {
            let mut scope = NativeScope;
            start(HostFutureContext::new(&mut scope, context, dependencies)).await
        };
        let work =
            self.runtime
                .work()
                .native(native, self.runtime.codec_scope(), self.runtime.origin());
        self.return_work(work)
    }
}
