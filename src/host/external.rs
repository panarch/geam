mod dynamic;
mod store;
mod stored;

use crate::host::{HostProfile, HostProvider, HostTypeListEnd};
use ecow::EcoString;
use std::marker::PhantomData;

pub use dynamic::HostStoredDynamic;
pub(crate) use dynamic::HostStoredValueFamily;
pub(crate) use store::ExternalPayloadLease;
pub use store::HostExternalStore;
pub use stored::{
    HostExternalPayloadBuilder, HostExternalPayloadView, HostStoredType, HostStoredValue,
};

/// A source-declared external Gleam type linked to Rust storage.
pub trait HostExternalSchema: Send + Sync + 'static {
    const PACKAGE: &'static str;
    const MODULE: &'static str;
    const NAME: &'static str;
    const PARAMETER_COUNT: usize;
}

/// Provider-owned storage and Gleam source semantics for one external schema.
///
/// Payloads are immutable after creation. Values that compare equal through
/// [`HostExternalStorage::source_equal`] must return the same
/// [`HostExternalStorage::source_hash`]. Hash collisions are allowed and are
/// resolved through source equality. Source hashes are runtime indexes, not
/// stable serialized values.
pub trait HostExternalStorage<Profile, Schema>: Send + Sync + 'static
where
    Profile: HostProfile,
    Schema: HostExternalSchema,
{
    type Payload: 'static;

    /// Projects the typed payload store from the final profile's external stores.
    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload>;

    /// Compares two payloads using Gleam source equality.
    fn source_equal(
        context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool;

    /// Hashes a payload consistently with [`HostExternalStorage::source_equal`].
    fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64;

    /// Produces the payload's canonical source-oriented inspection.
    fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString;
}

/// Selects a provider-owned external storage adapter for one source schema.
pub trait HostExternalBinding<Profile, Schema>: HostProvider<Profile>
where
    Profile: HostProfile,
    Schema: HostExternalSchema,
{
    type Storage: HostExternalStorage<Profile, Schema>;
}

/// Gleam source equality for values retained by an external payload.
pub struct HostExternalEquality<'context> {
    equal: &'context dyn Fn(
        &crate::runtime::StoredRuntimeValue,
        &crate::runtime::StoredRuntimeValue,
    ) -> bool,
}

/// Gleam source hashing for values retained by an external payload.
pub struct HostExternalHashing<'context> {
    source_hash: &'context dyn Fn(&crate::runtime::StoredRuntimeValue) -> u64,
}

/// Canonical inspection for values retained by an external payload.
pub struct HostExternalInspection<'context> {
    inspect: &'context dyn Fn(&crate::runtime::StoredRuntimeValue) -> EcoString,
}

/// A source-declared external type and its concrete type arguments.
pub struct HostExternalType<Schema, Arguments = HostTypeListEnd>(PhantomData<(Schema, Arguments)>);

impl<'context> HostExternalEquality<'context> {
    pub(crate) fn new(
        equal: &'context dyn Fn(
            &crate::runtime::StoredRuntimeValue,
            &crate::runtime::StoredRuntimeValue,
        ) -> bool,
    ) -> Self {
        Self { equal }
    }

    /// Compares two retained values with their exact sealed host type.
    pub fn stored_values_equal<Type>(
        &self,
        left: &HostStoredValue<Type>,
        right: &HostStoredValue<Type>,
    ) -> bool {
        (self.equal)(&left.value, &right.value)
    }

    /// Compares two existentially retained values using their specialized shapes.
    pub fn dynamic_values_equal(
        &self,
        left: &HostStoredDynamic,
        right: &HostStoredDynamic,
    ) -> bool {
        (self.equal)(left.runtime_value(), right.runtime_value())
    }
}

impl<'context> HostExternalHashing<'context> {
    pub(crate) fn new(
        source_hash: &'context dyn Fn(&crate::runtime::StoredRuntimeValue) -> u64,
    ) -> Self {
        Self { source_hash }
    }

    /// Hashes a retained value with its exact sealed host type.
    pub fn stored_value_hash<Type>(&self, value: &HostStoredValue<Type>) -> u64 {
        (self.source_hash)(&value.value)
    }

