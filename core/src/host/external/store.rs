use std::collections::HashMap;
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ecow::EcoString;
use parking_lot::ReentrantMutex;

use crate::host::{HostExternalEquality, HostExternalHashing, HostExternalInspection};
use crate::host::{RetainedValueEquality, RetainedValueHashing, RetainedValueInspection};

pub struct HostExternalStore<Payload> {
    values: Arc<Mutex<HashMap<u64, Arc<StoredExternalPayload<Payload>>>>>,
}

pub(crate) struct ExternalPayloadLease {
    release: Arc<dyn ExternalPayloadRelease + Send + Sync>,
    value: Arc<dyn ExternalPayload + Send + Sync>,
}

pub(crate) struct ExternalPayloadView<Payload> {
    guard: parking_lot::ArcReentrantMutexGuard<
        parking_lot::RawMutex,
        parking_lot::RawThreadId,
        Payload,
    >,
}

struct StoredExternalPayload<Payload> {
    id: u64,
    value: Arc<ReentrantMutex<Payload>>,
    values: Arc<Mutex<HashMap<u64, Arc<StoredExternalPayload<Payload>>>>>,
    source_equal: for<'context> fn(&HostExternalEquality<'context>, &Payload, &Payload) -> bool,
    source_hash: for<'context> fn(&HostExternalHashing<'context>, &Payload) -> u64,
    inspect: for<'context> fn(&HostExternalInspection<'context>, &Payload) -> EcoString,
    source_hash_cache: OnceLock<u64>,
    inspection_cache: OnceLock<EcoString>,
}

struct ExternalPayloadReleaseGuard<Payload> {
    id: u64,
    values: Arc<Mutex<HashMap<u64, Arc<StoredExternalPayload<Payload>>>>>,
}

