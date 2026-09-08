use super::shared::Shared;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Weak};
use std::task::{Context, Poll, Wake, Waker};

mod callback;
mod composition;
mod dependency;
mod drain;
pub(crate) mod driver;
pub(crate) mod execution;
mod request;

pub(crate) use dependency::Dependencies;
use drain::DrainQueue;

pub(crate) struct WorkScope<Value> {
    registry: Arc<Mutex<Registry<Value>>>,
    releases: Arc<DrainQueue<Phase<Value>>>,
    notifications: Arc<DrainQueue<Arc<Waker>>>,
}

pub(crate) struct WorkFactory<Value> {
    registry: Weak<Mutex<Registry<Value>>>,
    releases: Arc<DrainQueue<Phase<Value>>>,
    notifications: Arc<DrainQueue<Arc<Waker>>>,
}

pub(crate) struct Work<Value> {
    operation: Arc<Operation<Value>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Cancelled;

pub(crate) struct Observer<Value> {
    work: Work<Value>,
    ticket: u64,
}

struct Registry<Value> {
    accepting: bool,
    operations: BTreeMap<u64, Weak<Operation<Value>>>,
}

struct Operation<Value> {
    identity: u64,
    registry: Weak<Mutex<Registry<Value>>>,
    phase: Mutex<Phase<Value>>,
    wake: Arc<Observers>,
    dependencies: Dependencies<Value>,
    releases: Arc<DrainQueue<Phase<Value>>>,
}

enum Phase<Value> {
    Idle(Pin<Box<dyn Future<Output = Result<Shared<Value>, Cancelled>> + Send>>),
    Polling,
    Complete(Shared<Value>),
    Cancelled,
}

struct CancelUnfinishedPoll<'operation, Value> {
    operation: &'operation Operation<Value>,
    armed: bool,
}

struct Observers {
    next: AtomicU64,
    generation: AtomicU64,
    waiters: Mutex<BTreeMap<u64, Arc<Waker>>>,
    notified: AtomicBool,
    deliveries: Arc<DrainQueue<Arc<Waker>>>,
}

impl<Value> WorkScope<Value> {
    pub(crate) fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(Registry {
                accepting: true,
                operations: BTreeMap::new(),
            })),
            releases: Arc::new(DrainQueue::new()),
            notifications: Arc::new(DrainQueue::new()),
        }
    }

    pub(crate) fn factory(&self) -> WorkFactory<Value> {
        WorkFactory {
            registry: Arc::downgrade(&self.registry),
            releases: Arc::clone(&self.releases),
            notifications: Arc::clone(&self.notifications),
        }
    }
}

impl<Value> Drop for WorkScope<Value> {
    fn drop(&mut self) {
        let operations = {
            let mut registry = self.registry.lock();
            registry.accepting = false;
            std::mem::take(&mut registry.operations)
        };
        for operation in operations.into_values().filter_map(|value| value.upgrade()) {
            operation.cancel();
        }
    }
}

impl<Value> WorkFactory<Value> {
    #[cfg(test)]
    pub(crate) fn create(
        &self,
        native: impl Future<Output = Value> + Send + 'static,
    ) -> Work<Value> {
        self.compose(|_| async move { Ok(Shared::new(native.await)) })
    }

    pub(crate) fn ready(&self, value: Value) -> Work<Value> {
        self.register(Phase::Complete(Shared::new(value)), Dependencies::new())
    }

    fn compose<Native>(&self, build: impl FnOnce(Dependencies<Value>) -> Native) -> Work<Value>
    where
        Native: Future<Output = Result<Shared<Value>, Cancelled>> + Send + 'static,
    {
        let dependencies = Dependencies::new();
        let native = build(dependencies.clone());
        self.register(Phase::Idle(Box::pin(native)), dependencies)
    }

