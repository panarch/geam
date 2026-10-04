use crate::ExecutionError;
use crate::execution::{ExitStatus, InvalidExitStatus};
use ecow::EcoString;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFailure {
    message: EcoString,
}

#[derive(Debug, PartialEq)]
pub struct HostCallError {
    kind: HostCallErrorKind,
}

#[derive(Debug, PartialEq)]
pub(crate) enum HostCallErrorKind {
    Failure(HostFailure),
    Nested(ExecutionError),
    Exited(ExitStatus),
}

impl HostFailure {
    pub fn new(message: impl Into<EcoString>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &EcoString {
        &self.message
    }

    pub(super) fn uninhabited_return() -> Self {
        Self::new("native call completed with a value for an uninhabited return type")
    }
}

impl HostCallError {
    pub(crate) fn exited(status: ExitStatus) -> Self {
        Self {
            kind: HostCallErrorKind::Exited(status),
        }
    }

    pub(crate) fn nested(error: ExecutionError) -> Self {
        Self {
            kind: HostCallErrorKind::Nested(error),
        }
    }

    pub(crate) fn into_kind(self) -> HostCallErrorKind {
        self.kind
    }
}

impl Display for HostFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for HostFailure {}

impl Display for HostCallError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            HostCallErrorKind::Failure(failure) => Display::fmt(failure, formatter),
            HostCallErrorKind::Nested(error) => Display::fmt(error, formatter),
            HostCallErrorKind::Exited(status) => {
                write!(formatter, "application requested exit status {status}")
            }
        }
    }
}

impl std::error::Error for HostCallError {}

impl From<HostFailure> for HostCallError {
    fn from(error: HostFailure) -> Self {
        Self {
            kind: HostCallErrorKind::Failure(error),
        }
    }
}

impl From<std::convert::Infallible> for HostCallError {
    fn from(error: std::convert::Infallible) -> Self {
        match error {}
    }
}

impl From<InvalidExitStatus> for HostCallError {
    fn from(error: InvalidExitStatus) -> Self {
        HostFailure::new(error.to_string()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::{HostCallError, HostCallErrorKind, HostFailure};
    use crate::{ExecutionError, InvariantError, ValueType};

    #[test]
    fn host_failure_owns_and_displays_its_message() {
        let failure = HostFailure::new("database unavailable");

        assert_eq!(failure.message(), "database unavailable");
        assert_eq!(failure.to_string(), "database unavailable");
    }

    #[test]
    fn host_call_error_preserves_the_owned_host_failure() {
        let local = HostCallError::from(HostFailure::new("invalid input"));

        assert_eq!(local.to_string(), "invalid input");
        assert_eq!(
            local.into_kind(),
            super::HostCallErrorKind::Failure(HostFailure::new("invalid input")),
        );
    }

    #[test]
    fn host_call_error_preserves_a_nested_execution_failure() {
        let execution = ExecutionError::Invariant(InvariantError::ListIndexOutOfBounds {
            item_type: ValueType::Int,
            index: 1,
            length: 0,
        });
        let nested = HostCallError::nested(execution.clone());

        assert_eq!(
            nested.to_string(),
            "list index out of bounds for Int list (index 1, length 0)",
        );
        assert_eq!(
            nested.into_kind(),
            super::HostCallErrorKind::Nested(execution),
        );
    }

    #[test]
    fn async_host_call_error_preserves_owned_and_nested_failures() {
        let failure = HostCallError::from(HostFailure::new("async input rejected"));

        assert_eq!(failure.to_string(), "async input rejected");

        assert_eq!(
            failure.into_kind(),
            HostCallErrorKind::Failure(HostFailure::new("async input rejected")),
        );

        let invariant = InvariantError::ListIndexOutOfBounds {
            item_type: ValueType::String,
            index: 2,
            length: 1,
        };
        let nested = HostCallError::nested(invariant.clone().into());

        assert_eq!(
            nested.to_string(),
            "list index out of bounds for String list (index 2, length 1)",
        );

        assert_eq!(
            nested.into_kind(),
            HostCallErrorKind::Nested(ExecutionError::Invariant(invariant))
        );
    }

    #[test]
    fn intentional_termination_and_invalid_status_keep_distinct_protocols() {
        use crate::execution::{ExitStatus, InvalidExitStatus};
        let status = ExitStatus::new(7);
        let exit = HostCallError::exited(status);
        assert_eq!(exit.to_string(), "application requested exit status 7");
        assert_eq!(exit.into_kind(), HostCallErrorKind::Exited(status));
        let invalid = HostCallError::from(InvalidExitStatus);
        assert_eq!(
            invalid.to_string(),
            "application exit status must be between 0 and 255"
        );
        assert_eq!(
            invalid.into_kind(),
            HostCallErrorKind::Failure(HostFailure::new(
                "application exit status must be between 0 and 255"
            ))
        );
    }
}
