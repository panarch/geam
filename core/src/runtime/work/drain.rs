use parking_lot::Mutex;
use std::collections::VecDeque;

pub(super) struct DrainQueue<Value> {
    state: Mutex<State<Value>>,
}

struct State<Value> {
    draining: bool,
    queued: VecDeque<Value>,
}

struct ResetOnUnwind<'queue, Value> {
    queue: &'queue DrainQueue<Value>,
    armed: bool,
}

impl<Value> DrainQueue<Value> {
    pub(super) fn new() -> Self {
        Self {
            state: Mutex::new(State {
                draining: false,
                queued: VecDeque::new(),
            }),
        }
    }

    // Nested wake/drop callbacks enqueue work for the current draining caller.
    // No callback runs under the queue lock, and no background worker exists.
    pub(super) fn deliver(
        &self,
        values: impl IntoIterator<Item = Value>,
        mut consume: impl FnMut(Value),
    ) {
        {
            let mut state = self.state.lock();
            state.queued.extend(values);
            if state.draining {
                return;
            }
            state.draining = true;
        }
        let mut reset = ResetOnUnwind {
            queue: self,
            armed: true,
        };
        loop {
            let value = {
                let mut state = self.state.lock();
                match state.queued.pop_front() {
                    Some(value) => value,
                    None => {
                        state.draining = false;
                        reset.armed = false;
                        return;
                    }
                }
            };
            consume(value);
        }
    }
}

impl<Value> Drop for ResetOnUnwind<'_, Value> {
    fn drop(&mut self) {
        if self.armed {
            let queued = {
                let mut state = self.queue.state.lock();
                state.draining = false;
                std::mem::take(&mut state.queued)
            };
            drop(queued);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DrainQueue;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};

    #[test]
    fn reentrant_delivery_is_iterative_and_ordered() {
        fn advance(queue: &DrainQueue<usize>, next: &AtomicUsize, value: usize) {
            assert_eq!(next.fetch_add(1, Ordering::SeqCst), value);
            if value < 20_000 {
                queue.deliver([value + 1], |value| advance(queue, next, value));
            }
        }
        let queue = DrainQueue::new();
        let next = AtomicUsize::new(0);
        queue.deliver([0], |value| advance(&queue, &next, value));
        assert_eq!(next.load(Ordering::SeqCst), 20_001);
        assert!(!queue.state.lock().draining);
        assert!(queue.state.lock().queued.is_empty());
        let queue = DrainQueue::new();
        let next = AtomicUsize::new(0);
        advance(&queue, &next, 0);
        assert_eq!(next.load(Ordering::SeqCst), 20_001);
    }

    #[test]
    fn concurrent_delivery_does_not_hold_the_queue_lock_across_a_callback() {
        let queue = Arc::new(DrainQueue::new());
        let entered = Arc::new(Barrier::new(2));
        let release = Arc::new(Barrier::new(2));
        let total = Arc::new(AtomicUsize::new(0));
        std::thread::scope(|threads| {
            let consume = |value| {
                if value == 1 {
                    entered.wait();
                    release.wait();
                }
                total.fetch_add(value, Ordering::SeqCst);
            };
            let worker_queue = &queue;
            let worker = threads.spawn(move || {
                worker_queue.deliver([1], consume);
            });
            entered.wait();
            queue.deliver([2], consume);
            assert_eq!(total.load(Ordering::SeqCst), 0);
            release.wait();
            worker.join().expect("queue worker");
        });
        assert_eq!(total.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn panicking_delivery_releases_queued_values_and_a_later_drain_can_start() {
        struct CountDrop(Arc<AtomicUsize>);
        impl Drop for CountDrop {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let queue = DrainQueue::new();
        let drops = Arc::new(AtomicUsize::new(0));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            queue.deliver([CountDrop(Arc::clone(&drops))], |value| {
                queue.deliver([CountDrop(Arc::clone(&drops))], drop);
                drop(value);
                panic!("native callback unwinds");
            });
        }));
        assert!(result.is_err());
        assert_eq!(drops.load(Ordering::SeqCst), 2);
        queue.deliver([CountDrop(Arc::clone(&drops))], drop);
        assert_eq!(drops.load(Ordering::SeqCst), 3);
    }
}
