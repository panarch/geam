use std::collections::HashMap;
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ecow::EcoString;
use parking_lot::ReentrantMutex;

#[cfg(test)]
use super::TransferStoredRuntimeValue;
use crate::host::{TransferExternalEquality, TransferExternalHashing, TransferExternalInspection};
use crate::runtime::RuntimeExternalLease;

pub(crate) struct TransferExternalStore<Payload> {
    values: Arc<Mutex<HashMap<u64, Arc<StoredTransferExternalPayload<Payload>>>>>,
}

pub(crate) struct TransferExternalPayloadLease {
    release: Arc<dyn TransferExternalPayloadRelease + Send + Sync>,
    value: Arc<dyn TransferExternalPayload + Send + Sync>,
}

pub(crate) struct TransferExternalPayloadView<Payload> {
    guard: parking_lot::ArcReentrantMutexGuard<
        parking_lot::RawMutex,
        parking_lot::RawThreadId,
        Payload,
    >,
}

struct StoredTransferExternalPayload<Payload> {
    id: u64,
    value: Arc<ReentrantMutex<Payload>>,
    values: Arc<Mutex<HashMap<u64, Arc<StoredTransferExternalPayload<Payload>>>>>,
    source_equal: for<'context> fn(&TransferExternalEquality<'context>, &Payload, &Payload) -> bool,
    source_hash: for<'context> fn(&TransferExternalHashing<'context>, &Payload) -> u64,
    inspect: for<'context> fn(&TransferExternalInspection<'context>, &Payload) -> EcoString,
    source_hash_cache: OnceLock<u64>,
    inspection_cache: OnceLock<EcoString>,
}

struct TransferExternalPayloadReleaseGuard<Payload> {
    id: u64,
    values: Arc<Mutex<HashMap<u64, Arc<StoredTransferExternalPayload<Payload>>>>>,
}

