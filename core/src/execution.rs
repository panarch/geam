//! Host-selected scheduling and clocks for owned Gleam execution.

#[cfg(feature = "tokio")]
mod tokio;
mod unit;

#[cfg(feature = "tokio")]
pub use tokio::TokioHost;
pub use unit::{ExecutionUnit, ExecutionUnitId, HostExecutionState, UnitExit};
pub(crate) use unit::{UnitFinished, UnitOwner, UnitRecord};

use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

/// A worker owns its execution state and closed service endpoints, not host borrows.
pub type Worker = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// Physical completion of an executor worker, distinct from a Gleam return or exit.
pub enum TaskExit {
    Completed,
    Cancelled,
    Failed(Box<dyn std::error::Error + Send + Sync>),
}

/// An executor stopped an active worker outside the domain's shutdown.
#[derive(Debug)]
pub enum DriverError {
    Cancelled,
    Failed(Box<dyn std::error::Error + Send + Sync>),
}

/// Failure of an explicitly driven application entry.
#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("the Gleam entry was cancelled")]
    Cancelled,
    #[error(transparent)]
    Execution(#[from] crate::ExecutionError),
    #[error(transparent)]
    Driver(#[from] DriverError),
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => formatter.write_str("the host executor cancelled an active worker"),
            Self::Failed(error) => write!(formatter, "the host executor failed: {error}"),
        }
    }
}

impl std::error::Error for DriverError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Cancelled => None,
            Self::Failed(error) => Some(error.as_ref()),
        }
    }
}

/// Cancellation and acknowledged destruction of one executor worker.
///
/// Cancellation is a request, not a synchronous join. A ready result guarantees
/// that the worker Future has been destroyed. Dropping this handle must request
/// cancellation without blocking or detaching the worker.
pub trait HostTask: Future<Output = TaskExit> + Send + Unpin + 'static {
    fn cancel(&self);
}

/// An application's scheduling and monotonic-clock capabilities.
///
/// Implementations use the application's executor. They must not synchronously
/// poll a worker from `spawn` or a wake notification. The returned task handle
/// owns cancellation and completion acknowledgement for that worker.
pub trait ExecutionHost: Send + Sync + 'static {
    fn spawn(&self, worker: Worker) -> Box<dyn HostTask>;
    fn now(&self) -> Instant;
    fn sleep_until(&self, deadline: Instant) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}

/// Immutable value metadata available when a domain initializes its services.
pub struct ExecutionMetadata<'plan>(
    pub(crate) crate::plan::execution::runtime::RuntimeValueMetadata<'plan>,
);

impl<'plan> ExecutionMetadata<'plan> {
    /// Native tags of constructors retained in the sealed execution catalog.
    /// Multiple specializations may contain the same tag.
    pub fn native_constructor_tags(self) -> impl Iterator<Item = &'plan ecow::EcoString> {
        self.0.native_constructor_tags()
    }
}

/// Borrowed access to the host's monotonic clock, without worker admission.
#[derive(Clone, Copy)]
pub struct ExecutionClock<'host>(&'host dyn ExecutionHost);

impl<'host> ExecutionClock<'host> {
    pub(crate) fn new(host: &'host dyn ExecutionHost) -> Self {
        Self(host)
    }

    pub fn now(self) -> Instant {
        self.0.now()
    }

    /// The returned wait owns no host or provider-state borrow.
    pub fn sleep_until(self, deadline: Instant) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        self.0.sleep_until(deadline)
    }
}

#[cfg(test)]
mod tests {
    use super::{DriverError, RunError};
    use std::error::Error;

    #[test]
    fn domain_initialization_observes_sealed_native_tags_before_any_unit_starts() {
        use super::{
            ExecutionMetadata, ExecutionUnit, ExecutionUnitId, HostExecutionState, UnitExit,
        };
        use crate::{HostProfile, HostedExecution};
        use std::collections::BTreeSet;
        use std::sync::{Arc, Mutex};

        #[derive(Default)]
        struct Tags(Arc<Mutex<BTreeSet<ecow::EcoString>>>);
        impl HostExecutionState for Tags {
            fn initialize(&mut self, metadata: ExecutionMetadata<'_>) {
                self.0
                    .lock()
                    .unwrap()
                    .extend(metadata.native_constructor_tags().cloned());
            }
            fn started(&mut self, _: ExecutionUnit) {
                assert_eq!(
                    *self.0.lock().unwrap(),
                    BTreeSet::from(["before_first".into(), "has_data".into()])
                );
            }
            fn finished(&mut self, _: ExecutionUnitId, _: &UnitExit) {}
            fn close(&mut self) {}
        }
        struct Profile;
        impl HostProfile for Profile {
            type RunState = Arc<Mutex<BTreeSet<ecow::EcoString>>>;
            type ExternalStores = ();
            type ExecutionState = Tags;
            fn initialize_execution(state: &mut Self::RunState) -> Tags {
                Tags(Arc::clone(state))
            }
        }
        let program = crate::compile_typed_host_program(
            "application", "main",
            [crate::PackageSource::new("application", Vec::<String>::new(), [crate::ModuleSource::new(
                "main", "synthetic/main.gleam",
                "pub type Marker { BeforeFirst HasData(Int) }\npub fn main() { #(BeforeFirst, HasData(42)) }",
            )])],
            crate::HostProviderSet::<Profile>::from_providers([]).unwrap(),
        ).unwrap();
        assert!(program.package_resources().is_empty());
        let mut execution =
            HostedExecution::try_from_module_plan(crate::plan_host_program(program).unwrap())
                .unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut observed = Arc::default();
        let value = host
            .block_on(execution.run_main(&host, &mut observed, &mut Vec::new()))
            .unwrap();
        assert_eq!(value.inspect().to_string(), "#(BeforeFirst, HasData(42))");
        assert_eq!(
            *observed.lock().unwrap(),
            BTreeSet::from(["before_first".into(), "has_data".into()])
        );
    }

    #[test]
    fn borrowed_clock_uses_the_hosts_time_and_wait_without_spawning() {
        use super::{ExecutionClock, ExecutionHost, HostExecutionState};
        use crate::execution_fixture::TestHost;
        use std::task::{Context, Poll, Waker};
        use std::time::Duration;

        let host = TestHost::default();
        let clock = ExecutionClock::new(&host);
        assert_eq!(clock.now(), host.now());
        let mut wait = clock.sleep_until(clock.now() + Duration::from_secs(2));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(wait.as_mut().poll(&mut cx).is_pending());
        host.advance(Duration::from_secs(1));
        assert!(wait.as_mut().poll(&mut cx).is_pending());
        host.advance(Duration::from_secs(1));
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(()));
        assert_eq!(().poll(&mut cx, clock), Poll::Pending);
    }

    #[test]
    fn driver_diagnostics_preserve_the_executor_failure_source() {
        let cancelled = DriverError::Cancelled;
        assert_eq!(
            cancelled.to_string(),
            "the host executor cancelled an active worker"
        );
        assert!(cancelled.source().is_none());

        let failed = DriverError::Failed(std::io::Error::other("worker stopped").into());
        assert_eq!(
            failed.to_string(),
            "the host executor failed: worker stopped"
        );
        assert_eq!(failed.source().unwrap().to_string(), "worker stopped");
        let run = RunError::from(failed);
        assert_eq!(run.to_string(), "the host executor failed: worker stopped");
        assert_eq!(run.source().unwrap().to_string(), "worker stopped");
        assert_eq!(
            RunError::Cancelled.to_string(),
            "the Gleam entry was cancelled"
        );
    }
}