    /// Hashes an existentially retained value and its specialized shape.
    pub fn dynamic_value_hash(&self, value: &HostStoredDynamic) -> u64 {
        (self.source_hash)(value.runtime_value())
    }
}

impl<'context> HostExternalInspection<'context> {
    pub(crate) fn new(
        inspect: &'context dyn Fn(&crate::runtime::StoredRuntimeValue) -> EcoString,
    ) -> Self {
        Self { inspect }
    }

    /// Inspects a retained value with its exact sealed host type.
    pub fn inspect_stored_value<Type>(&self, value: &HostStoredValue<Type>) -> EcoString {
        (self.inspect)(&value.value)
    }

    /// Inspects an existentially retained value using its specialized shape.
    pub fn inspect_dynamic_value(&self, value: &HostStoredDynamic) -> EcoString {
        (self.inspect)(value.runtime_value())
    }
}

/// The source-facing identity of one registered external type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostExternalTypeSchema {
    package: EcoString,
    module: EcoString,
    name: EcoString,
    parameter_count: usize,
}

impl HostExternalTypeSchema {
    pub fn of<Schema: HostExternalSchema>() -> Self {
        Self::new(
            Schema::PACKAGE,
            Schema::MODULE,
            Schema::NAME,
            Schema::PARAMETER_COUNT,
        )
    }

    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
        name: impl Into<EcoString>,
        parameter_count: usize,
    ) -> Self {
        Self {
            package: package.into(),
            module: module.into(),
            name: name.into(),
            parameter_count,
        }
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn parameter_count(&self) -> usize {
        self.parameter_count
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostExternalEquality, HostExternalHashing, HostExternalInspection, HostStoredDynamic,
        HostStoredValue,
    };
    use num_bigint::BigInt;

    #[test]
    fn external_semantics_contexts_delegate_typed_and_dynamic_values() {
        let typed = HostStoredValue::<BigInt>::new(crate::runtime::StoredRuntimeValue::test_int(
            BigInt::from(7),
        ));
        let other = HostStoredValue::<BigInt>::new(crate::runtime::StoredRuntimeValue::test_int(
            BigInt::from(8),
        ));
        let dynamic = HostStoredDynamic::new(crate::runtime::StoredRuntimeValue::test_int(
            BigInt::from(9),
        ));
        let other_dynamic = HostStoredDynamic::new(crate::runtime::StoredRuntimeValue::test_int(
            BigInt::from(10),
        ));
        let equal =
            |_: &crate::runtime::StoredRuntimeValue, _: &crate::runtime::StoredRuntimeValue| true;
        let source_hash = |_: &crate::runtime::StoredRuntimeValue| 17;
        let inspect = |_: &crate::runtime::StoredRuntimeValue| "Int".into();
        let equality = HostExternalEquality::new(&equal);
        let hashing = HostExternalHashing::new(&source_hash);
        let inspection = HostExternalInspection::new(&inspect);

        assert!(equality.stored_values_equal(&typed, &other));
        assert!(equality.dynamic_values_equal(&dynamic, &other_dynamic));
        assert_eq!(hashing.stored_value_hash(&typed), 17);
        assert_eq!(hashing.dynamic_value_hash(&dynamic), 17);
        assert_eq!(inspection.inspect_stored_value(&typed), "Int");
        assert_eq!(inspection.inspect_dynamic_value(&dynamic), "Int");
    }
}

#[cfg(test)]
pub(crate) struct ExternalTestProfile;

#[cfg(test)]
#[derive(Default)]
pub(crate) struct ExternalTestRunState {
    pub(crate) provider: (),
}

#[cfg(test)]
#[derive(Default)]
pub(crate) struct ExternalTestStores {
    pub(crate) units: HostExternalStore<()>,
    pub(crate) integers: HostExternalStore<num_bigint::BigInt>,
    pub(crate) indices: HostExternalStore<usize>,
}

#[cfg(test)]
impl HostProfile for ExternalTestProfile {
    type RunState = ExternalTestRunState;
    type ExternalStores = ExternalTestStores;
}
