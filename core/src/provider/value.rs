use crate::HostType;
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