    fn register(&self, phase: Phase<Value>, dependencies: Dependencies<Value>) -> Work<Value> {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let operation = Arc::new(Operation {
            identity: NEXT.fetch_add(1, Ordering::Relaxed),
            registry: self.registry.clone(),
            phase: Mutex::new(phase),
            wake: Arc::new(Observers::new(Arc::clone(&self.notifications))),
            dependencies,
            releases: Arc::clone(&self.releases),
        });
        let registered = self.registry.upgrade().is_some_and(|registry| {
            let mut registry = registry.lock();
            if registry.accepting {
                registry
                    .operations
                    .insert(operation.identity, Arc::downgrade(&operation));
            }
            registry.accepting
        });
        if !registered {
            let phase = std::mem::replace(&mut *operation.phase.lock(), Phase::Cancelled);
            drop(phase);
        }
        Work { operation }
    }
}

impl<Value> Clone for WorkFactory<Value> {
    fn clone(&self) -> Self {
        Self {
            registry: self.registry.clone(),
            releases: Arc::clone(&self.releases),
            notifications: Arc::clone(&self.notifications),
        }
    }
}

impl<Value> Work<Value> {
    pub(crate) fn observe(&self) -> Observer<Value> {
        Observer {
            work: self.clone(),
            ticket: self.operation.wake.next.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub(crate) fn identity(&self) -> u64 {
        self.operation.identity
    }
}

impl<Value> Clone for Work<Value> {
    fn clone(&self) -> Self {
        Self {
            operation: Arc::clone(&self.operation),
        }
    }
}

impl<Value> Future for Observer<Value> {
    type Output = Result<Shared<Value>, Cancelled>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().poll_with(cx, || false)
    }
}

impl<Value> Observer<Value> {
    fn poll_with(
        &mut self,
        cx: &mut Context<'_>,
        service: impl FnMut() -> bool,
    ) -> Poll<Result<Shared<Value>, Cancelled>> {
        let operation = &self.work.operation;
        operation.wake.register(self.ticket, cx.waker().clone());
        let result = operation.poll_graph(service);
        if result.is_ready() {
            operation.wake.remove(self.ticket);
        }
        result
    }
}

impl<Value> Drop for Observer<Value> {
    fn drop(&mut self) {
        self.work.operation.wake.remove(self.ticket);
    }
}

impl<Value> Operation<Value> {
    fn completed(&self) -> Poll<Result<Shared<Value>, Cancelled>> {
        match &*self.phase.lock() {
            Phase::Complete(value) => Poll::Ready(Ok(value.clone())),
            Phase::Cancelled => Poll::Ready(Err(Cancelled)),
            Phase::Idle(_) | Phase::Polling => Poll::Pending,
        }
    }

    fn poll(&self) -> Poll<Result<Shared<Value>, Cancelled>> {
        self.wake.notified.store(false, Ordering::Release);
        let generation = self.wake.generation.load(Ordering::Acquire);
        let mut native = {
            let mut phase = self.phase.lock();
            match std::mem::replace(&mut *phase, Phase::Polling) {
                Phase::Polling => return Poll::Pending,
                Phase::Complete(value) => {
                    *phase = Phase::Complete(value.clone());
                    return Poll::Ready(Ok(value));
                }
                Phase::Cancelled => {
                    *phase = Phase::Cancelled;
                    return Poll::Ready(Err(Cancelled));
                }
                Phase::Idle(native) => native,
            }
        };
        let mut cancellation = CancelUnfinishedPoll {
            operation: self,
            armed: true,
        };
        let waker = Waker::from(Arc::clone(&self.wake));
        let (next, result) = match native.as_mut().poll(&mut Context::from_waker(&waker)) {
            Poll::Ready(Ok(value)) => (Phase::Complete(value.clone()), Poll::Ready(Ok(value))),
            Poll::Ready(Err(Cancelled)) => (Phase::Cancelled, Poll::Ready(Err(Cancelled))),
            Poll::Pending => (Phase::Idle(native), Poll::Pending),
        };
        let replaced = {
            let mut phase = self.phase.lock();
            cancellation.armed = false;
            if matches!(*phase, Phase::Cancelled) {
                None
            } else {
                Some(std::mem::replace(&mut *phase, next))
            }
        };
        if replaced.is_none() {
            return Poll::Ready(Err(Cancelled));
        }
        drop(replaced);
        if result.is_ready() {
            self.wake.finish();
        } else if self.wake.generation.load(Ordering::Acquire) != generation {
            // Re-deliver a wake racing the exclusive poll only after restoring Idle.
            self.wake.notified.store(false, Ordering::Release);
            self.wake.notify();
        }
        result
    }

