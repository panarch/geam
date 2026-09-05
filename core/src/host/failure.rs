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

/// A failure returned by a scoped async host function.
///
/// It represents either an explicit [`HostFailure`] or an error from a nested
/// Gleam callback invoked through [`crate::AsyncHostCall::invoke`].
pub struct AsyncHostCallError {
    kind: AsyncHostCallErrorKind,
}

#[derive(Debug, PartialEq)]
pub(crate) enum HostCallErrorKind {
    Failure(HostFailure),
    Nested(crate::ExecutionError),
}

pub(crate) enum AsyncHostCallErrorKind {
    Failure(HostFailure),
    Nested(crate::runtime::TransferExecutionError),
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

impl AsyncHostCallError {
    pub(crate) fn nested(error: crate::runtime::TransferExecutionError) -> Self {
        Self {
            kind: AsyncHostCallErrorKind::Nested(error),
        }
    }

    pub(crate) fn into_kind(self) -> AsyncHostCallErrorKind {
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

impl Display for AsyncHostCallError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            AsyncHostCallErrorKind::Failure(failure) => Display::fmt(failure, formatter),
            AsyncHostCallErrorKind::Nested(error) => Display::fmt(error, formatter),
        }
    }
}

impl fmt::Debug for AsyncHostCallError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("AsyncHostCallError")
            .field(&self.to_string())
            .finish()
    }
}

impl std::error::Error for AsyncHostCallError {}

impl From<HostFailure> for HostCallError {
    fn from(error: HostFailure) -> Self {
        Self {
            kind: HostCallErrorKind::Failure(error),
        }
    }
}

impl From<HostFailure> for AsyncHostCallError {
    fn from(error: HostFailure) -> Self {
        Self {
            kind: AsyncHostCallErrorKind::Failure(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AsyncHostCallError, AsyncHostCallErrorKind, HostCallError, HostFailure};
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
        fn classify(error: AsyncHostCallError) -> Result<HostFailure, ExecutionError> {
            match error.into_kind() {
                AsyncHostCallErrorKind::Failure(failure) => Ok(failure),
                AsyncHostCallErrorKind::Nested(error) => Err(error.into_local()),
            }
        }

        let failure = AsyncHostCallError::from(HostFailure::new("async input rejected"));

        assert_eq!(failure.to_string(), "async input rejected");
        assert_eq!(
            format!("{failure:?}"),
            "AsyncHostCallError(\"async input rejected\")"
        );
        assert_eq!(
            classify(failure),
            Ok(HostFailure::new("async input rejected")),
        );

        let invariant = InvariantError::ListIndexOutOfBounds {
            item_type: ValueType::String,
            index: 2,
            length: 1,
        };
        let nested = AsyncHostCallError::nested(invariant.clone().into());

        assert_eq!(
            nested.to_string(),
            "list index out of bounds for String list (index 2, length 1)",
        );
        assert_eq!(
            format!("{nested:?}"),
            "AsyncHostCallError(\"list index out of bounds for String list (index 2, length 1)\")",
        );
        assert_eq!(classify(nested), Err(ExecutionError::Invariant(invariant)));
    }
}
