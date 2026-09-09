use parking_lot::ReentrantMutex;
use std::sync::Arc;

/// Shared immutable ownership with short, serialized access to Send-only data.
pub(crate) struct Shared<Value> {
    value: Arc<ReentrantMutex<Value>>,
}

impl<Value> Shared<Value> {
    pub(crate) fn new(value: Value) -> Self {
        Self {
            value: Arc::new(ReentrantMutex::new(value)),
        }
    }

    pub(crate) fn read<Output>(&self, read: impl FnOnce(&Value) -> Output) -> Output {
        read(&self.value.lock())
    }
}

impl<Value> Clone for Shared<Value> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Shared;
    use std::cell::Cell;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Payload {
        reads: Cell<usize>,
        drops: Arc<AtomicUsize>,
    }

    impl Drop for Payload {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn aliases_share_a_non_clone_send_only_payload_and_allow_nested_reads() {
        let drops = Arc::new(AtomicUsize::new(0));
        let value = Shared::new(Payload {
            reads: Cell::new(0),
            drops: Arc::clone(&drops),
        });
        let alias = value.clone();
        value.read(|first| {
            alias.read(|second| assert!(std::ptr::eq(first, second)));
        });
        std::thread::scope(|threads| {
            for _ in 0..4 {
                let alias = alias.clone();
                threads.spawn(move || {
                    for _ in 0..100 {
                        alias.read(|value| value.reads.set(value.reads.get() + 1));
                    }
                });
            }
        });
        assert_eq!(value.read(|value| value.reads.get()), 400);
        drop(value);
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(alias);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn a_read_panic_does_not_poison_or_replace_the_owned_value() {
        let value = Shared::new(Cell::new(7));
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            value.read(|_| panic!("caller panic"));
        }));
        assert!(failed.is_err());
        assert_eq!(value.read(Cell::get), 7);
    }
}
