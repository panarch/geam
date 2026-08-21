use super::store::ExternalPayloadView;
use crate::host::type_::HostTypeAt;
use crate::host::{
    HostCall, HostCallRuntime, HostProfile, HostProvider, HostType, HostTypeSequence,
};
use std::marker::PhantomData;
use std::ops::Deref;

/// A typed Gleam value retained by one external payload.
///
/// Stored values are created by [`HostExternalPayloadBuilder`] and restored
/// through [`HostExternalPayloadView`]. They cannot be cloned or moved out of
/// the shared payload view.
///
/// ```compile_fail
/// use geam_core::HostStoredValue;
/// use num_bigint::BigInt;
///
/// struct Payload {
///     value: HostStoredValue<BigInt>,
/// }
///
/// fn take(payload: &Payload) -> HostStoredValue<BigInt> {
///     payload.value
/// }
/// ```
///
/// ```compile_fail
/// use geam_core::{HostModule, HostStoredValue};
/// use num_bigint::BigInt;
///
/// let _ = HostModule::new("host_support", "host/storage")
///     .unwrap()
///     .with_function(
///         "inject",
///         |value: HostStoredValue<BigInt>| -> BigInt {
///             let _ = value;
///             0.into()
///         },
///     );
/// ```
pub struct HostStoredValue<Type> {
    pub(crate) value: crate::runtime::StoredRuntimeValue,
    marker: PhantomData<fn() -> Type>,
}

/// A stored value selected by a stable external type-argument position.
pub struct HostStoredType<Index>(PhantomData<Index>);

/// The active-call builder used to retain Gleam values in a new payload.
pub struct HostExternalPayloadBuilder<'runtime, Profile, Arguments>
where
    Profile: HostProfile,
    Arguments: HostTypeSequence,
{
    pub(super) runtime: &'runtime mut dyn HostCallRuntime<Profile>,
    arguments: PhantomData<Arguments>,
}

/// An immutable external payload view that can restore its retained values.
///
/// ```compile_fail
/// use geam_core::{HostExternalPayloadView, HostTypeListEnd};
///
/// fn escape<'call, Payload>(
///     view: HostExternalPayloadView<'call, Payload, HostTypeListEnd>,
/// ) -> HostExternalPayloadView<'static, Payload, HostTypeListEnd> {
///     view
/// }
/// ```
pub struct HostExternalPayloadView<'call, Payload, Arguments>
where
    Arguments: HostTypeSequence,
{
    pub(super) value: ExternalPayloadView<Payload>,
    call: PhantomData<&'call Arguments>,
}

impl<Type> HostStoredValue<Type> {
    pub(crate) fn new(value: crate::runtime::StoredRuntimeValue) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }
}

impl<'runtime, Profile, Arguments> HostExternalPayloadBuilder<'runtime, Profile, Arguments>
where
    Profile: HostProfile,
    Arguments: HostTypeSequence,
{
    pub(crate) fn new(runtime: &'runtime mut dyn HostCallRuntime<Profile>) -> Self {
        Self {
            runtime,
            arguments: PhantomData,
        }
    }

    /// Retains a value with one exact monomorphic host type.
    pub fn store<Type>(&mut self, value: Type::Value<'_>) -> HostStoredValue<Type>
    where
        Type: HostType,
    {
        HostStoredValue::new(
            self.runtime
                .retain_stored(crate::host::type_::into_scoped::<Type>(value)),
        )
    }

    /// Retains a generic value by its stable external type-argument position.
    pub fn store_argument<Index>(
        &mut self,
        value: <<Arguments as HostTypeAt<Index>>::Type as HostType>::Value<'_>,
    ) -> HostStoredValue<HostStoredType<Index>>
    where
        Arguments: HostTypeAt<Index>,
    {
        HostStoredValue::new(
            self.runtime
                .retain_stored(crate::host::type_::into_scoped::<
                    <Arguments as HostTypeAt<Index>>::Type,
                >(value)),
        )
    }
}

impl<'call, Payload, Arguments> HostExternalPayloadView<'call, Payload, Arguments>
where
    Arguments: HostTypeSequence,
{
    pub(crate) fn new(value: ExternalPayloadView<Payload>) -> Self {
        Self {
            value,
            call: PhantomData,
        }
    }

    /// Restores one monomorphic value selected from this payload.
    pub fn restore<Profile, Provider, Return, Type>(
        &self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        select: impl FnOnce(&Payload) -> &HostStoredValue<Type>,
    ) -> Type::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
        Type: HostType,
    {
        call.restore_stored::<Type, Type>(select(&self.value))
    }

    /// Restores one value selected by its external type-argument position.
    pub fn restore_argument<Profile, Provider, Return, Index>(
        &self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        select: impl FnOnce(&Payload) -> &HostStoredValue<HostStoredType<Index>>,
    ) -> <<Arguments as HostTypeAt<Index>>::Type as HostType>::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
        Arguments: HostTypeAt<Index>,
    {
        call.restore_stored::<<Arguments as HostTypeAt<Index>>::Type, _>(select(&self.value))
    }
}

impl<Payload, Arguments> Deref for HostExternalPayloadView<'_, Payload, Arguments>
where
    Arguments: HostTypeSequence,
{
    type Target = Payload;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
