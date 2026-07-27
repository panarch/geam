use ecow::EcoString;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFailure {
    message: EcoString,
}

#[derive(Debug, PartialEq)]
pub struct HostCallError {
    pub(crate) kind: HostCallErrorKind,
}

#[derive(Debug, PartialEq)]
pub(crate) enum HostCallErrorKind {
    Failure(HostFailure),
    Execution(crate::runtime::ExecutionError),
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
            HostCallErrorKind::Failure(error) => Display::fmt(error, formatter),
            HostCallErrorKind::Execution(error) => Display::fmt(error, formatter),
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

impl From<crate::runtime::ExecutionError> for HostCallError {
    fn from(error: crate::runtime::ExecutionError) -> Self {
        Self {
            kind: HostCallErrorKind::Execution(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HostCallError, HostCallErrorKind, HostFailure};
    use crate::runtime::{ExecutionError, InvariantError};

    #[test]
    fn host_failure_owns_and_displays_its_message() {
        let failure = HostFailure::new("database unavailable");

        assert_eq!(failure.message(), "database unavailable");
        assert_eq!(failure.to_string(), "database unavailable");
    }

    #[test]
    fn host_call_error_preserves_local_and_nested_failure_domains() {
        let local = HostCallError::from(HostFailure::new("invalid input"));
        let nested = HostCallError::from(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: crate::plan::ValueType::Int,
                index: 1,
                length: 0,
            },
        ));

        assert_eq!(
            &local.kind,
            &HostCallErrorKind::Failure(HostFailure::new("invalid input")),
        );
        assert_eq!(
            &nested.kind,
            &HostCallErrorKind::Execution(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: crate::plan::ValueType::Int,
                    index: 1,
                    length: 0,
                },
            )),
        );
        assert_eq!(local.to_string(), "invalid input");
        assert_eq!(
            nested.to_string(),
            "list index out of bounds for Int list (index 1, length 0)"
        );
    }
}