trait ExternalPayload {
    fn id(&self) -> u64;
    fn source_hash(&self, context: &RetainedValueHashing<'_>) -> u64;
    fn inspection(&self, context: &RetainedValueInspection<'_>) -> EcoString;
    fn source_equal(
        &self,
        context: &RetainedValueEquality<'_>,
        other: &ExternalPayloadLease,
    ) -> bool;
}

trait ExternalPayloadRelease {}

impl<Payload> Default for HostExternalStore<Payload> {
    fn default() -> Self {
        Self {
            values: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<Payload> HostExternalStore<Payload>
where
    Payload: Send + 'static,
{
    pub(crate) fn clone_handle(&self) -> Self {
        Self {
            values: Arc::clone(&self.values),
        }
    }

    pub(crate) fn insert_with_storage<Profile, Schema, Storage>(
        &self,
        value: Payload,
    ) -> ExternalPayloadLease
    where
        Profile: crate::HostProfile,
        Schema: crate::HostExternalSchema,
        Storage: crate::HostExternalStorage<Profile, Schema, Payload = Payload>,
    {
        self.insert(
            value,
            Storage::source_equal,
            Storage::source_hash,
            Storage::inspect,
        )
    }

    pub(crate) fn insert(
        &self,
        value: Payload,
        source_equal: for<'context> fn(&HostExternalEquality<'context>, &Payload, &Payload) -> bool,
        source_hash: for<'context> fn(&HostExternalHashing<'context>, &Payload) -> u64,
        inspect: for<'context> fn(&HostExternalInspection<'context>, &Payload) -> EcoString,
    ) -> ExternalPayloadLease {
        let id = crate::runtime::ExternalValueIdentity::allocate_id();
        let value = Arc::new(StoredExternalPayload {
            id,
            value: Arc::new(ReentrantMutex::new(value)),
            values: Arc::clone(&self.values),
            source_equal,
            source_hash,
            inspect,
            source_hash_cache: OnceLock::new(),
            inspection_cache: OnceLock::new(),
        });
        lock(&self.values).insert(id, Arc::clone(&value));
        ExternalPayloadLease {
            release: Arc::new(ExternalPayloadReleaseGuard {
                id,
                values: Arc::clone(&self.values),
            }),
            value,
        }
    }

    pub(crate) fn with_view<Output>(
        &self,
        lease: &ExternalPayloadLease,
        view: impl FnOnce(&Payload) -> Output,
    ) -> Output {
        let value = Arc::clone(&lock(&self.values)[&lease.identity()]);
        view(&value.value.lock())
    }

    pub(crate) fn view(&self, lease: &ExternalPayloadLease) -> ExternalPayloadView<Payload> {
        let value = Arc::clone(&lock(&self.values)[&lease.identity()]);
        ExternalPayloadView {
            guard: value.value.lock_arc(),
        }
    }
}

impl<Payload> std::ops::Deref for ExternalPayloadView<Payload> {
    type Target = Payload;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl ExternalPayloadLease {
    pub(crate) fn identity(&self) -> u64 {
        self.value.id()
    }

    pub(crate) fn source_hash(&self, context: &RetainedValueHashing<'_>) -> u64 {
        self.value.source_hash(context)
    }

    pub(crate) fn inspection(&self, context: &RetainedValueInspection<'_>) -> EcoString {
        self.value.inspection(context)
    }

    pub(crate) fn source_equal(&self, context: &RetainedValueEquality<'_>, other: &Self) -> bool {
        self.value.source_equal(context, other)
    }
}

impl Clone for ExternalPayloadLease {
    fn clone(&self) -> Self {
        Self {
            release: Arc::clone(&self.release),
            value: Arc::clone(&self.value),
        }
    }
}

impl std::fmt::Debug for ExternalPayloadLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ExternalPayloadLease")
            .field("identity", &self.identity())
            .finish()
    }
}

impl<Payload> ExternalPayload for StoredExternalPayload<Payload>
where
    Payload: Send + 'static,
{
    fn id(&self) -> u64 {
        self.id
    }

    fn source_hash(&self, context: &RetainedValueHashing<'_>) -> u64 {
        *self
            .source_hash_cache
            .get_or_init(|| (self.source_hash)(&HostExternalHashing(context), &self.value.lock()))
    }

    fn inspection(&self, context: &RetainedValueInspection<'_>) -> EcoString {
        self.inspection_cache
            .get_or_init(|| (self.inspect)(&HostExternalInspection(context), &self.value.lock()))
            .clone()
    }

    fn source_equal(
        &self,
        context: &RetainedValueEquality<'_>,
        other: &ExternalPayloadLease,
    ) -> bool {
        if self.id == other.identity() {
            let value = self.value.lock();
            return (self.source_equal)(&HostExternalEquality(context), &value, &value);
        }
        let Some(other) = lock(&self.values).get(&other.identity()).cloned() else {
            return false;
        };
        if self.id < other.id {
            let left = self.value.lock();
            let right = other.value.lock();
            (self.source_equal)(&HostExternalEquality(context), &left, &right)
        } else {
            let right = other.value.lock();
            let left = self.value.lock();
            (self.source_equal)(&HostExternalEquality(context), &left, &right)
        }
    }
}

impl<Payload> ExternalPayloadRelease for ExternalPayloadReleaseGuard<Payload> {}

impl<Payload> Drop for ExternalPayloadReleaseGuard<Payload> {
    fn drop(&mut self) {
        lock(&self.values).remove(&self.id);
    }
}

fn lock<Value>(mutex: &Mutex<Value>) -> MutexGuard<'_, Value> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
mod tests {
    use super::HostExternalStore;
    use super::lock;
    use crate::host::{HostExternalHashing, HostExternalInspection};
    use ecow::EcoString;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    struct Payload {
        value: usize,
        drops: Arc<AtomicUsize>,
        hashes: Arc<AtomicUsize>,
        inspections: Arc<AtomicUsize>,
    }

    impl Drop for Payload {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn equal(_: &crate::host::HostExternalEquality<'_>, left: &Payload, right: &Payload) -> bool {
        left.value == right.value
    }

