use crate::host::{
    HostCall, HostExecutionError, HostExternal, HostFutureType, HostFutureValue, HostProfile,
    HostProvider, HostType, HostWorkProfile,
};
use std::marker::PhantomData;

/// A Gleam Future received by native provider code.
///
/// Receiving or cloning it does not start the operation. An async provider uses
/// its [`super::Call::observe`] capability to explicitly observe the shared work.
pub struct Future<Value, Context = MissingFutureContext> {
    context: Context,
    value: PhantomData<fn() -> Value>,
}

#[doc(hidden)]
pub struct MissingFutureContext;

type Decode<Profile, Provider, Host, Output> = for<'call> fn(
    HostCall<'call, Profile, Provider, ()>,
    <Host as HostType>::Value<'call>,
) -> Output;

/// The exact native decoder and the work's original execution endpoint.
#[doc(hidden)]
pub struct ProviderFutureValueContext<Profile, Provider, Host, Output>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Host: HostType,
{
    work: HostFutureValue<Profile, Provider, Host>,
    decode: Decode<Profile, Provider, Host, Output>,
}

impl<Value, Profile, Provider, Host, Output>
    Future<Value, ProviderFutureValueContext<Profile, Provider, Host, Output>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Host: HostType,
{
    #[doc(hidden)]
    pub fn from_host<'call, Return: HostType>(
        call: &HostCall<'call, Profile, Provider, Return>,
        value: HostExternal<'call, HostFutureType<Host, crate::host::HostWorkSchema<Profile>>>,
        decode: Decode<Profile, Provider, Host, Output>,
    ) -> Self
    where
        Profile: HostWorkProfile,
    {
        Self {
            context: ProviderFutureValueContext {
                work: call.future_value(value),
                decode,
            },
            value: PhantomData,
        }
    }

    pub(crate) async fn observe(
        &self,
        dependencies: &crate::runtime::work::Dependencies<
            crate::runtime::work::execution::Completion,
        >,
    ) -> Result<Output, HostExecutionError>
    where
        Output: Send + 'static,
    {
        let decode = self.context.decode;
        self.context
            .work
            .observe_dependencies(dependencies, move |call, value| Ok(decode(call, value)))
            .await
    }
}

impl<Profile, Provider, Host, Output> Clone
    for ProviderFutureValueContext<Profile, Provider, Host, Output>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Host: HostType,
{
    fn clone(&self) -> Self {
        Self {
            work: self.work.clone(),
            decode: self.decode,
        }
    }
}

impl<Value, Context: Clone> Clone for Future<Value, Context> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            value: PhantomData,
        }
    }
}
