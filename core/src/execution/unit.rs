use futures_channel::mpsc::UnboundedSender;
use futures_util::task::AtomicWaker;
use std::future::{Future, poll_fn};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock, Weak};
use std::task::Poll;

/// A non-owning identity and cancellation handle for a logical execution.
///
/// Keeping this handle does not keep execution or its domain alive. Cancellation
/// closes effect admission immediately; the domain still joins physical workers.
#[derive(Clone)]
pub struct ExecutionUnit {
    id: ExecutionUnitId,
    state: Weak<State>,
}

/// Identity of one logical execution, independent of its executor workers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionUnitId(u64);

struct State {
    id: ExecutionUnitId,
    exit: OnceLock<UnitExit>,
    root: AtomicWaker,
    finished: UnboundedSender<UnitFinished>,
}

pub(crate) struct UnitOwner {
    state: Arc<State>,
}

/// Keeps the terminal result available to domain shutdown, not the worker alive.
pub(crate) struct UnitRecord(Arc<State>);

pub(crate) struct UnitFinished {
    pub(crate) unit: ExecutionUnitId,
    pub(crate) exit: UnitExit,
}

/// Logical termination, not acknowledgement of physical worker destruction.
#[derive(Clone, Debug)]
pub enum UnitExit {
    Completed,
    Cancelled,
    Failed(crate::ExecutionError),
}

/// Typed, domain-owned services observing logical execution lifetimes.
///
/// Hooks run on the domain driver, without an outstanding provider-state borrow.
/// They must finish synchronously. Package-specific policies belong to the
/// implementation, not the core scheduler.
pub trait HostExecutionState: Default + Send {
    /// Initializes services once, before admitting the domain's first unit.
    fn initialize(&mut self, _metadata: super::ExecutionMetadata<'_>) {}

    fn started(&mut self, unit: ExecutionUnit);
    fn finished(&mut self, unit: ExecutionUnitId, exit: &UnitExit);
    fn close(&mut self);

    /// Polls one bounded unit of domain-owned service work. Return Ready only
    /// after making progress; Pending must register any wake needed for work.
    fn poll(
        &mut self,
        _cx: &mut std::task::Context<'_>,
        _clock: super::ExecutionClock<'_>,
    ) -> std::task::Poll<()> {
        std::task::Poll::Pending
    }
}

impl HostExecutionState for () {
    fn started(&mut self, _unit: ExecutionUnit) {}
    fn finished(&mut self, _unit: ExecutionUnitId, _exit: &UnitExit) {}
    fn close(&mut self) {}
}

impl ExecutionUnit {
    pub fn id(&self) -> ExecutionUnitId {
        self.id
    }

    pub fn is_active(&self) -> bool {
        self.state
            .upgrade()
            .is_some_and(|state| state.exit.get().is_none())
    }

    /// Requests cancellation once. Already terminated units remain terminated.
    pub fn cancel(&self) -> bool {
        self.state.upgrade().is_some_and(|state| state.cancel())
    }
}

impl std::fmt::Debug for ExecutionUnit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("ExecutionUnit")
            .field(&self.id)
            .finish()
    }
}

