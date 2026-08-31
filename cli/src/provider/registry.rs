mod archive;
mod configuration;
mod crates_io;
mod discovery;
mod index;
mod search;

pub(crate) use crates_io::CratesIoRegistry;
pub(crate) use discovery::ProviderCandidate;
pub(super) use discovery::{CandidateRejection, RegistryDiscoveryError, discover};

pub(crate) trait ProviderRegistry {
    fn search(&self, query: &str) -> Result<Vec<u8>, RegistryAccessError>;
    fn index(&self, crate_name: &str) -> Result<Vec<u8>, RegistryAccessError>;
    fn configuration(&self) -> Result<Vec<u8>, RegistryAccessError>;
    fn download(&self, url: &str) -> Result<Vec<u8>, RegistryAccessError>;
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{operation} failed: {reason}")]
pub(crate) struct RegistryAccessError {
    operation: String,
    reason: String,
}

impl RegistryAccessError {
    pub(super) fn new(operation: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            reason: reason.into(),
        }
    }
}
