use crate::host::{HostExternalEquality, HostExternalHashing, HostExternalInspection};
use ecow::EcoString;
use std::cell::{OnceCell, RefCell};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_EXTERNAL_VALUE_ID: AtomicU64 = AtomicU64::new(0);

pub struct HostExternalStore<Payload> {
    values: Rc<RefCell<HashMap<u64, Rc<StoredExternalPayload<Payload>>>>>,
}

pub(crate) struct ExternalPayloadLease {
    release: Rc<dyn ExternalPayloadRelease>,
    value: Rc<dyn ExternalPayload>,
}

pub(crate) struct ExternalPayloadView<Payload> {
    value: Rc<StoredExternalPayload<Payload>>,
}

struct StoredExternalPayload<Payload> {
    id: u64,
    value: Payload,
    values: Weak<RefCell<HashMap<u64, Rc<StoredExternalPayload<Payload>>>>>,
    source_equal: for<'context> fn(&HostExternalEquality<'context>, &Payload, &Payload) -> bool,
    source_hash: for<'context> fn(&HostExternalHashing<'context>, &Payload) -> u64,
    inspect: for<'context> fn(&HostExternalInspection<'context>, &Payload) -> EcoString,
    source_hash_cache: OnceCell<u64>,
    inspection_cache: OnceCell<EcoString>,
}

struct ExternalPayloadReleaseGuard<Payload> {
    id: u64,
    values: Weak<RefCell<HashMap<u64, Rc<StoredExternalPayload<Payload>>>>>,
}

trait ExternalPayload {
    fn id(&self) -> u64;
    fn source_hash(&self, context: &HostExternalHashing<'_>) -> u64;
    fn inspection<'payload>(
        &'payload self,
        context: &HostExternalInspection<'_>,
    ) -> &'payload EcoString;
    fn source_equal(
        &self,
        context: &HostExternalEquality<'_>,
        other: &ExternalPayloadLease,
    ) -> bool;
}

trait ExternalPayloadRelease {}

impl<Payload> Default for HostExternalStore<Payload> {
    fn default() -> Self {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl<Payload> HostExternalStore<Payload>
where
    Payload: 'static,
{
    pub(crate) fn clone_handle(&self) -> Self {
        Self {
            values: Rc::clone(&self.values),
        }
    }

    pub(crate) fn insert(
        &self,
        value: Payload,
        source_equal: for<'context> fn(&HostExternalEquality<'context>, &Payload, &Payload) -> bool,
        source_hash: for<'context> fn(&HostExternalHashing<'context>, &Payload) -> u64,
        inspect: for<'context> fn(&HostExternalInspection<'context>, &Payload) -> EcoString,
    ) -> ExternalPayloadLease {
        let id = NEXT_EXTERNAL_VALUE_ID.fetch_add(1, Ordering::Relaxed);
        let value = Rc::new(StoredExternalPayload {
            id,
            value,
            values: Rc::downgrade(&self.values),
            source_equal,
            source_hash,
            inspect,
            source_hash_cache: OnceCell::new(),
            inspection_cache: OnceCell::new(),
        });
        self.values.borrow_mut().insert(id, Rc::clone(&value));
        ExternalPayloadLease {
            release: Rc::new(ExternalPayloadReleaseGuard {
                id,
                values: Rc::downgrade(&self.values),
            }),
            value,
        }
    }

    pub(crate) fn view(&self, lease: &ExternalPayloadLease) -> ExternalPayloadView<Payload> {
        let value = Rc::clone(&self.values.borrow()[&lease.id()]);
        ExternalPayloadView { value }
    }
}

impl ExternalPayloadLease {
    pub(crate) fn id(&self) -> u64 {
        self.value.id()
    }

    pub(crate) fn source_hash(&self, context: &HostExternalHashing<'_>) -> u64 {
        self.value.source_hash(context)
    }

    pub(crate) fn inspection(&self, context: &HostExternalInspection<'_>) -> &EcoString {
        self.value.inspection(context)
    }

