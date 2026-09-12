use super::{HostExecutionContext, HostExecutionError, HostOwnedCompletion, NativeScope};
use crate::host::{
    HostCall, HostConstructions, HostProfile, HostProvider, HostType, HostTypeSequence,
};
use crate::runtime::execution::Continuation;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

/// An owned continuation of the current native call, not a Gleam Future value.
pub struct HostCallContinuation<'call, Return> {
    pub(crate) continuation: Continuation,
    call: PhantomData<&'call mut ()>,
    return_: PhantomData<fn() -> Return>,
}

impl<'call, Profile, Provider, Output> HostCall<'call, Profile, Provider, Output>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Output: HostType,
{
    /// Continues this call with owned inputs and short access to its original host.
    ///
    /// The caller's execution driver polls the returned operation. Neither this
    /// method nor registration creates an executor or starts a source Future.
    pub fn resume<Constructions: HostTypeSequence>(
        self,
        _constructions: HostConstructions<'call, Constructions>,
        start: impl for<'run> FnOnce(
            HostExecutionContext<'run, Profile, Provider, Constructions>,
        ) -> Pin<
            Box<
                dyn Future<
                        Output = Result<
                            HostOwnedCompletion<Profile, Provider, Output, Constructions>,
                            HostExecutionError,
                        >,
                    > + Send
                    + 'run,
            >,
        > + Send
        + 'static,
    ) -> HostCallContinuation<'call, Output> {
        let execution = self.runtime.execution();
        let codec = self.runtime.codec_scope();
        let origin = self.runtime.origin();
        let continuation = Continuation::new(async move {
            let mut scope = NativeScope;
            let completion = start(HostExecutionContext::new(
                &mut scope,
                execution.clone(),
                codec.clone(),
                origin.clone(),
            ))
            .await;
            execution.complete_native(completion, codec, origin).await
        });
        HostCallContinuation {
            continuation,
            call: PhantomData,
            return_: PhantomData,
        }
    }
}
