use camino::Utf8PathBuf;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum CliError {
    #[error("failed to determine the current directory")]
    CurrentDirectory(#[source] io::Error),

    #[error("no gleam.toml was found at or above {start}")]
    ProjectRootNotFound { start: Utf8PathBuf },

    #[error("path is not valid UTF-8: {0:?}")]
    NonUtf8Path(std::path::PathBuf),

    #[error(transparent)]
    Project(#[from] geam_core::ProjectError),

    #[error("failed to read {path}")]
    FileRead {
        path: Utf8PathBuf,
        #[source]
        error: io::Error,
    },

    #[error("failed to write {path}")]
    FileWrite {
        path: Utf8PathBuf,
        #[source]
        error: io::Error,
    },

    #[error("failed to create a temporary provider candidate workspace")]
    TemporaryProviderWorkspace(#[source] io::Error),

    #[error("invalid {kind} at {path}: {reason}")]
    InvalidToml {
        kind: &'static str,
        path: Utf8PathBuf,
        reason: String,
    },

    #[error(
        "refusing to modify user-owned Cargo manifest {path}; use the manual embedding workflow tracked by #115"
    )]
    UserOwnedCargoManifest { path: Utf8PathBuf },

    #[error("managed Cargo manifest {path} uses unsupported runner schema {schema}")]
    UnsupportedRunnerSchema { path: Utf8PathBuf, schema: i64 },

    #[error("managed Cargo dependency {alias} is malformed: {reason}")]
    InvalidManagedDependency { alias: String, reason: String },

    #[error("provider for Gleam package {package} is already selected")]
    ProviderAlreadySelected { package: String },

    #[error("no provider is selected for Gleam package {package}")]
    ProviderNotSelected { package: String },

    #[error("Gleam package {package} is provided by Geam and cannot use an external provider")]
    BuiltInProviderPackage { package: String },

    #[error("provider targets Gleam package {package}, which is absent from the resolved project")]
    MissingGleamPackage { package: String },

    #[error("provider registry access for Gleam package {package} failed: {reason}")]
    ProviderRegistryAccess { package: String, reason: String },

    #[error("provider registry response for Gleam package {package} is unusable: {reason}")]
    ProviderRegistryProtocol { package: String, reason: String },

    #[error(
        "no metadata-verified provider is available for Gleam package {package} {version}: {details}"
    )]
    ProviderCandidatesUnavailable {
        package: String,
        version: String,
        details: String,
    },

    #[error(
        "Gleam package {package} requires native provider approval; run Geam interactively or select it explicitly with `{command}`"
    )]
    ProviderApprovalRequired { package: String, command: String },

    #[error(
        "provider selection for Gleam package {package} was cancelled; no provider selections were changed"
    )]
    ProviderApprovalCancelled { package: String },

    #[error("failed to {operation} provider approval prompt")]
    ProviderApprovalIo {
        operation: &'static str,
        #[source]
        error: io::Error,
    },

    #[error("failed to write provider list")]
    ProviderListIo(#[source] io::Error),

    #[error("provider configuration {spec} is invalid: {reason}")]
    InvalidProviderConfiguration { spec: String, reason: String },

    #[error("provider configuration for Gleam package {package} was supplied more than once")]
    DuplicateProviderConfiguration { package: String },

    #[error("no selected provider accepts configuration for Gleam package {package}")]
    UnknownProviderConfiguration { package: String },

    #[error("provider crate specification {spec} is invalid: {reason}")]
    InvalidCrateSpecification { spec: String, reason: String },

    #[error("provider path does not contain a Cargo manifest: {path}")]
    MissingProviderManifest { path: Utf8PathBuf },

    #[error(
        "provider workspace has multiple metadata-bearing packages; use --package with one of: {packages}"
    )]
    AmbiguousProviderPackage { packages: String },

    #[error("provider package {package} was not found in Cargo metadata")]
    MissingProviderPackage { package: String },

    #[error("crate {package} has invalid Geam provider metadata: {reason}")]
    InvalidProviderMetadata { package: String, reason: String },

    #[error(
        "provider {provider} targets {package} {version}, which is outside its Gleam range {range}"
    )]
    IncompatibleProvider {
        provider: String,
        package: String,
        version: String,
        range: String,
    },

    #[error("failed to start `{command}`")]
    ProcessIo {
        command: String,
        #[source]
        error: io::Error,
    },

    #[error("`{command}` failed with status {status:?}: {stderr}")]
    ProcessFailure {
        command: String,
        status: Option<i32>,
        stderr: String,
    },

    #[error("`{command}` failed with status {status:?} after writing its output directly")]
    InheritedProcessFailure {
        command: String,
        status: Option<i32>,
    },

    #[error("Cargo metadata did not contain the resolved dependency {alias}")]
    MissingResolvedDependency { alias: String },

    #[error("Cargo returned invalid metadata for {manifest}: {reason}")]
    InvalidCargoMetadata {
        manifest: Utf8PathBuf,
        reason: String,
    },
}
