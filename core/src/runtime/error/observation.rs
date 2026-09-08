use super::SharedExecutionError;

/// Work termination is separate from a source `Result` value.
#[derive(Clone)]
pub enum ObservationError {
    /// The operation was cancelled before completing.
    Cancelled,
    /// A source panic or native failure at its original site.
    Execution(SharedExecutionError),
}

impl std::fmt::Debug for ObservationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => f.write_str("Cancelled"),
            Self::Execution(error) => f.debug_tuple("Execution").field(error).finish(),
        }
    }
}

impl std::fmt::Display for ObservationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => f.write_str("the Future operation was cancelled"),
            Self::Execution(error) => std::fmt::Display::fmt(error, f),
        }
    }
}

impl std::error::Error for ObservationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Cancelled => None,
            Self::Execution(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ObservationError;
    #[test]
    fn cancelled_observation_has_its_own_lifecycle_message_without_an_error_source() {
        use std::error::Error;
        let cancelled = ObservationError::Cancelled;
        assert_eq!(format!("{cancelled:?}"), "Cancelled");
        assert_eq!(cancelled.to_string(), "the Future operation was cancelled");
        assert!(cancelled.source().is_none());
    }

    #[test]
    fn failed_observation_keeps_the_shared_execution_diagnostic_as_its_source() {
        use crate::runtime::SharedExecutionError;
        use crate::runtime::shared::Shared;
        use crate::{ExecutionError, InvariantError, ValueType};
        use std::error::Error;
        let error = ObservationError::Execution(SharedExecutionError(Shared::new(
            ExecutionError::Invariant(InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Int,
                index: 1,
                length: 0,
            }),
        )));
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
