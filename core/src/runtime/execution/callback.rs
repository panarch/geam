use super::ExecutionContext;
use crate::host::{
    HostCallError, HostCallRuntime, HostCodecScope, HostExecutionError, HostProfile, HostValueToken,
};
use crate::runtime::host::RuntimeHostCall;
use crate::runtime::{CallbackInputs, HostCallOrigin, RetainedCallable};
use std::future::Future;

impl<Profile: HostProfile> ExecutionContext<Profile> {
    pub(crate) fn with_codec<Output: Send + 'static, Operation>(
        &self,
        codec: HostCodecScope,
        origin: HostCallOrigin,
        operation: Operation,
    ) -> impl Future<Output = Result<Output, crate::runtime::work::Cancelled>>
    + Send
    + use<Profile, Output, Operation>
    where
        Operation: FnOnce(&mut dyn HostCallRuntime<Profile>) -> Output + Send + 'static,
    {
        self.with_runtime(move |plan, state| {
            let mut runtime = RuntimeHostCall::new_codec(plan, state, &codec, origin);
            operation(&mut runtime)
        })
    }

    pub(crate) fn decode_completion<Output: Send + 'static, Decode>(
        &self,
        value: crate::runtime::shared::Shared<crate::runtime::StoredRuntimeValue>,
        codec: HostCodecScope,
        origin: HostCallOrigin,
        decode: Decode,
    ) -> impl Future<Output = Result<Output, HostExecutionError>> + Send + use<Profile, Output, Decode>
    where
        Decode: FnOnce(
                &mut dyn HostCallRuntime<Profile>,
                HostValueToken,
            ) -> Result<Output, HostCallError>
            + Send
            + 'static,
    {
        let request = self.with_runtime(move |plan, state| {
            let mut runtime = RuntimeHostCall::new_codec(plan, state, &codec, origin);
            let token = value.read(|value| runtime.restore_stored(value));
            decode(&mut runtime, token)
        });
        async move { request.await?.map_err(Into::into) }
    }

    pub(crate) fn invoke_owned<Output: Send + 'static, Inputs, Decode>(
        &self,
        callable: RetainedCallable,
        codec: HostCodecScope,
        origin: HostCallOrigin,
        inputs: Inputs,
        decode: Decode,
    ) -> impl Future<Output = Result<Output, HostExecutionError>>
    + Send
    + use<Profile, Output, Inputs, Decode>
    where
        Inputs: FnOnce(&mut dyn HostCallRuntime<Profile>) -> CallbackInputs + Send + 'static,
        Decode: FnOnce(
                &mut dyn HostCallRuntime<Profile>,
                HostValueToken,
            ) -> Result<Output, HostCallError>
            + Send
            + 'static,
    {
        let context = self.clone();
        async move {
            let input_codec = codec.clone();
            let input_origin = origin.clone();
            let inputs = context
                .with_runtime(move |plan, state| {
                    let mut runtime =
                        RuntimeHostCall::new_codec(plan, state, &input_codec, input_origin);
                    inputs(&mut runtime)
                })
                .await?;
            let returned = context
                .invoke(callable, origin.clone(), inputs)
                .await?
                .map_err(HostCallError::nested)?;
            context
                .with_runtime(move |plan, state| {
                    let mut runtime = RuntimeHostCall::new_codec(plan, state, &codec, origin);
                    let token = runtime.restore_stored(&returned);
                    decode(&mut runtime, token)
                })
                .await?
                .map_err(Into::into)
        }
    }
}
