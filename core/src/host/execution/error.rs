use crate::host::HostCallError;
pub use crate::runtime::SharedExecutionError;
use std::fmt;

/// A native execution failure, separate from the Gleam function's return value.
#[derive(Debug)]
pub enum HostExecutionError {
    /// The operation was cancelled before completing.
    Cancelled,
    /// A native failure or an unchanged error from a Gleam callback.
    Host(HostCallError),
    /// The unchanged shared failure of an explicitly observed source Future.
    Execution(SharedExecutionError),
}

impl From<crate::runtime::work::Cancelled> for HostExecutionError {
    fn from(_: crate::runtime::work::Cancelled) -> Self {
        Self::Cancelled
    }
}

impl From<HostCallError> for HostExecutionError {
    fn from(error: HostCallError) -> Self {
        Self::Host(error)
    }
}

impl From<crate::HostFailure> for HostExecutionError {
    fn from(error: crate::HostFailure) -> Self {
        Self::Host(error.into())
    }
}

impl fmt::Display for HostExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled => formatter.write_str("the native operation was cancelled"),
            Self::Host(error) => fmt::Display::fmt(error, formatter),
            Self::Execution(error) => fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for HostExecutionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Cancelled => None,
            Self::Host(error) => Some(error),
            Self::Execution(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HostExecutionError, SharedExecutionError};
    use crate::runtime::shared::Shared;
    use crate::{ExecutionError, HostFailure, InvariantError, ValueType};
    use std::error::Error;

    #[test]
    fn termination_and_native_and_shared_failures_keep_distinct_diagnostics() {
        let cancelled = HostExecutionError::from(crate::runtime::work::Cancelled);
        assert_eq!(cancelled.to_string(), "the native operation was cancelled");
        assert_eq!(format!("{cancelled:?}"), "Cancelled");
        assert!(cancelled.source().is_none());

        let host = HostExecutionError::from(HostFailure::new("disconnected"));
        assert_eq!(host.to_string(), "disconnected");
        assert_eq!(
            format!("{host:?}"),
            "Host(HostCallError { kind: Failure(HostFailure { message: \"disconnected\" }) })"
        );
        assert_eq!(
            host.source().expect("host failure").to_string(),
            "disconnected"
        );

        let shared = SharedExecutionError(Shared::new(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Int,
                index: 1,
                length: 0,
            },
        )));
        let error = HostExecutionError::Execution(shared);
        assert_eq!(
            error.to_string(),
            "list index out of bounds for Int list (index 1, length 0)"
        );
        assert_eq!(
            format!("{error:?}"),
            "Execution(Invariant(ListIndexOutOfBounds { item_type: Int, index: 1, length: 0 }))"
        );
        assert_eq!(
            error
                .source()
                .expect("shared execution failure")
                .to_string(),
            "list index out of bounds for Int list (index 1, length 0)"
        );
    }
}
