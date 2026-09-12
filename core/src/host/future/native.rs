use super::{HostFutureContext, HostFutureType, HostWorkProfile};
use crate::host::execution::NativeScope;
use crate::host::{
    HostCall, HostCallCompletion, HostConstructions, HostExecutionError, HostOwnedCompletion,
    HostProvider, HostType, HostTypeSequence,
};
use std::future::Future;
use std::pin::Pin;

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
                            HostOwnedCompletion<Profile, Provider, Output, Constructions>,
                            HostExecutionError,
                        >,
                    > + Send
                    + 'work,
            >,
        > + Send
        + 'static,
    ) -> HostCallCompletion<'call, HostFutureType<Output, crate::host::HostWorkSchema<Profile>>>
    {
        let context = self.runtime.work();
        let codec = self.runtime.codec_scope();
        let origin = self.runtime.origin();
        let native = move |dependencies| async move {
            let mut scope = NativeScope;
            start(HostFutureContext::new(
                &mut scope,
                context,
                dependencies,
                codec,
                origin,
            ))
            .await
        };
        let work =
            self.runtime
                .work()
                .native(native, self.runtime.codec_scope(), self.runtime.origin());
        self.return_work(work)
    }
}
