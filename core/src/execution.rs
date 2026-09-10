//! Host-selected scheduling and clocks for owned Gleam execution.

#[cfg(feature = "tokio")]
mod tokio;

#[cfg(feature = "tokio")]
pub use tokio::TokioHost;

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

#[cfg(test)]
mod tests {
    use super::{DriverError, RunError};
    use std::error::Error;

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
