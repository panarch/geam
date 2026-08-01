use ecow::EcoString;
use std::cell::RefCell;
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
    equal: fn(&Payload, &Payload) -> bool,
    inspect: fn(&Payload) -> EcoString,
}

struct ExternalPayloadReleaseGuard<Payload> {
    id: u64,
    values: Weak<RefCell<HashMap<u64, Rc<StoredExternalPayload<Payload>>>>>,
}

trait ExternalPayload {
    fn id(&self) -> u64;
    fn inspect(&self) -> EcoString;
    fn source_equal(&self, other: &ExternalPayloadLease) -> bool;
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
    pub(crate) fn insert(
        &self,
        value: Payload,
        equal: fn(&Payload, &Payload) -> bool,
        inspect: fn(&Payload) -> EcoString,
    ) -> ExternalPayloadLease {
        let id = NEXT_EXTERNAL_VALUE_ID.fetch_add(1, Ordering::Relaxed);
        let value = Rc::new(StoredExternalPayload {
            id,
            value,
            values: Rc::downgrade(&self.values),
            equal,
            inspect,
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

    pub(crate) fn inspect(&self) -> EcoString {
        self.value.inspect()
    }

    pub(crate) fn source_equal(&self, other: &Self) -> bool {
        self.value.source_equal(other)
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

    fn inspect(&self) -> EcoString {
        (self.inspect)(&self.value)
    }

    fn source_equal(&self, other: &ExternalPayloadLease) -> bool {
        if self.id == other.id() {
            return true;
        }
        let Some(values) = self.values.upgrade() else {
            return false;
        };
        values
            .borrow()
            .get(&other.id())
            .is_some_and(|other| (self.equal)(&self.value, &other.value))
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
    use ecow::EcoString;
    use std::cell::Cell;
    use std::rc::Rc;

    struct Payload {
        value: usize,
        drops: Rc<Cell<usize>>,
    }

    impl Drop for Payload {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
        }
    }

    fn equal(left: &Payload, right: &Payload) -> bool {
        left.value == right.value
    }

    fn inspect(value: &Payload) -> EcoString {
        format!("Payload({})", value.value).into()
    }

    #[test]
    fn lease_controls_typed_index_and_payload_lifetime() {
        let drops = Rc::new(Cell::new(0));
        let store = HostExternalStore::default();
        let lease = store.insert(
            Payload {
                value: 7,
                drops: Rc::clone(&drops),
            },
            equal,
            inspect,
        );
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
        let first = store.insert(
            Payload {
                value: 7,
                drops: Rc::clone(&drops),
            },
            equal,
            inspect,
        );
        let second = store.insert(
            Payload {
                value: 7,
                drops: Rc::clone(&drops),
            },
            equal,
            inspect,
        );

        assert!(first.source_equal(&first));
        assert!(first.source_equal(&second));
        assert_eq!(first.inspect(), "Payload(7)");

        drop(store);

        assert!(!first.source_equal(&second));
        assert_eq!(first.inspect(), "Payload(7)");
        drop(first);
        drop(second);
        assert_eq!(drops.get(), 2);
    }

    #[test]
    fn source_equality_does_not_cross_typed_store_instances() {
        let drops = Rc::new(Cell::new(0));
        let first_store = HostExternalStore::default();
        let second_store = HostExternalStore::default();
        let first = first_store.insert(
            Payload {
                value: 7,
                drops: Rc::clone(&drops),
            },
            equal,
            inspect,
        );
        let second = second_store.insert(
            Payload {
                value: 7,
                drops: Rc::clone(&drops),
            },
            equal,
            inspect,
        );

        assert!(!first.source_equal(&second));
    }
}