    fn cancel(&self) {
        let previous = {
            let mut phase = self.phase.lock();
            match &*phase {
                Phase::Complete(_) | Phase::Cancelled => return,
                Phase::Idle(_) | Phase::Polling => std::mem::replace(&mut *phase, Phase::Cancelled),
            }
        };
        self.releases.deliver([previous], drop);
        self.wake.finish();
    }
}

impl<Value> Drop for Operation<Value> {
    fn drop(&mut self) {
        if let Some(registry) = self.registry.upgrade() {
            registry.lock().operations.remove(&self.identity);
        }
        let phase = std::mem::replace(self.phase.get_mut(), Phase::Cancelled);
        self.releases.deliver([phase], drop);
    }
}

impl<Value> Drop for CancelUnfinishedPoll<'_, Value> {
    fn drop(&mut self) {
        if self.armed {
            self.operation.cancel();
        }
    }
}

impl Observers {
    fn new(deliveries: Arc<DrainQueue<Arc<Waker>>>) -> Self {
        Self {
            next: AtomicU64::new(0),
            generation: AtomicU64::new(0),
            waiters: Mutex::new(BTreeMap::new()),
            notified: AtomicBool::new(false),
            deliveries,
        }
    }

    fn register(&self, ticket: u64, waker: Waker) {
        self.notified.store(false, Ordering::Release);
        let previous = self.waiters.lock().insert(ticket, Arc::new(waker));
        drop(previous);
    }

    fn remove(&self, ticket: u64) {
        let previous = self.waiters.lock().remove(&ticket);
        drop(previous);
    }

    fn notify(&self) {
        self.generation.fetch_add(1, Ordering::AcqRel);
        if self.notified.swap(true, Ordering::AcqRel) {
            return;
        }
        let waiters: Vec<_> = self.waiters.lock().values().map(Arc::clone).collect();
        self.deliveries
            .deliver(waiters, |waiter| waiter.wake_by_ref());
    }

    fn finish(&self) {
        self.generation.fetch_add(1, Ordering::AcqRel);
        let waiters = std::mem::take(&mut *self.waiters.lock());
        self.deliveries
            .deliver(waiters.into_values(), |waiter| waiter.wake_by_ref());
    }
}

impl Default for Observers {
    fn default() -> Self {
        Self::new(Arc::new(DrainQueue::new()))
    }
}

impl Wake for Observers {
    fn wake(self: Arc<Self>) {
        self.notify();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::{Cancelled, WorkScope};
    use parking_lot::Mutex;
    use std::cell::Cell;
    use std::future::{Future, poll_fn};
    use std::pin::Pin;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::task::{Context, Poll, Wake, Waker};

    #[derive(Default)]
    struct Gate {
        open: AtomicBool,
        polls: AtomicUsize,
        waker: Mutex<Option<Waker>>,
    }

    impl Gate {
        fn open(&self) {
            self.open.store(true, Ordering::Release);
            let waker = self.waker.lock().take();
            if let Some(waker) = waker {
                waker.wake();
            }
        }

        async fn wait(&self) {
            poll_fn(|cx| {
                self.polls.fetch_add(1, Ordering::Relaxed);
                let mut waker = self.waker.lock();
                if self.open.load(Ordering::Acquire) {
                    Poll::Ready(())
                } else {
                    *waker = Some(cx.waker().clone());
                    Poll::Pending
                }
            })
            .await;
        }
    }

    struct Input(Arc<AtomicUsize>);

    struct Completion(Cell<i32>);

    impl Completion {
        fn new(value: i32) -> Self {
            Self(Cell::new(value))
        }

        fn get(&self) -> i32 {
            self.0.get()
        }
    }

    struct HeldInput<Inner> {
        _input: Input,
        inner: Inner,
    }

    impl<Inner: Future + Unpin> Future for HeldInput<Inner> {
        type Output = Inner::Output;
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            Pin::new(&mut self.inner).poll(cx)
        }
    }

