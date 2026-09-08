use super::{HostFutureContext, HostFutureError, HostFutureType, HostWorkProfile};
use crate::host::{
    AsyncHostCallError, HostExternal, HostProfile, HostProvider, HostType, HostTypeSequence,
    TransferHostCall, TransferHostCodecScope,
};
use crate::runtime::HostCallOrigin;
use crate::runtime::SharedExecutionError;
use crate::runtime::work::execution::{SourceWork, WorkContext};
use std::future::Future;
use std::marker::PhantomData;

/// One typed source Future retained by native code.
///
/// Receiving this value does not drive it. Observation uses its original
/// execution endpoint and shares the operation's success or failure. Retaining
/// it cannot restart pending work after that execution has ended.
pub struct HostFutureValue<Profile, Provider, Value>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Value: HostType,
{
    work: SourceWork,
    context: WorkContext<Profile>,
    codec: TransferHostCodecScope,
    origin: HostCallOrigin,
    signature: PhantomData<fn(Provider) -> Value>,
}

impl<'call, Profile, Provider, Return> TransferHostCall<'call, Profile, Provider, Return>
where
    Profile: HostWorkProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    /// Retains a Future-valued argument or callback result without observing it.
    pub fn future_value<Value: HostType>(
        &self,
        value: HostExternal<'call, HostFutureType<Value, crate::host::HostWorkSchema<Profile>>>,
    ) -> HostFutureValue<Profile, Provider, Value> {
        let lease = self.runtime.external_lease(value.token);
        HostFutureValue {
            work: crate::host::work_store::<Profile>(self.runtime.external_stores()).work(&lease),
            context: self.runtime.work(),
            codec: self.runtime.codec_scope(),
            origin: self.runtime.origin(),
            signature: PhantomData,
        }
    }
}

impl<Profile, Provider, Value> HostFutureValue<Profile, Provider, Value>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Value: HostType,
{
    /// Explicitly observes this work while the host drives the native operation.
    ///
    /// Each observer supplies its own typed result decoder. The underlying work
    /// and completion are shared, never replayed for another observation.
    pub fn observe<'request, Constructions, Output, Decode>(
        &'request self,
        context: &'request HostFutureContext<'_, Profile, Provider, Constructions>,
        decode: Decode,
    ) -> impl Future<Output = Result<Output, HostFutureError>> + Send + 'request
    where
        Constructions: HostTypeSequence,
        Output: Send + 'static,
        Decode: for<'call> FnOnce(
                TransferHostCall<'call, Profile, Provider, ()>,
                Value::Value<'call>,
            ) -> Result<Output, AsyncHostCallError>
            + Send
            + 'static,
    {
        let observer = context.dependencies.observe(&self.work);
        async move {
            let _scope = context;
            let completion = observer.await?;
            let value = completion
                .read(Clone::clone)
                .map_err(|error| HostFutureError::Execution(SharedExecutionError(error)))?;
            self.context
                .decode_completion(
                    value,
                    self.codec.clone(),
                    self.origin.clone(),
                    move |runtime, token| {
                        let value =
                            crate::host::type_::from_runtime_token::<Value, _>(runtime, token);
                        decode(TransferHostCall::new(runtime), value)
                    },
                )
                .await
        }
    }
}

impl<Profile, Provider, Value> Clone for HostFutureValue<Profile, Provider, Value>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Value: HostType,
{
    fn clone(&self) -> Self {
        Self {
            work: self.work.clone(),
            context: self.context.clone(),
            codec: self.codec.clone(),
            origin: self.origin.clone(),
            signature: PhantomData,
        }
    }
}
