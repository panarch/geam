use ecow::EcoString;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFailure {
    message: EcoString,
}

#[derive(Debug, PartialEq)]
pub struct HostCallError {
    failure: HostFailure,
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
    pub(crate) fn into_failure(self) -> HostFailure {
        self.failure
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
        Display::fmt(&self.failure, formatter)
    }
}

impl std::error::Error for HostCallError {}

impl From<HostFailure> for HostCallError {
    fn from(error: HostFailure) -> Self {
        Self { failure: error }
    }
}

#[cfg(test)]
mod tests {
    use super::{HostCallError, HostFailure};

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
        assert_eq!(local.into_failure(), HostFailure::new("invalid input"));
    }
}
