use crate::host::TransferHostCall;
use crate::runtime::{StoredRuntimeValue, TransferValues};
use crate::{HostProfile, HostProvider, HostType};
use std::marker::PhantomData;

/// A call-scoped opaque handle for one statically declared source type.
///
/// Provider macros use this type for generic source values and opaque function
/// pass-through. It does not expose the concrete runtime family or materialize
/// the represented value. Use an active [`super::Call`] for source equality,
/// hashing, and inspection.
pub struct Value<Type, Context = MissingValueContext> {
    context: Context,
    type_: PhantomData<fn() -> Type>,
}

#[doc(hidden)]
pub struct MissingValueContext;

/// The exact typed host handle inserted by provider macro expansion.
#[doc(hidden)]
pub struct ProviderValueContext<'call, Host>
where
    Host: HostType,
{
    value: Host::Value<'call>,
}

/// Owned retained value used by a transferable provider invocation.
#[doc(hidden)]
pub struct ProviderTransferValueContext<Host>
where
    Host: HostType,
{
    value: StoredRuntimeValue<TransferValues>,
    host: PhantomData<fn() -> Host>,
}

impl<'call, Type, Host> Value<Type, ProviderValueContext<'call, Host>>
where
    Host: HostType,
{
    #[doc(hidden)]
    pub fn from_host(value: Host::Value<'call>) -> Self {
        Self {
            context: ProviderValueContext { value },
            type_: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn into_host(self) -> Host::Value<'call> {
        self.context.value
    }

    pub(crate) fn host(&self) -> Host::Value<'call>
    where
        Host::Value<'call>: Clone,
    {
        self.context.value.clone()
    }
}

impl<Type, Host> Value<Type, ProviderTransferValueContext<Host>>
where
    Host: HostType,
{
    #[doc(hidden)]
    pub fn from_transfer_host<'call, Profile, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        value: Host::Value<'call>,
    ) -> Self
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        Self {
            context: ProviderTransferValueContext {
                value: call.retain_value::<Host>(value),
                host: PhantomData,
            },
            type_: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn into_transfer_host<'call, Profile, Provider, Return>(
        self,
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
    ) -> Host::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        call.restore_value::<Host>(&self.context.value)
    }

    pub(crate) fn stored(&self) -> &StoredRuntimeValue<TransferValues> {
        &self.context.value
    }

    pub(crate) fn from_stored(value: StoredRuntimeValue<TransferValues>) -> Self {
        Self {
            context: ProviderTransferValueContext {
                value,
                host: PhantomData,
            },
            type_: PhantomData,
        }
    }

    pub(crate) fn into_stored(self) -> StoredRuntimeValue<TransferValues> {
        self.context.value
    }
}

#[cfg(test)]
mod tests {
    use super::{ProviderValueContext, Value};
    use crate::host::{HostTypeParameter, HostValue, HostValueFamily, HostValueToken};

    #[test]
    fn provider_value_preserves_the_exact_call_scoped_host_handle() {
        type Parameter = HostTypeParameter<0>;
        let host = HostValue::<Parameter>::new(HostValueToken {
            family: HostValueFamily::String,
            index: 4,
        });
        let value = Value::<Parameter, ProviderValueContext<'_, Parameter>>::from_host(host);

        assert_eq!(value.host().token, host.token);
        assert_eq!(value.into_host().token, host.token);
    }
}
