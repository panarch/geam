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
    Nested(crate::ExecutionError),
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
}

impl HostCallError {
    pub(crate) fn nested(error: crate::ExecutionError) -> Self {
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

#[cfg(test)]
mod tests {
    use super::{HostCallError, HostFailure};
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
}