    pub(crate) fn source_equal(&self, context: &HostExternalEquality<'_>, other: &Self) -> bool {
        self.value.source_equal(context, other)
    }
}

impl Clone for ExternalPayloadLease {
    fn clone(&self) -> Self {
        Self {
            release: Rc::clone(&self.release),
            value: Rc::clone(&self.value),
        }
    }
}

impl<Payload> Deref for ExternalPayloadView<Payload> {
    type Target = Payload;

    fn deref(&self) -> &Self::Target {
        &self.value.value
    }
}

impl<Payload> ExternalPayload for StoredExternalPayload<Payload>
where
    Payload: 'static,
{
    fn id(&self) -> u64 {
        self.id
    }

    fn source_hash(&self, context: &HostExternalHashing<'_>) -> u64 {
        *self
            .source_hash_cache
            .get_or_init(|| (self.source_hash)(context, &self.value))
    }

    fn inspection<'payload>(
        &'payload self,
        context: &HostExternalInspection<'_>,
    ) -> &'payload EcoString {
        self.inspection_cache
            .get_or_init(|| (self.inspect)(context, &self.value))
    }

    fn source_equal(
        &self,
        context: &HostExternalEquality<'_>,
        other: &ExternalPayloadLease,
    ) -> bool {
        if self.id == other.id() {
            return (self.source_equal)(context, &self.value, &self.value);
        }
        let Some(values) = self.values.upgrade() else {
            return false;
        };
        values
            .borrow()
            .get(&other.id())
            .is_some_and(|other| (self.source_equal)(context, &self.value, &other.value))
    }
}

impl<Payload> ExternalPayloadRelease for ExternalPayloadReleaseGuard<Payload> {}

