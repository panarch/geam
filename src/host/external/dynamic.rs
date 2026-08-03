use super::stored::{HostExternalPayloadBuilder, HostExternalPayloadView};
use crate::host::{HostCall, HostProfile, HostProvider, HostType, HostTypeSequence};

/// An existential Gleam value retained with its exact specialized type.
///
/// Dynamic values belong to external payloads and cannot be moved out of a
/// shared payload view.
///
/// ```compile_fail
/// use geam::HostStoredDynamic;
///
/// struct Payload {
///     value: HostStoredDynamic,
/// }
///
/// fn take(payload: &Payload) -> HostStoredDynamic {
///     payload.value
/// }
/// ```
///
/// They are not ordinary host ABI arguments.
///
/// ```compile_fail
/// use geam::{HostModule, HostStoredDynamic};
/// use num_bigint::BigInt;
///
/// let _ = HostModule::new("host_support", "host/storage")
///     .unwrap()
///     .with_function(
///         "inject",
///         |value: HostStoredDynamic| -> BigInt {
///             let _ = value;
///             0.into()
///         },
///     );
/// ```
pub struct HostStoredDynamic {
    value: crate::runtime::StoredRuntimeValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostStoredValueFamily {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    List,
    Tuple,
    Custom,
    External,
    Function,
}

impl HostStoredDynamic {
    pub(super) fn new(value: crate::runtime::StoredRuntimeValue) -> Self {
        Self { value }
    }

    fn has_type(&self, type_: &crate::plan::ValueType) -> bool {
        self.value.type_() == type_
    }

    pub(crate) fn value_type(&self) -> &crate::plan::ValueType {
        self.value.type_()
    }

    pub(crate) fn value_family(&self) -> HostStoredValueFamily {
        self.value.family()
    }

    pub(super) fn runtime_value(&self) -> &crate::runtime::StoredRuntimeValue {
        &self.value
    }

    fn decode<'call, Profile, Provider, Return, Type>(
        &self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
    ) -> Option<Type::Value<'call>>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
        Type: HostType,
    {
        let requested = call.resolve_host_type::<Type>()?;
        if !self.has_type(&requested) {
            return None;
        }
        Some(call.restore_runtime_value::<Type>(&self.value))
    }
}

impl<Profile, Arguments> HostExternalPayloadBuilder<'_, Profile, Arguments>
where
    Profile: HostProfile,
    Arguments: HostTypeSequence,
{
    /// Retains a typed value for later existential decoding.
    pub fn store_dynamic<Type>(&mut self, value: Type::Value<'_>) -> HostStoredDynamic
    where
        Type: HostType,
    {
        HostStoredDynamic::new(
            self.runtime
                .retain_stored(crate::host::type_::into_scoped::<Type>(value)),
        )
    }
}

impl<'call, Payload, Arguments> HostExternalPayloadView<'call, Payload, Arguments>
where
    Arguments: HostTypeSequence,
{
    /// Decodes a retained value when its exact specialized type matches.
    pub fn decode<Profile, Provider, Return, Type>(
        &self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        select: impl FnOnce(&Payload) -> &HostStoredDynamic,
    ) -> Option<Type::Value<'call>>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
        Type: HostType,
    {
        select(&self.value).decode::<Profile, Provider, Return, Type>(call)
    }
}

#[cfg(test)]
mod tests {
    use super::HostStoredDynamic;
    use crate::plan::ValueType;
    use num_bigint::BigInt;

    #[test]
    fn dynamic_value_matches_only_its_exact_specialized_type() {
        let stored = HostStoredDynamic::new(crate::runtime::StoredRuntimeValue::test_int(
            BigInt::from(7),
        ));

        assert_eq!(stored.value_type(), &ValueType::Int);
        assert!(stored.has_type(&ValueType::Int));
        assert!(!stored.has_type(&ValueType::String));
    }
}