impl UnitOwner {
    pub(crate) fn new(finished: UnboundedSender<UnitFinished>) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        Self {
            state: Arc::new(State {
                id: ExecutionUnitId(NEXT.fetch_add(1, Ordering::Relaxed)),
                exit: OnceLock::new(),
                root: AtomicWaker::new(),
                finished,
            }),
        }
    }

    pub(crate) fn handle(&self) -> ExecutionUnit {
        ExecutionUnit {
            id: self.state.id,
            state: Arc::downgrade(&self.state),
        }
    }

    pub(crate) fn finish(&self, exit: UnitExit) -> bool {
        self.state.finish(exit).1
    }

    pub(crate) fn record(&self) -> UnitRecord {
        UnitRecord(Arc::clone(&self.state))
    }

    pub(crate) fn cancelled(&self) -> impl Future<Output = ()> + Send + use<> {
        let state = Arc::clone(&self.state);
        poll_fn(move |cx| {
            state.root.register(cx.waker());
            if state.exit.get().is_none() {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
    }
}

impl Drop for UnitOwner {
    fn drop(&mut self) {
        self.state.cancel();
    }
}

impl UnitRecord {
    pub(crate) fn close(&self) -> UnitExit {
        self.0.finish(UnitExit::Cancelled).0.clone()
    }
}

impl State {
    fn cancel(&self) -> bool {
        self.finish(UnitExit::Cancelled).1
    }

    fn finish(&self, exit: UnitExit) -> (&UnitExit, bool) {
        let mut first = false;
        let exit = self.exit.get_or_init(|| {
            first = true;
            exit
        });
        if first {
            let _ = self.finished.unbounded_send(UnitFinished {
                unit: self.id,
                exit: exit.clone(),
            });
            self.root.wake();
        }
        (exit, first)
    }
}

#[cfg(test)]
mod tests {
    use super::{HostExecutionState, UnitExit, UnitOwner};
    use std::future::Future;
    use std::pin::pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Wake, Waker};

    #[derive(Default)]
    struct Wakes(AtomicUsize);

    impl Wake for Wakes {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn cancellation_closes_admission_and_wakes_the_root_exactly_once() {
        let (finished, mut events) = futures_channel::mpsc::unbounded();
        let owner = UnitOwner::new(finished);
        let handle = owner.handle();
        let alias = handle.clone();
        let wakes = Arc::new(Wakes::default());
        let waker = Waker::from(Arc::clone(&wakes));
        let mut cx = Context::from_waker(&waker);
        let mut cancelled = pin!(owner.cancelled());

        assert_eq!(handle.id(), alias.id());
        assert!(handle.is_active());
        assert!(cancelled.as_mut().poll(&mut cx).is_pending());
        assert!(alias.cancel());
        assert!(!handle.is_active());
        assert!(!handle.cancel());
        assert_eq!(wakes.0.load(Ordering::SeqCst), 1);
        let event = events.try_recv().unwrap();
        assert_eq!(event.unit, handle.id());
        assert_eq!(format!("{:?}", event.exit), "Cancelled");
        assert!(events.try_recv().is_err());
        assert_eq!(cancelled.as_mut().poll(&mut cx), Poll::Ready(()));
        drop(owner);
        assert!(!handle.cancel());
        assert_eq!(wakes.0.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn dead_handles_do_not_retain_or_reactivate_the_execution() {
        let (finished, _) = futures_channel::mpsc::unbounded();
        let owner = UnitOwner::new(finished.clone());
        let handle = owner.handle();
        let distinct = UnitOwner::new(finished);
        assert_ne!(handle.id(), distinct.handle().id());
        assert_eq!(
            format!("{handle:?}"),
            format!("ExecutionUnit(ExecutionUnitId({}))", handle.id().0)
        );
        drop(owner);
        assert!(handle.state.upgrade().is_none());
        assert!(!handle.is_active());
        assert!(!handle.cancel());
        assert!(distinct.handle().is_active());
    }

    #[test]
    fn completion_and_owner_drop_close_before_a_late_wait() {
        for complete in [false, true] {
            let (finished, mut events) = futures_channel::mpsc::unbounded();
            let owner = UnitOwner::new(finished);
            let handle = owner.handle();
            let cancelled = owner.cancelled();
            if complete {
                assert!(owner.finish(UnitExit::Completed));
            }
            drop(owner);
            assert!(!handle.is_active());
            assert!(!handle.cancel());
            let event = events.try_recv().unwrap();
            assert_eq!(event.unit, handle.id());
            assert_eq!(matches!(event.exit, UnitExit::Completed), complete);
            assert!(events.try_recv().is_err());
            assert_eq!(
                pin!(cancelled)
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop())),
                Poll::Ready(()),
            );
        }
    }

    #[test]
    fn stateless_execution_services_accept_the_full_lifecycle() {
        let (finished, _) = futures_channel::mpsc::unbounded();
        let owner = UnitOwner::new(finished);
        let mut state = ();
        state.started(owner.handle());
        for (exit, expected) in [
            (UnitExit::Completed, "Completed"),
            (UnitExit::Cancelled, "Cancelled"),
        ] {
            state.finished(owner.handle().id(), &exit);
            assert_eq!(format!("{exit:?}"), expected);
        }
        state.close();
    }
}
