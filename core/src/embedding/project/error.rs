use crate::{HostRegistrationError, ProjectError};
use thiserror::Error;

/// A failure while registering providers or loading a hosted Gleam project.
#[derive(Debug, Error)]
pub enum HostedProjectError {
    /// Static host provider registration failed.
    #[error(transparent)]
    HostRegistration(#[from] HostRegistrationError),

    /// The selected Gleam project failed to load or compile.
    #[error(transparent)]
    Project(#[from] ProjectError),
}
