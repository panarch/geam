use crate::error::CliError;
use camino::Utf8Path;
use std::fs;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SyncOutcome {
    Unchanged,
    Updated,
}

pub(super) fn sync(
    directory: &Utf8Path,
    destination: &Utf8Path,
    expected: &[u8],
) -> Result<SyncOutcome, CliError> {
    match fs::read(destination) {
        Ok(current) if current == expected => return Ok(SyncOutcome::Unchanged),
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(CliError::FileRead {
                path: destination.to_path_buf(),
                error,
            });
        }
    }

    let mut temporary =
        tempfile::NamedTempFile::new_in(directory).map_err(|error| CliError::FileWrite {
            path: destination.to_path_buf(),
            error,
        })?;
    write_expected(&mut temporary, destination, expected).and_then(|()| {
        temporary
            .persist(destination)
            .map(|_| ())
            .map_err(|error| CliError::FileWrite {
                path: destination.to_path_buf(),
                error: error.error,
            })
    })?;
    Ok(SyncOutcome::Updated)
}

fn write_expected(
    writer: &mut dyn Write,
    destination: &Utf8Path,
    expected: &[u8],
) -> Result<(), CliError> {
    writer
        .write_all(expected)
        .and_then(|()| writer.flush())
        .map_err(|error| CliError::FileWrite {
            path: destination.to_path_buf(),
            error,
        })
}

#[cfg(test)]
mod tests {
    use super::{SyncOutcome, sync, write_expected};
    use crate::error::CliError;
    use camino::Utf8PathBuf;
    use std::fs;
    use std::io::{self, Write};
    use tempfile::tempdir;

    #[test]
    fn leaves_identical_output_untouched_and_atomically_replaces_changes() {
        let directory = tempdir().expect("temporary directory should be created");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let destination = root.join("geam_bindings.rs");
        fs::write(&destination, "same").expect("fixture output should be written");
        #[cfg(unix)]
        let original_inode = std::os::unix::fs::MetadataExt::ino(
            &fs::metadata(&destination).expect("fixture metadata should be readable"),
        );

        assert_eq!(
            sync(&root, &destination, b"same").expect("identical output should succeed"),
            SyncOutcome::Unchanged,
        );
        #[cfg(unix)]
        assert_eq!(
            std::os::unix::fs::MetadataExt::ino(
                &fs::metadata(&destination).expect("unchanged metadata should be readable"),
            ),
            original_inode,
        );

        assert_eq!(
            sync(&root, &destination, b"changed").expect("changed output should succeed"),
            SyncOutcome::Updated,
        );
        assert_eq!(
            fs::read(&destination).expect("updated output should be readable"),
            b"changed",
        );
        #[cfg(unix)]
        assert_ne!(
            std::os::unix::fs::MetadataExt::ino(
                &fs::metadata(&destination).expect("updated metadata should be readable"),
            ),
            original_inode,
        );
    }

    #[test]
    fn preserves_read_failures_with_destination_context() {
        let directory = tempdir().expect("temporary directory should be created");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let destination = root.join("geam_bindings.rs");
        fs::create_dir(&destination).expect("directory fixture should be created");

        let error =
            sync(&root, &destination, b"expected").expect_err("unreadable destination should fail");
        assert!(matches!(
            error,
            CliError::FileRead { path, .. } if path == destination
        ));
        assert!(destination.is_dir());
    }

    #[test]
    fn preserves_write_failures_with_destination_context() {
        let directory = tempdir().expect("temporary directory should be created");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let missing_directory = root.join("missing");
        let destination = missing_directory.join("geam_bindings.rs");

        let error = sync(&missing_directory, &destination, b"expected")
            .expect_err("missing output directory should fail");
        assert!(matches!(
            error,
            CliError::FileWrite { path, error }
                if path == destination && error.kind() == std::io::ErrorKind::NotFound
        ));
        assert!(!destination.exists());
    }

    #[test]
    fn preserves_content_write_and_flush_failures_with_destination_context() {
        enum Failure {
            Write,
            Flush,
        }

        struct FailingWriter(Failure);

        impl Write for FailingWriter {
            fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
                match self.0 {
                    Failure::Write => Err(io::Error::other("fixture write failure")),
                    Failure::Flush => Ok(buffer.len()),
                }
            }

            fn flush(&mut self) -> io::Result<()> {
                Err(io::Error::other("fixture flush failure"))
            }
        }

        let destination = Utf8PathBuf::from("/workspace/src/geam_bindings.rs");
        for failure in [Failure::Write, Failure::Flush] {
            let error = write_expected(&mut FailingWriter(failure), &destination, b"expected")
                .expect_err("content output failure should be preserved");
            assert!(matches!(
                error,
                CliError::FileWrite { path, error }
                    if path == destination && error.kind() == io::ErrorKind::Other
            ));
        }
    }

    #[test]
    fn preserves_atomic_persist_failures_with_destination_context() {
        let directory = tempdir().expect("temporary directory should be created");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let destination = root.join("missing/geam_bindings.rs");

        let error = sync(&root, &destination, b"expected")
            .expect_err("missing destination parent should reject persistence");
        assert!(matches!(
            error,
            CliError::FileWrite { path, error }
                if path == destination && error.kind() == io::ErrorKind::NotFound
        ));
        assert!(!destination.exists());
    }

    #[cfg(unix)]
    #[test]
    fn preserves_previous_output_when_atomic_replacement_cannot_start() {
        use std::os::unix::fs::PermissionsExt;

        let directory = tempdir().expect("temporary directory should be created");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let destination = root.join("geam_bindings.rs");
        fs::write(&destination, "previous").expect("previous output should be written");
        fs::set_permissions(&root, fs::Permissions::from_mode(0o500))
            .expect("output directory should become read-only");

        let result = sync(&root, &destination, b"changed");
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))
            .expect("output directory permissions should be restored");

        let error = result.expect_err("read-only output directory should reject replacement");
        assert!(matches!(
            error,
            CliError::FileWrite { path, error }
                if path == destination
                    && error.kind() == std::io::ErrorKind::PermissionDenied
        ));
        assert_eq!(
            fs::read(&destination).expect("previous output should remain readable"),
            b"previous",
        );
    }
}