    impl Drop for Input {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[derive(Default)]
    struct Notifications(AtomicUsize);

    impl Wake for Notifications {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn deep_finite_work_chains_poll_wake_and_release_without_recursive_stack_growth() {
        let scope = WorkScope::new();
        let gate = Arc::new(Gate::default());
        let drops = Arc::new(AtomicUsize::new(0));
        let mut work = scope.factory().create({
            let gate = Arc::clone(&gate);
            let input = Input(Arc::clone(&drops));
            async move {
                gate.wait().await;
                drop(input);
                42
            }
        });
        for _ in 0..20_000 {
            work = scope
                .factory()
                .compose(|dependencies| async move { dependencies.observe(&work).await });
        }
        let wake = Arc::new(Notifications::default());
        let waker = Waker::from(Arc::clone(&wake));
        let mut cx = Context::from_waker(&waker);
        assert!(Pin::new(&mut work.observe()).poll(&mut cx).is_pending());
        gate.open();
        let completion = Pin::new(&mut work.observe()).poll(&mut cx);
        assert_eq!(
            completion.map(|value| value.expect("finite work completed").read(|value| *value)),
            Poll::Ready(42),
        );
        assert_eq!(drops.load(Ordering::SeqCst), 1);
        drop(work);
    }

    #[test]
    fn abandoning_deep_unpolled_and_pending_graphs_releases_every_input() {
        for (poll_first, complete) in [(false, false), (true, false), (true, true)] {
            let scope = WorkScope::new();
            let drops = Arc::new(AtomicUsize::new(0));
            let gate = Arc::new(Gate::default());
            let native_gate = Arc::clone(&gate);
            let mut work = scope.factory().create(HeldInput {
                _input: Input(Arc::clone(&drops)),
                inner: Box::pin(async move { native_gate.wait().await }),
            });
            for _ in 0..20_000 {
                let input = Input(Arc::clone(&drops));
                work = scope.factory().compose(|dependencies| HeldInput {
                    _input: input,
                    inner: Box::pin(async move { dependencies.observe(&work).await }),
                });
            }
            if poll_first {
                assert!(
                    Pin::new(&mut work.observe())
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            if complete {
                gate.open();
                assert_eq!(
                    Pin::new(&mut work.observe())
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .map(|result| result.is_ok()),
                    Poll::Ready(true)
                );
            }
            drop(work);
            assert_eq!(drops.load(Ordering::SeqCst), 20_001);
            assert!(scope.registry.lock().operations.is_empty());
        }
    }

    #[test]
    fn aliases_share_one_lazy_operation_and_non_clone_non_sync_completion() {
        let scope = WorkScope::new();
        let gate = Arc::new(Gate::default());
        let native_gate = Arc::clone(&gate);
        let work = scope.factory().create(async move {
            native_gate.wait().await;
            Completion::new(42)
        });
        let alias = work.clone();
        let other = scope
            .factory()
            .create(std::future::ready(Completion::new(42)));
        assert_eq!(work.identity(), alias.identity());
        assert_ne!(work.identity(), other.identity());
        assert_eq!(gate.polls.load(Ordering::Relaxed), 0);

        let mut first = work.observe();
        let mut second = alias.observe();
        let mut cx = Context::from_waker(Waker::noop());
        assert!(Pin::new(&mut first).poll(&mut cx).is_pending());
        gate.open();
        assert_eq!(gate.polls.load(Ordering::Relaxed), 1);
        let first = Pin::new(&mut first)
            .poll(&mut cx)
            .map(|value| value.map(|shared| shared.read(Completion::get)));
        assert_eq!(first, Poll::Ready(Ok(42)));
        let second = Pin::new(&mut second)
            .poll(&mut cx)
            .map(|value| value.map(|shared| shared.read(Completion::get)));
        assert_eq!(second, Poll::Ready(Ok(42)));
        assert_eq!(gate.polls.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn competing_observers_do_not_poll_the_same_native_future_concurrently() {
        use std::sync::Barrier;
        let scope = WorkScope::new();
        let entered = Arc::new(Barrier::new(2));
        let released = Arc::new(Barrier::new(2));
        let polls = Arc::new(AtomicUsize::new(0));
        let work = scope.factory().create({
            let entered = Arc::clone(&entered);
            let released = Arc::clone(&released);
            let polls = Arc::clone(&polls);
            std::future::poll_fn(move |_| {
                polls.fetch_add(1, Ordering::SeqCst);
                entered.wait();
                released.wait();
                Poll::Ready(Completion::new(42))
            })
        });
        let mut first = work.observe();
        let polling = std::thread::spawn(move || {
            Pin::new(&mut first)
                .poll(&mut Context::from_waker(Waker::noop()))
                .map(|value| value.map(|value| value.read(Completion::get)))
        });
        entered.wait();
        let mut second = work.observe();
        let while_polling = Pin::new(&mut second)
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending();
        released.wait();
        assert_eq!(
            polling.join().expect("first observer worker"),
            Poll::Ready(Ok(42))
        );
        assert!(while_polling);
        assert_eq!(
            Pin::new(&mut second)
                .poll(&mut Context::from_waker(Waker::noop()))
                .map(|value| value.map(|value| value.read(Completion::get))),
            Poll::Ready(Ok(42))
        );
        assert_eq!(polls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn shutdown_rejects_registration_even_while_an_in_flight_factory_keeps_the_registry_alive() {
        let scope = WorkScope::new();
        let factory = scope.factory();
        let in_flight = factory
            .registry
            .upgrade()
            .expect("a live registration reservation");
        drop(scope);
        let work = factory.ready(42);
        assert_eq!(
            Pin::new(&mut work.observe())
                .poll(&mut Context::from_waker(Waker::noop()))
                .map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
        assert!(in_flight.lock().operations.is_empty());
        drop(work);
        drop(in_flight);
    }

    #[test]
    fn following_work_shares_the_completed_value_without_cloning_its_payload() {
        struct Payload(Cell<usize>);

        let scope = WorkScope::new();
        let original = scope.factory().ready(Payload(Cell::new(42)));
        let source = original.clone();
        let following = scope
            .factory()
            .compose(|dependencies| async move { dependencies.observe(&source).await });
        let mut cx = Context::from_waker(Waker::noop());
        let first = Pin::new(&mut original.observe())
            .poll(&mut cx)
            .map(|result| result.map(|shared| shared.read(|payload| payload as *const Payload)));
        let second = Pin::new(&mut following.observe())
            .poll(&mut cx)
            .map(|result| result.map(|shared| shared.read(|payload| payload as *const Payload)));
        assert_eq!(first, second);
        assert_ne!(original.identity(), following.identity());
        drop(scope);
        assert_eq!(
            Pin::new(&mut following.observe())
                .poll(&mut cx)
                .map(|result| result.map(|shared| shared.read(|payload| payload.0.get()))),
            Poll::Ready(Ok(42))
        );
    }

    #[test]
    fn ready_is_complete_at_construction_but_a_closed_factory_rejects_new_work() {
        let scope = WorkScope::new();
        let factory = scope.factory();
        let original = factory.ready(42);
        drop(scope);
        let later = factory.ready(7);
        let mut cx = Context::from_waker(Waker::noop());
        assert_eq!(
            Pin::new(&mut original.observe())
                .poll(&mut cx)
                .map(|result| result.map(|shared| shared.read(|value| *value))),
            Poll::Ready(Ok(42))
        );
        assert_eq!(
            Pin::new(&mut later.observe())
                .poll(&mut cx)
                .map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
    }

    #[test]
    fn cancelled_dependency_terminates_its_follower_without_restarting_it() {
        let first_scope = WorkScope::new();
        let dependency = first_scope
            .factory()
            .create(std::future::pending::<Completion>());
        drop(first_scope);
        let scope = WorkScope::new();
        let following = scope
            .factory()
            .compose(|dependencies| async move { dependencies.observe(&dependency).await });
        let independent = scope.factory().ready(Completion::new(42));
        let mut cx = Context::from_waker(Waker::noop());
        for _ in 0..2 {
            assert_eq!(
                Pin::new(&mut following.observe())
                    .poll(&mut cx)
                    .map(Result::err),
                Poll::Ready(Some(Cancelled))
            );
        }
        assert_eq!(
            Pin::new(&mut independent.observe())
                .poll(&mut cx)
                .map(|result| result.map(|shared| shared.read(Completion::get))),
            Poll::Ready(Ok(42))
        );
    }

    #[test]
    fn a_wake_during_native_poll_is_delivered_after_pending_is_restored() {
        let scope = WorkScope::new();
        let mut first = true;
        let work = scope.factory().create(poll_fn(move |cx| {
            if first {
                first = false;
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(Completion::new(42))
            }
        }));
        let notifications = Arc::new(Notifications::default());
        let waker = Waker::from(Arc::clone(&notifications));
        let mut cx = Context::from_waker(&waker);
        let mut observer = work.observe();
        assert!(Pin::new(&mut observer).poll(&mut cx).is_pending());
        assert_eq!(notifications.0.load(Ordering::Relaxed), 2);
        assert_eq!(
            Pin::new(&mut observer)
                .poll(&mut cx)
                .map(|result| result.map(|shared| shared.read(Completion::get))),
            Poll::Ready(Ok(42))
        );
        assert!(work.operation.wake.waiters.lock().is_empty());
    }

    #[test]
    fn waiter_drop_unregisters_without_cancelling_retained_work() {
        let scope = WorkScope::new();
        let gate = Arc::new(Gate::default());
        let native_gate = Arc::clone(&gate);
        let work = scope.factory().create(async move {
            native_gate.wait().await;
            42
        });
        let notified = Arc::new(Notifications::default());
        let waker = Waker::from(Arc::clone(&notified));
        let mut observer = work.observe();
        assert!(
            Pin::new(&mut observer)
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        assert_eq!(work.operation.wake.waiters.lock().len(), 1);
        drop(observer);
        assert_eq!(work.operation.wake.waiters.lock().len(), 0);
        gate.open();
        assert_eq!(notified.0.load(Ordering::Relaxed), 0);
        assert_eq!(gate.polls.load(Ordering::Relaxed), 1);
        let result = Pin::new(&mut work.observe())
            .poll(&mut Context::from_waker(Waker::noop()))
            .map(|value| value.map(|shared| shared.read(|value| *value)));
        assert_eq!(result, Poll::Ready(Ok(42)));
    }

    #[test]
    fn final_owner_releases_unpolled_inputs_and_registry_entries() {
        let scope = WorkScope::new();
        let drops = Arc::new(AtomicUsize::new(0));
        for _ in 0..100 {
            let input = Input(Arc::clone(&drops));
            let work = scope.factory().create(HeldInput {
                _input: input,
                inner: std::future::pending::<()>(),
            });
            drop(work);
        }
        assert_eq!(drops.load(Ordering::Relaxed), 100);
        let registered = scope.registry.lock().operations.len();
        assert_eq!(registered, 0);
    }

    #[test]
    fn ending_scope_cancels_pending_work_and_preserves_completed_results() {
        let scope = WorkScope::new();
        let drops = Arc::new(AtomicUsize::new(0));
        let input = Input(Arc::clone(&drops));
        let pending = scope.factory().create(HeldInput {
            _input: input,
            inner: std::future::pending::<i32>(),
        });
        let completed = scope.factory().create(async { 42 });
        let mut cx = Context::from_waker(Waker::noop());
        assert!(Pin::new(&mut pending.observe()).poll(&mut cx).is_pending());
        assert!(Pin::new(&mut completed.observe()).poll(&mut cx).is_ready());
        drop(scope);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(
            Pin::new(&mut pending.observe())
                .poll(&mut cx)
                .map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
        assert_eq!(
            Pin::new(&mut completed.observe())
                .poll(&mut cx)
                .map(|value| value.map(|shared| shared.read(|value| *value))),
            Poll::Ready(Ok(42))
        );
    }

    #[test]
    fn factory_after_scope_drop_discards_new_work_without_polling() {
        let scope = WorkScope::new();
        let factory = scope.factory();
        drop(scope);
        let drops = Arc::new(AtomicUsize::new(0));
        let input = Input(Arc::clone(&drops));
        let work = factory.create(HeldInput {
            _input: input,
            inner: std::future::ready(42),
        });
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(
            Pin::new(&mut work.observe())
                .poll(&mut Context::from_waker(Waker::noop()))
                .map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
    }

    #[test]
    fn dropping_the_last_polled_waiter_preserves_other_wake_registrations() {
        let scope = WorkScope::new();
        let gate = Arc::new(Gate::default());
        let native_gate = Arc::clone(&gate);
        let work = scope
            .factory()
            .create(async move { native_gate.wait().await });
        let first = Arc::new(Notifications::default());
        let second = Arc::new(Notifications::default());
        let first_waker = Waker::from(Arc::clone(&first));
        let second_waker = Waker::from(Arc::clone(&second));
        let mut one = work.observe();
        let mut two = work.observe();
        assert!(
            Pin::new(&mut one)
                .poll(&mut Context::from_waker(&first_waker))
                .is_pending()
        );
        assert!(
            Pin::new(&mut two)
                .poll(&mut Context::from_waker(&second_waker))
                .is_pending()
        );
        drop(two);
        gate.open();
        assert_eq!(first.0.load(Ordering::Relaxed), 1);
        assert_eq!(second.0.load(Ordering::Relaxed), 0);
        assert_eq!(
            Pin::new(&mut one)
                .poll(&mut Context::from_waker(&first_waker))
                .map(|result| result.is_ok()),
            Poll::Ready(true)
        );
        gate.open();
    }

    #[test]
    fn scope_shutdown_during_native_poll_does_not_wait_or_publish_a_late_result() {
        let scope = WorkScope::new();
        let entered = Arc::new(Barrier::new(2));
        let release = Arc::new(Barrier::new(2));
        let native_entered = Arc::clone(&entered);
        let native_release = Arc::clone(&release);
        let drops = Arc::new(AtomicUsize::new(0));
        let input = Input(Arc::clone(&drops));
        let work = scope.factory().create(async move {
            native_entered.wait();
            native_release.wait();
            drop(input);
            Completion::new(42)
        });
        std::thread::scope(|threads| {
            let work = work.clone();
            let worker = threads.spawn(move || {
                Pin::new(&mut work.observe())
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .map(Result::err)
            });
            entered.wait();
            drop(scope);
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            release.wait();
            assert_eq!(worker.join().unwrap(), Poll::Ready(Some(Cancelled)));
        });
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn native_panic_unwinds_and_cancels_the_abandoned_poll() {
        let scope = WorkScope::<Completion>::new();
        let work = scope.factory().create(async { panic!("native panic") });
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = Pin::new(&mut work.observe()).poll(&mut Context::from_waker(Waker::noop()));
        }));
        assert!(result.is_err());
        assert_eq!(
            Pin::new(&mut work.observe())
                .poll(&mut Context::from_waker(Waker::noop()))
                .map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
    }
}
