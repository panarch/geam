use ecow::EcoString;
use std::fmt::{self, Display, Formatter};

mod list;

pub use list::List;
#[doc(hidden)]
pub use list::{
    ProviderExternalItem, ProviderExternalPayloadAccess, ProviderListContext,
    ProviderListItemDecoder, ProviderListItemValue,
};

pub type Configuration = crate::HostProviderConfiguration;

/// Source-visible semantics for one immutable Rust-owned external payload.
///
/// Values that compare equal must return the same source hash. Hashes are
/// runtime indexes rather than stable serialized values. Inspection should be
/// canonical and source-oriented rather than exposing the payload's Rust
/// `Debug` representation. A payload must not change through interior
/// mutability after it is returned to Geam; source-visible updates return a new
/// owned payload instead.
pub trait ExternalPayload: 'static {
    fn source_equal(&self, other: &Self) -> bool;

    fn source_hash(&self) -> u64;

    fn inspect(&self) -> EcoString;
}

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
    use super::{ExternalPayload, InitializationError};
    use ecow::EcoString;

    struct Payload(u64);

    impl ExternalPayload for Payload {
        fn source_equal(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        fn source_hash(&self) -> u64 {
            self.0
        }

        fn inspect(&self) -> EcoString {
            format!("Payload({})", self.0).into()
        }
    }

    #[test]
    fn initialization_error_owns_only_the_provider_reason() {
        let error = InitializationError::new("configuration key `start` is missing");

        assert_eq!(error.reason(), "configuration key `start` is missing");
        assert_eq!(error.to_string(), "configuration key `start` is missing");
        assert_eq!(error.clone(), error);
    }

    #[test]
    fn external_payload_owns_context_free_source_semantics() {
        let value = Payload(7);
        let equal = Payload(7);
        let different = Payload(8);

        assert!(value.source_equal(&equal));
        assert!(!value.source_equal(&different));
        assert_eq!(value.source_hash(), equal.source_hash());
        assert_eq!(value.inspect(), "Payload(7)");
    }
}
