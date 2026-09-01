use crate::error::CliError;
use crate::process::run_checked_with_progress;
use crate::progress::Progress;
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::{Metadata, MetadataCommand};
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CargoMetadataMode {
    Workspace,
    Resolve,
    Locked,
}

pub(super) trait CargoMetadataLoader {
    fn load(
        &self,
        current_directory: &Utf8Path,
        manifest: &Utf8Path,
        mode: CargoMetadataMode,
        progress: &mut Progress<'_>,
    ) -> Result<Metadata, CliError>;
}

pub(super) struct SystemCargoMetadata;

impl CargoMetadataLoader for SystemCargoMetadata {
    fn load(
        &self,
        current_directory: &Utf8Path,
        manifest: &Utf8Path,
        mode: CargoMetadataMode,
        progress: &mut Progress<'_>,
    ) -> Result<Metadata, CliError> {
        let mut command = Command::new("cargo");
        command
            .arg("metadata")
            .arg("--format-version")
            .arg("1")
            .arg("--manifest-path")
            .arg(manifest)
            .current_dir(current_directory);
        match mode {
            CargoMetadataMode::Workspace => {
                command.arg("--no-deps");
            }
            CargoMetadataMode::Resolve => {}
            CargoMetadataMode::Locked => {
                command.arg("--locked");
            }
        }
        let output = run_checked_with_progress(&mut command, progress, Stdio::piped())?;
        parse_metadata_output(manifest, &output.stdout)
    }
}

pub(super) fn parse_metadata_output(
    manifest: &Utf8Path,
    output: &[u8],
) -> Result<Metadata, CliError> {
    MetadataCommand::parse(String::from_utf8_lossy(output)).map_err(|error| {
        CliError::InvalidCargoMetadata {
            manifest: manifest.to_path_buf(),
            reason: error.to_string(),
        }
    })
}

pub(super) fn canonical_manifest(path: Utf8PathBuf) -> Result<Utf8PathBuf, CliError> {
    std::fs::canonicalize(&path)
        .map_err(|error| CliError::FileRead {
            path: path.clone(),
            error,
        })
        .and_then(crate::project::into_utf8_path)
}

#[cfg(test)]
mod tests {
    use super::{
        CargoMetadataLoader, CargoMetadataMode, SystemCargoMetadata, canonical_manifest,
        parse_metadata_output,
    };
    use crate::error::CliError;
    use crate::progress::Progress;
    use camino::Utf8PathBuf;

    #[test]
    fn rejects_invalid_cargo_metadata_with_manifest_context() {
        let manifest = Utf8PathBuf::from("/workspace/Cargo.toml");
        let expected = cargo_metadata::MetadataCommand::parse("not JSON")
            .expect_err("invalid Cargo output fixture should be rejected")
            .to_string();
        let error = parse_metadata_output(&manifest, b"not JSON")
            .expect_err("invalid Cargo metadata should fail");

        assert!(matches!(
            error,
            CliError::InvalidCargoMetadata { manifest: path, reason }
                if path == manifest && reason == expected
        ));
    }

    #[test]
    fn preserves_missing_manifest_process_failure() {
        let root = tempfile::tempdir().expect("temporary directory should be created");
        let root = Utf8PathBuf::from_path_buf(root.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let manifest = root.join("missing/Cargo.toml");
        let error = SystemCargoMetadata
            .load(
                &root,
                &manifest,
                CargoMetadataMode::Locked,
                &mut Progress::Hidden,
            )
            .expect_err("missing manifest should fail");
        let expected_command =
            format!("cargo metadata --format-version 1 --manifest-path {manifest} --locked");

        assert!(matches!(
            error,
            CliError::ProcessFailure { command, status: Some(101), stderr }
                if command == expected_command && stderr.contains("manifest path")
        ));
    }

    #[test]
    fn preserves_canonical_manifest_filesystem_failures() {
        let directory = tempfile::tempdir().expect("temporary directory should be created");
        let path = Utf8PathBuf::from_path_buf(directory.path().join("Cargo.toml"))
            .expect("temporary path should be UTF-8");
        let error =
            canonical_manifest(path.clone()).expect_err("an absent canonical manifest should fail");
        assert!(matches!(error, CliError::FileRead { path: actual, error }
            if actual == path && error.kind() == std::io::ErrorKind::NotFound));
    }
}