trait TransferExternalPayload {
    fn id(&self) -> u64;
    fn source_hash(&self, context: &TransferExternalHashing<'_>) -> u64;
    fn inspection(&self, context: &TransferExternalInspection<'_>) -> EcoString;
    fn source_equal(
        &self,
        context: &TransferExternalEquality<'_>,
        other: &TransferExternalPayloadLease,
    ) -> bool;
}

trait TransferExternalPayloadRelease {}

impl<Payload> Default for TransferExternalStore<Payload> {
    fn default() -> Self {
        Self {
            values: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<Payload> TransferExternalStore<Payload>
where
    Payload: Send + 'static,
{
    pub(crate) fn clone_handle(&self) -> Self {
        Self {
            values: Arc::clone(&self.values),
        }
    }

    pub(crate) fn insert(
        &self,
        value: Payload,
        source_equal: for<'context> fn(
            &TransferExternalEquality<'context>,
            &Payload,
            &Payload,
        ) -> bool,
        source_hash: for<'context> fn(&TransferExternalHashing<'context>, &Payload) -> u64,
        inspect: for<'context> fn(&TransferExternalInspection<'context>, &Payload) -> EcoString,
    ) -> TransferExternalPayloadLease {
        let id = crate::runtime::ExternalValueIdentity::allocate_id();
        let value = Arc::new(StoredTransferExternalPayload {
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
        TransferExternalPayloadLease {
            release: Arc::new(TransferExternalPayloadReleaseGuard {
                id,
                values: Arc::clone(&self.values),
            }),
            value,
        }
    }

    pub(crate) fn with_view<Output>(
        &self,
        lease: &TransferExternalPayloadLease,
        view: impl FnOnce(&Payload) -> Output,
    ) -> Output {
        let value = Arc::clone(&lock(&self.values)[&lease.identity()]);
        view(&value.value.lock())
    }

    pub(crate) fn view(
        &self,
        lease: &TransferExternalPayloadLease,
    ) -> TransferExternalPayloadView<Payload> {
        let value = Arc::clone(&lock(&self.values)[&lease.identity()]);
        TransferExternalPayloadView {
            guard: value.value.lock_arc(),
        }
    }
}

impl<Payload> std::ops::Deref for TransferExternalPayloadView<Payload> {
    type Target = Payload;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl TransferExternalPayloadLease {
    pub(in crate::runtime) fn source_hash(&self, context: &TransferExternalHashing<'_>) -> u64 {
        self.value.source_hash(context)
    }

    pub(in crate::runtime) fn inspection(
        &self,
        context: &TransferExternalInspection<'_>,
    ) -> EcoString {
        self.value.inspection(context)
    }

    pub(in crate::runtime) fn source_equal(
        &self,
        context: &TransferExternalEquality<'_>,
        other: &Self,
    ) -> bool {
        self.value.source_equal(context, other)
    }
}

impl Clone for TransferExternalPayloadLease {
    fn clone(&self) -> Self {
        Self {
            release: Arc::clone(&self.release),
            value: Arc::clone(&self.value),
        }
    }
}

impl std::fmt::Debug for TransferExternalPayloadLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TransferExternalPayloadLease")
            .field("identity", &self.identity())
            .finish()
    }
}

impl RuntimeExternalLease for TransferExternalPayloadLease {
    fn identity(&self) -> u64 {
        self.value.id()
    }
}

impl<Payload> TransferExternalPayload for StoredTransferExternalPayload<Payload>
where
    Payload: Send + 'static,
{
    fn id(&self) -> u64 {
        self.id
    }

    fn source_hash(&self, context: &TransferExternalHashing<'_>) -> u64 {
        *self
            .source_hash_cache
            .get_or_init(|| (self.source_hash)(context, &self.value.lock()))
    }

    fn inspection(&self, context: &TransferExternalInspection<'_>) -> EcoString {
        self.inspection_cache
            .get_or_init(|| (self.inspect)(context, &self.value.lock()))
            .clone()
    }

    fn source_equal(
        &self,
        context: &TransferExternalEquality<'_>,
        other: &TransferExternalPayloadLease,
    ) -> bool {
        if self.id == other.identity() {
            let value = self.value.lock();
            return (self.source_equal)(context, &value, &value);
        }
        let Some(other) = lock(&self.values).get(&other.identity()).cloned() else {
            return false;
        };
        if self.id < other.id {
            let left = self.value.lock();
            let right = other.value.lock();
            (self.source_equal)(context, &left, &right)
        } else {
            let right = other.value.lock();
            let left = self.value.lock();
            (self.source_equal)(context, &left, &right)
        }
    }
}

impl<Payload> TransferExternalPayloadRelease for TransferExternalPayloadReleaseGuard<Payload> {}

impl<Payload> Drop for TransferExternalPayloadReleaseGuard<Payload> {
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
    use super::{
        TransferExternalEquality, TransferExternalHashing, TransferExternalInspection,
        TransferExternalPayloadLease, TransferExternalStore, TransferStoredRuntimeValue, lock,
    };
    use crate::runtime::{EvaluatedValue, RuntimeExternalLease, TransferValues};
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
        context: &TransferExternalEquality<'_>,
        left: &SendOnlyPayload,
        right: &SendOnlyPayload,
    ) -> bool {
        left.equal_calls.set(left.equal_calls.get() + 1);
        let left = TransferStoredRuntimeValue::new(EvaluatedValue::Int(left.value.get().into()));
        let right = TransferStoredRuntimeValue::new(EvaluatedValue::Int(right.value.get().into()));
        context.stored_values_equal(&left, &right)
    }

    fn source_hash(context: &TransferExternalHashing<'_>, value: &SendOnlyPayload) -> u64 {
        value.hash_calls.set(value.hash_calls.get() + 1);
        let stored = TransferStoredRuntimeValue::new(EvaluatedValue::Int(value.value.get().into()));
        context.stored_value_hash(&stored)
    }

    fn inspect(context: &TransferExternalInspection<'_>, value: &SendOnlyPayload) -> EcoString {
        value.inspect_calls.set(value.inspect_calls.get() + 1);
        let stored = TransferStoredRuntimeValue::new(EvaluatedValue::Int(value.value.get().into()));
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
        assert_send::<TransferExternalStore<SendOnlyPayload>>();
        assert_send::<TransferExternalPayloadLease>();
        assert_send::<EvaluatedValue<TransferValues>>();

        let store = TransferExternalStore::default();
        let drops = Arc::new(AtomicUsize::new(0));
        let lease = store.insert(payload(7, Arc::clone(&drops)), equal, source_hash, inspect);
        assert_eq!(store.with_view(&lease, |value| value.value.get()), 7);
        assert_eq!(drops.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn escaped_lease_preserves_source_semantics_and_releases_once() {
        let store = TransferExternalStore::default();
        let drops = Arc::new(AtomicUsize::new(0));
        let first = store.insert(payload(7, Arc::clone(&drops)), equal, source_hash, inspect);
        let first_clone = first.clone();
        let second = store.insert(payload(7, Arc::clone(&drops)), equal, source_hash, inspect);
        let other_store = TransferExternalStore::default();
        let foreign =
            other_store.insert(payload(7, Arc::clone(&drops)), equal, source_hash, inspect);
        let stored_equal = |left: &TransferStoredRuntimeValue,
                            right: &TransferStoredRuntimeValue| {
            left.value() == right.value()
        };
        let equality = TransferExternalEquality::new(&stored_equal);
        let stored_hash = |_: &TransferStoredRuntimeValue| 7;
        let hashing = TransferExternalHashing::new(&stored_hash);
        let stored_inspect = |_: &TransferStoredRuntimeValue| "7".into();
        let inspection = TransferExternalInspection::new(&stored_inspect);

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
        assert!(format!("{first:?}").contains("TransferExternalPayloadLease"));

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
        let store = TransferExternalStore::default();
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

        let store = TransferExternalStore::default();
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
        let store = TransferExternalStore::default();
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
        let store = TransferExternalStore::default();
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
