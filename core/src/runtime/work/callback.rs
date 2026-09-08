use super::execution::WorkContext;
use crate::host::{
    AsyncHostCallError, HostFutureError, HostProfile, HostValueToken, TransferHostCallRuntime,
    TransferHostCodecScope,
};
use crate::runtime::host::RuntimeTransferHostCall;
use crate::runtime::{HostCallOrigin, TransferCallable, TransferCallbackInputs};
use std::future::Future;

impl<Profile: HostProfile> WorkContext<Profile> {
    pub(crate) fn decode_completion<Output: Send + 'static, Decode>(
        &self,
        value: super::Shared<crate::runtime::StoredRuntimeValue<crate::runtime::TransferValues>>,
        codec: TransferHostCodecScope,
        origin: HostCallOrigin,
        decode: Decode,
    ) -> impl Future<Output = Result<Output, HostFutureError>> + Send + use<Profile, Output, Decode>
    where
        Decode: FnOnce(
                &mut dyn TransferHostCallRuntime<Profile>,
                HostValueToken,
            ) -> Result<Output, AsyncHostCallError>
            + Send
            + 'static,
    {
        let request = self.with_runtime(move |plan, state| {
            let mut runtime = RuntimeTransferHostCall::new_codec(plan, state, &codec, origin);
            let token = value.read(|value| runtime.restore_stored(value));
            decode(&mut runtime, token)
        });
        async move { request.await?.map_err(Into::into) }
    }

    pub(crate) fn invoke_owned<Output: Send + 'static, Inputs, Decode>(
        &self,
        callable: TransferCallable,
        codec: TransferHostCodecScope,
        origin: HostCallOrigin,
        inputs: Inputs,
        decode: Decode,
    ) -> impl Future<Output = Result<Output, HostFutureError>>
    + Send
    + use<Profile, Output, Inputs, Decode>
    where
        Inputs: FnOnce(&mut dyn TransferHostCallRuntime<Profile>) -> TransferCallbackInputs
            + Send
            + 'static,
        Decode: FnOnce(
                &mut dyn TransferHostCallRuntime<Profile>,
                HostValueToken,
            ) -> Result<Output, AsyncHostCallError>
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
                        RuntimeTransferHostCall::new_codec(plan, state, &input_codec, input_origin);
                    inputs(&mut runtime)
                })
                .await?;
            let returned = context
                .invoke(callable, origin.clone(), inputs)
                .await?
                .map_err(AsyncHostCallError::nested)?;
            context
                .with_runtime(move |plan, state| {
                    let mut runtime =
                        RuntimeTransferHostCall::new_codec(plan, state, &codec, origin);
                    let token = runtime.restore_stored(&returned);
                    decode(&mut runtime, token)
                })
                .await?
                .map_err(Into::into)
        }
    }
}