    fn source_hash(_: &HostExternalHashing<'_>, value: &Payload) -> u64 {
        value.hashes.fetch_add(1, Ordering::SeqCst);
        value.value as u64
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &Payload) -> EcoString {
        value.inspections.fetch_add(1, Ordering::SeqCst);
        let stored = crate::host::HostStoredValue::<num_bigint::BigInt>::new(
            crate::runtime::StoredRuntimeValue::test_int(value.value.into()),
        );
        format!("Payload({})", context.inspect_stored_value(&stored)).into()
    }

    fn payload(value: usize, drops: &Arc<AtomicUsize>) -> Payload {
        Payload {
            value,
            drops: Arc::clone(drops),
            hashes: Arc::new(AtomicUsize::new(0)),
            inspections: Arc::new(AtomicUsize::new(0)),
        }
    }

    #[test]
    fn lease_controls_typed_index_and_payload_lifetime() {
        let drops = Arc::new(AtomicUsize::new(0));
        let store = HostExternalStore::default();
        let lease = store.insert(payload(7, &drops), equal, source_hash, inspect);
        let clone = lease.clone();

        assert_eq!(lock(&store.values).len(), 1);
        assert_eq!(store.view(&lease).value, 7);
        drop(lease);
        assert_eq!(lock(&store.values).len(), 1);
        assert_eq!(drops.load(Ordering::SeqCst), 0);

        drop(clone);
        assert!(lock(&store.values).is_empty());
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn escaped_lease_remains_self_contained_after_store_drop() {
        let drops = Arc::new(AtomicUsize::new(0));
        let store = HostExternalStore::default();
        let first_payload = payload(7, &drops);
        let first_hashes = Arc::clone(&first_payload.hashes);
        let first_inspections = Arc::clone(&first_payload.inspections);
        let first = store.insert(first_payload, equal, source_hash, inspect);
        let second = store.insert(payload(7, &drops), equal, source_hash, inspect);

        let stored_equal =
            |_: &crate::runtime::RetainedValueRef, _: &crate::runtime::RetainedValueRef| false;
        let equality = crate::host::RetainedValueEquality::new(&stored_equal);
        let stored_hash = |_: &crate::runtime::RetainedValueRef| 17;
        let stored_inspect = |_: &crate::runtime::RetainedValueRef| EcoString::from("7");
        let hashing = crate::host::RetainedValueHashing::new(&stored_hash);
        let inspection = crate::host::RetainedValueInspection::new(&stored_inspect);

        assert!(first.source_equal(&equality, &first));
        assert!(first.source_equal(&equality, &second));
        assert_eq!(first_hashes.load(Ordering::SeqCst), 0);
        assert_eq!(first_inspections.load(Ordering::SeqCst), 0);
        assert_eq!(first.source_hash(&hashing), 7);
        assert_eq!(first.source_hash(&hashing), 7);
        assert_eq!(first.inspection(&inspection), "Payload(7)");
        assert_eq!(first.inspection(&inspection), "Payload(7)");
        assert_eq!(first_hashes.load(Ordering::SeqCst), 1);
        assert_eq!(first_inspections.load(Ordering::SeqCst), 1);

        drop(store);

        assert!(first.source_equal(&equality, &second));
        assert_eq!(first.inspection(&inspection), "Payload(7)");
        drop(first);
        drop(second);
        assert_eq!(drops.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn source_equality_does_not_cross_typed_store_instances() {
        let drops = Arc::new(AtomicUsize::new(0));
        let first_store = HostExternalStore::default();
        let second_store = HostExternalStore::default();
        let first = first_store.insert(payload(7, &drops), equal, source_hash, inspect);
        let second = second_store.insert(payload(7, &drops), equal, source_hash, inspect);

        let stored_equal =
            |_: &crate::runtime::RetainedValueRef, _: &crate::runtime::RetainedValueRef| false;
        let equality = crate::host::RetainedValueEquality::new(&stored_equal);

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
            |_: &crate::runtime::RetainedValueRef, _: &crate::runtime::RetainedValueRef| false;
        let equality = crate::host::RetainedValueEquality::new(&stored_equal);
        let stored_hash = |_: &crate::runtime::RetainedValueRef| 7;
        let stored_inspect = |_: &crate::runtime::RetainedValueRef| EcoString::from("7");

        assert!(!lease.source_equal(&equality, &lease));
        assert_eq!(
            lease.source_hash(&crate::host::RetainedValueHashing::new(&stored_hash)),
            7
        );
        assert_eq!(
            lease.inspection(&crate::host::RetainedValueInspection::new(&stored_inspect)),
            "7",
        );
    }
}

#[cfg(test)]
mod transfer_tests {
    use super::{ExternalPayloadLease, HostExternalStore, lock};
    use crate::host::{HostExternalEquality, HostExternalHashing, HostExternalInspection};
    use crate::runtime::RetainedValueRef;
    use ecow::EcoString;
    use std::cell::Cell;
    use std::future::{Future, poll_fn};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Waker};

    struct SendOnlyPayload {
        value: Cell<usize>,
        equal_calls: Cell<usize>,
        hash_calls: Cell<usize>,
        inspect_calls: Cell<usize>,
        drops: Arc<AtomicUsize>,
    }

    impl Drop for SendOnlyPayload {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn equal(
        context: &HostExternalEquality<'_>,
        left: &SendOnlyPayload,
        right: &SendOnlyPayload,
    ) -> bool {
        left.equal_calls.set(left.equal_calls.get() + 1);
        let left = crate::host::HostStoredValue::<num_bigint::BigInt>::new(
            crate::runtime::StoredRuntimeValue::test_int(left.value.get().into()),
        );
        let right = crate::host::HostStoredValue::<num_bigint::BigInt>::new(
            crate::runtime::StoredRuntimeValue::test_int(right.value.get().into()),
        );
        context.stored_values_equal(&left, &right)
    }

    fn source_hash(context: &HostExternalHashing<'_>, value: &SendOnlyPayload) -> u64 {
        value.hash_calls.set(value.hash_calls.get() + 1);
        let stored = crate::host::HostStoredValue::<num_bigint::BigInt>::new(
            crate::runtime::StoredRuntimeValue::test_int(value.value.get().into()),
        );
        context.stored_value_hash(&stored)
    }

    fn inspect(context: &HostExternalInspection<'_>, value: &SendOnlyPayload) -> EcoString {
        value.inspect_calls.set(value.inspect_calls.get() + 1);
        let stored = crate::host::HostStoredValue::<num_bigint::BigInt>::new(
            crate::runtime::StoredRuntimeValue::test_int(value.value.get().into()),
        );
        format!("Resource({})", context.inspect_stored_value(&stored)).into()
    }

    fn payload(value: usize, drops: Arc<AtomicUsize>) -> SendOnlyPayload {
        SendOnlyPayload {
            value: Cell::new(value),
            equal_calls: Cell::new(0),
            hash_calls: Cell::new(0),
            inspect_calls: Cell::new(0),
            drops,
        }
    }

    fn assert_send<Value: Send>() {}

    #[test]
    fn transfer_external_lease_accepts_send_only_payloads() {
        assert_send::<HostExternalStore<SendOnlyPayload>>();
        assert_send::<ExternalPayloadLease>();

        let store = HostExternalStore::default();
        let drops = Arc::new(AtomicUsize::new(0));
        let lease = store.insert(payload(7, Arc::clone(&drops)), equal, source_hash, inspect);
        assert_eq!(store.with_view(&lease, |value| value.value.get()), 7);
        assert_eq!(drops.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn escaped_lease_preserves_source_semantics_and_releases_once() {
        let store = HostExternalStore::default();
        let drops = Arc::new(AtomicUsize::new(0));
        let first = store.insert(payload(7, Arc::clone(&drops)), equal, source_hash, inspect);
        let first_clone = first.clone();
        let second = store.insert(payload(7, Arc::clone(&drops)), equal, source_hash, inspect);
        let other_store = HostExternalStore::default();
        let foreign =
            other_store.insert(payload(7, Arc::clone(&drops)), equal, source_hash, inspect);
        let stored_equal = |_: &RetainedValueRef, _: &RetainedValueRef| true;
        let equality = crate::host::RetainedValueEquality::new(&stored_equal);
        let stored_hash = |_: &RetainedValueRef| 7;
        let hashing = crate::host::RetainedValueHashing::new(&stored_hash);
        let stored_inspect = |_: &RetainedValueRef| "7".into();
        let inspection = crate::host::RetainedValueInspection::new(&stored_inspect);

        assert!(first.source_equal(&equality, &first));
        assert!(first.source_equal(&equality, &second));
        assert!(second.source_equal(&equality, &first));
        assert!(!first.source_equal(&equality, &foreign));
        assert_ne!(first.identity(), second.identity());
        assert_eq!(first.source_hash(&hashing), 7);
        assert_eq!(first.source_hash(&hashing), 7);
        assert_eq!(first.inspection(&inspection), "Resource(7)");
        assert_eq!(first.inspection(&inspection), "Resource(7)");
        assert_eq!(store.with_view(&first, |value| value.equal_calls.get()), 2);
        assert_eq!(store.with_view(&first, |value| value.hash_calls.get()), 1);
        assert_eq!(
            store.with_view(&first, |value| value.inspect_calls.get()),
            1
        );
        assert!(format!("{first:?}").contains("ExternalPayloadLease"));

        drop(store);
        assert_eq!(first.source_hash(&hashing), 7);
        assert!(first.source_equal(&equality, &second));
        drop(first);
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        drop(first_clone);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        drop(second);
        drop(foreign);
        assert_eq!(drops.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn pending_owner_preserves_payload_and_cancellation_releases_it_once() {
        let drops = Arc::new(AtomicUsize::new(0));
        let store = HostExternalStore::default();
        let lease = store.insert(payload(7, Arc::clone(&drops)), equal, source_hash, inspect);
        let mut first_poll = true;
        let mut pending_owner = Box::pin(async move {
            poll_fn(move |context| {
                if first_poll {
                    first_poll = false;
                    context.waker().wake_by_ref();
                    Poll::Pending
                } else {
                    Poll::Ready(())
                }
            })
            .await;
            drop(lease);
        });
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);

        assert_eq!(pending_owner.as_mut().poll(&mut context), Poll::Pending);
        drop(store);
        assert_eq!(pending_owner.as_mut().poll(&mut context), Poll::Ready(()));
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        drop(pending_owner);

        let store = HostExternalStore::default();
        let lease = store.insert(payload(8, Arc::clone(&drops)), equal, source_hash, inspect);
        let mut cancelled_owner = Box::pin(poll_fn(move |_| {
            let _lease = &lease;
            Poll::<()>::Pending
        }));
        assert_eq!(cancelled_owner.as_mut().poll(&mut context), Poll::Pending);
        drop(store);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        drop(cancelled_owner);
        assert_eq!(drops.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn unwinding_a_payload_view_releases_its_lock_without_a_runtime_error() {
        let store = HostExternalStore::default();
        let lease = store.insert(
            payload(7, Arc::new(AtomicUsize::new(0))),
            equal,
            source_hash,
            inspect,
        );
        let payload = Arc::clone(&lock(&store.values)[&lease.identity()]);
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = payload.value.lock();
            panic!("poison transfer payload");
        }));

        assert_eq!(store.with_view(&lease, |value| value.value.get()), 7);
    }

    #[test]
    fn nested_semantic_reads_can_revisit_a_shared_send_only_payload() {
        let store = HostExternalStore::default();
        let drops = Arc::new(AtomicUsize::new(0));
        let first = store.insert(payload(7, Arc::clone(&drops)), equal, source_hash, inspect);
        let second = store.insert(payload(8, Arc::clone(&drops)), equal, source_hash, inspect);

        let values = store.with_view(&first, |first_value| {
            store.with_view(&second, |second_value| {
                store.with_view(&first, |shared_value| {
                    (
                        first_value.value.get(),
                        second_value.value.get(),
                        shared_value.value.get(),
                    )
                })
            })
        });

        assert_eq!(values, (7, 8, 7));
        drop(first);
        drop(second);
        assert_eq!(drops.load(Ordering::Relaxed), 2);
    }
}
