use crate::host::HostCall;
use crate::runtime::StoredRuntimeValue;
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

/// Owned retained value used by a transferable provider invocation.
#[doc(hidden)]
pub struct ProviderValueContext<Host>
where
    Host: HostType,
{
    value: StoredRuntimeValue,
    host: PhantomData<fn() -> Host>,
}

impl<Type, Host> Value<Type, ProviderValueContext<Host>>
where
    Host: HostType,
{
    #[doc(hidden)]
    pub fn from_host<'call, Profile, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        value: Host::Value<'call>,
    ) -> Self
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        Self {
            context: ProviderValueContext {
                value: call.retain_value::<Host>(value),
                host: PhantomData,
            },
            type_: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn into_host<'call, Profile, Provider, Return>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> Host::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
    {
        call.restore_value::<Host>(&self.context.value)
    }

    pub(crate) fn stored(&self) -> &StoredRuntimeValue {
        &self.context.value
    }

    pub(crate) fn from_stored(value: StoredRuntimeValue) -> Self {
        Self {
            context: ProviderValueContext {
                value,
                host: PhantomData,
            },
            type_: PhantomData,
        }
    }

    pub(crate) fn into_stored(self) -> StoredRuntimeValue {
        self.context.value
    }
}

#[cfg(test)]
mod tests {
    use super::{ProviderValueContext, Value};
    use crate::host::HostTypeParameter;
    use crate::runtime::{BorrowedValue, StoredRuntimeValue};

    #[test]
    fn provider_value_preserves_its_owned_value_through_retention() {
        type Parameter = HostTypeParameter<0>;
        let value = Value::<Parameter, ProviderValueContext<Parameter>>::from_stored(
            StoredRuntimeValue::test_int(42.into()),
        );
        assert_eq!(BorrowedValue::from_stored(value.stored()).int(), &42.into());
        assert_eq!(
            BorrowedValue::from_stored(&value.into_stored()).int(),
            &42.into()
        );
    }
}
