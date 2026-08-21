use ecow::EcoString;
use std::fmt::{self, Display, Formatter};

pub type Configuration = crate::HostProviderConfiguration;

/// A provider-owned configuration failure before component identity is added.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitializationError {
    reason: EcoString,
}

impl InitializationError {
    pub fn new(reason: impl Into<EcoString>) -> Self {
        Self {
            reason: reason.into(),
        }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl Display for InitializationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.reason)
    }
}

impl std::error::Error for InitializationError {}

#[cfg(test)]
mod tests {
    use super::InitializationError;

    #[test]
    fn initialization_error_owns_only_the_provider_reason() {
        let error = InitializationError::new("configuration key `start` is missing");

        assert_eq!(error.reason(), "configuration key `start` is missing");
        assert_eq!(error.to_string(), "configuration key `start` is missing");
        assert_eq!(error.clone(), error);
    }
}
