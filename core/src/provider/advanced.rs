use crate::host::{
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostStoredType,
    HostStoredValue, HostTypeIndex0, HostTypeIndexNext,
};
use crate::provider::ProviderStoredOwner;
use ecow::EcoString;
use std::marker::PhantomData;

/// Source-equality access for retained values in one immutable payload.
pub type Equality<'value> = HostExternalEquality<'value>;

/// Source-hash access for retained values in one immutable payload.
pub type Hashing<'value> = HostExternalHashing<'value>;

/// Source-inspection access for retained values in one immutable payload.
pub type Inspection<'value> = HostExternalInspection<'value>;

/// One retained source value owned by an advanced external payload.
///
/// The payload type is the owner brand. The argument index identifies the
/// corresponding source type parameter. Values are created only by
/// [`super::Call::store`] followed by the generated external boundary.
pub struct Retained<Owner, Index> {
    value: HostStoredValue<HostStoredType<Index>>,
    owner: PhantomData<fn() -> Owner>,
}

/// The first source type-argument position of an advanced external payload.
pub type Index0 = HostTypeIndex0;

/// The source type-argument position after `Index`.
pub type Next<Index> = HostTypeIndexNext<Index>;

/// Context-aware source semantics for an advanced retained payload.
pub trait RetainedExternalPayload: 'static {
    fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool;

    fn source_hash(&self, context: &Hashing<'_>) -> u64;

    fn inspect(&self, context: &Inspection<'_>) -> EcoString;
}

impl<Owner, Index> Retained<Owner, Index>
where
    Owner: ProviderStoredOwner,
{
    pub(crate) fn new(value: HostStoredValue<HostStoredType<Index>>) -> Self {
        Self {
            value,
            owner: PhantomData,
        }
    }

    pub(crate) fn host(&self) -> &HostStoredValue<HostStoredType<Index>> {
        &self.value
    }

    /// Compares two retained values with Gleam source equality.
    pub fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
        context.stored_values_equal(&self.value, &other.value)
    }

    /// Hashes this retained value consistently with Gleam source equality.
    pub fn source_hash(&self, context: &Hashing<'_>) -> u64 {
        context.stored_value_hash(&self.value)
    }

    /// Inspects this retained value with Gleam source formatting.
    pub fn inspect(&self, context: &Inspection<'_>) -> EcoString {
        context.inspect_stored_value(&self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Index0, Retained};
    use crate::host::{
        HostExternalEquality, HostExternalHashing, HostExternalInspection, HostStoredType,
        HostStoredValue,
    };
    use crate::runtime::StoredRuntimeValue;

    struct Payload;

    impl crate::provider::ProviderStoredOwner for Payload {}

    fn retained(value: i64) -> Retained<Payload, Index0> {
        Retained::new(HostStoredValue::<HostStoredType<Index0>>::new(
            StoredRuntimeValue::test_int(value.into()),
        ))
    }

    #[test]
    fn retained_values_delegate_each_source_operation_to_its_narrow_context() {
        let first = retained(7);
        let different = retained(8);
        let stored_equal =
            |left: &StoredRuntimeValue, right: &StoredRuntimeValue| std::ptr::eq(left, right);
        let stored_hash = |_: &StoredRuntimeValue| 17;
        let stored_inspect = |_: &StoredRuntimeValue| "Int(7)".into();
        let equality = HostExternalEquality::new(&stored_equal);
        let hashing = HostExternalHashing::new(&stored_hash);
        let inspection = HostExternalInspection::new(&stored_inspect);

        assert!(first.source_equal(&equality, &first));
        assert!(!first.source_equal(&equality, &different));
        assert_eq!(first.source_hash(&hashing), 17);
        assert_eq!(first.inspect(&inspection), "Int(7)");
    }
}