impl<Payload> Drop for ExternalPayloadReleaseGuard<Payload> {
    fn drop(&mut self) {
        if let Some(values) = self.values.upgrade() {
            values.borrow_mut().remove(&self.id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HostExternalStore;
    use crate::host::{HostExternalHashing, HostExternalInspection};
    use ecow::EcoString;
    use std::cell::Cell;
    use std::rc::Rc;

    struct Payload {
        value: usize,
        drops: Rc<Cell<usize>>,
        hashes: Rc<Cell<usize>>,
        inspections: Rc<Cell<usize>>,
    }

    impl Drop for Payload {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
        }
    }

    fn equal(_: &crate::host::HostExternalEquality<'_>, left: &Payload, right: &Payload) -> bool {
        left.value == right.value
    }

    fn source_hash(_: &HostExternalHashing<'_>, value: &Payload) -> u64 {
        value.hashes.set(value.hashes.get() + 1);
        value.value as u64
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Payload) -> EcoString {
        value.inspections.set(value.inspections.get() + 1);
        let stored = crate::host::HostStoredValue::<num_bigint::BigInt>::new(
            crate::runtime::StoredRuntimeValue::test_int(value.value.into()),
        );
        format!("Payload({})", context.inspect_stored_value(&stored)).into()
    }

    fn payload(value: usize, drops: &Rc<Cell<usize>>) -> Payload {
        Payload {
            value,
            drops: Rc::clone(drops),
            hashes: Rc::new(Cell::new(0)),
            inspections: Rc::new(Cell::new(0)),
        }
    }

    #[test]
    fn lease_controls_typed_index_and_payload_lifetime() {
        let drops = Rc::new(Cell::new(0));
        let store = HostExternalStore::default();
        let lease = store.insert(payload(7, &drops), equal, source_hash, inspect);
        let clone = lease.clone();

        assert_eq!(store.values.borrow().len(), 1);
        assert_eq!((*store.view(&lease)).value, 7);
        drop(lease);
        assert_eq!(store.values.borrow().len(), 1);
        assert_eq!(drops.get(), 0);

        drop(clone);
        assert!(store.values.borrow().is_empty());
        assert_eq!(drops.get(), 1);
    }

    #[test]
    fn escaped_lease_remains_self_contained_after_store_drop() {
        let drops = Rc::new(Cell::new(0));
        let store = HostExternalStore::default();
        let first_payload = payload(7, &drops);
        let first_hashes = Rc::clone(&first_payload.hashes);
        let first_inspections = Rc::clone(&first_payload.inspections);
        let first = store.insert(first_payload, equal, source_hash, inspect);
        let second = store.insert(payload(7, &drops), equal, source_hash, inspect);

        let stored_equal =
            |_: &crate::runtime::StoredRuntimeValue, _: &crate::runtime::StoredRuntimeValue| false;
        let equality = crate::host::HostExternalEquality::new(&stored_equal);
        let stored_hash = |_: &crate::runtime::StoredRuntimeValue| 17;
        let stored_inspect = |_: &crate::runtime::StoredRuntimeValue| EcoString::from("7");
        let hashing = HostExternalHashing::new(&stored_hash);
        let inspection = HostExternalInspection::new(&stored_inspect);

        assert!(first.source_equal(&equality, &first));
        assert!(first.source_equal(&equality, &second));
        assert_eq!(first_hashes.get(), 0);
        assert_eq!(first_inspections.get(), 0);
        assert_eq!(first.source_hash(&hashing), 7);
        assert_eq!(first.source_hash(&hashing), 7);
        assert_eq!(first.inspection(&inspection), "Payload(7)");
        assert_eq!(first.inspection(&inspection), "Payload(7)");
        assert_eq!(first_hashes.get(), 1);
        assert_eq!(first_inspections.get(), 1);

        drop(store);

        assert!(!first.source_equal(&equality, &second));
        assert_eq!(first.inspection(&inspection), "Payload(7)");
        drop(first);
        drop(second);
        assert_eq!(drops.get(), 2);
    }

    #[test]
    fn source_equality_does_not_cross_typed_store_instances() {
        let drops = Rc::new(Cell::new(0));
        let first_store = HostExternalStore::default();
        let second_store = HostExternalStore::default();
        let first = first_store.insert(payload(7, &drops), equal, source_hash, inspect);
        let second = second_store.insert(payload(7, &drops), equal, source_hash, inspect);

        let stored_equal =
            |_: &crate::runtime::StoredRuntimeValue, _: &crate::runtime::StoredRuntimeValue| false;
        let equality = crate::host::HostExternalEquality::new(&stored_equal);

        assert!(!first.source_equal(&equality, &second));
    }

    #[test]
    fn source_equality_does_not_assume_opaque_identity_is_reflexive() {
        fn source_hash(
            context: &HostExternalHashing<'_>,
            value: &crate::host::HostStoredValue<num_bigint::BigInt>,
        ) -> u64 {
            context.stored_value_hash(value)
        }

        fn inspect(
            context: &HostExternalInspection<'_>,
            value: &crate::host::HostStoredValue<num_bigint::BigInt>,
        ) -> EcoString {
            context.inspect_stored_value(value)
        }

        let store = HostExternalStore::default();
        let lease = store.insert(
            crate::host::HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int(7.into()),
            ),
            |context, left, right| context.stored_values_equal(left, right),
            source_hash,
            inspect,
        );
        let stored_equal =
            |_: &crate::runtime::StoredRuntimeValue, _: &crate::runtime::StoredRuntimeValue| false;
        let equality = crate::host::HostExternalEquality::new(&stored_equal);
        let stored_hash = |_: &crate::runtime::StoredRuntimeValue| 7;
        let stored_inspect = |_: &crate::runtime::StoredRuntimeValue| EcoString::from("7");

        assert!(!lease.source_equal(&equality, &lease));
        assert_eq!(
            lease.source_hash(&HostExternalHashing::new(&stored_hash)),
            7
        );
        assert_eq!(
            lease.inspection(&HostExternalInspection::new(&stored_inspect)),
            "7",
        );
    }
}
