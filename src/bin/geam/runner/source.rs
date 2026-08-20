use super::generator::render_source;
use crate::error::CliError;
use camino::Utf8Path;
use std::fs;

const RUNNER_SOURCE: &str = "build/geam/runner.rs";

pub(crate) fn reconcile_source(
    project_root: &Utf8Path,
    provider_aliases: &[String],
) -> Result<bool, CliError> {
    write_source(project_root, provider_aliases)
}

fn write_source(project_root: &Utf8Path, provider_aliases: &[String]) -> Result<bool, CliError> {
    let directory = project_root.join("build/geam");
    let path = project_root.join(RUNNER_SOURCE);
    let source = render_source(provider_aliases);
    create_runner_directory(&directory)?;
    match fs::read_to_string(&path) {
        Ok(current) if current == source => return Ok(false),
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(CliError::FileRead {
                path: path.clone(),
                error,
            });
        }
    }
    write_generated_source(&path, source)?;
    Ok(true)
}

fn create_runner_directory(path: &Utf8Path) -> Result<(), CliError> {
    fs::create_dir_all(path).map_err(|error| CliError::FileWrite {
        path: path.to_path_buf(),
        error,
    })
}

fn write_generated_source(path: &Utf8Path, source: String) -> Result<(), CliError> {
    fs::write(path, source).map_err(|error| CliError::FileWrite {
        path: path.to_path_buf(),
        error,
    })
}

#[cfg(test)]
mod tests {
    use super::{reconcile_source, write_generated_source};
    use crate::error::CliError;
    use camino::Utf8PathBuf;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn writes_only_changed_generated_sources() {
        let project = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let providers = ["geam_provider_images".to_owned()];

        assert!(reconcile_source(&root, &providers).expect("initial source should reconcile"));
        let source = fs::read_to_string(root.join("build/geam/runner.rs"))
            .expect("runner source should be readable");

        assert!(!reconcile_source(&root, &providers).expect("unchanged source should reconcile"));
        assert_eq!(
            fs::read_to_string(root.join("build/geam/runner.rs"))
                .expect("runner source should remain readable"),
            source,
        );
    }

    #[test]
    fn preserves_generated_source_filesystem_failures() {
        let blocked = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(blocked.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        fs::write(root.join("build"), "blocked").expect("blocking file should be written");
        let directory = root.join("build/geam");
        let expected_kind = fs::create_dir_all(&directory)
            .expect_err("blocking file should prevent directory creation")
            .kind();
        let error = reconcile_source(&root, &[]).expect_err("blocked runner directory should fail");
        assert!(matches!(
            error,
            CliError::FileWrite { path, error }
                if path == directory && error.kind() == expected_kind
        ));

        let unreadable = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(unreadable.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        fs::create_dir_all(root.join("build/geam/runner.rs"))
            .expect("blocking source directory should be created");
        let source = root.join("build/geam/runner.rs");
        let expected_kind = fs::read_to_string(&source)
            .expect_err("source directory should not be readable as a file")
            .kind();
        let error = reconcile_source(&root, &[]).expect_err("unreadable runner source should fail");
        assert!(matches!(
            error,
            CliError::FileRead { path, error }
                if path == source && error.kind() == expected_kind
        ));

        let destination = tempdir().expect("temporary destination should be created");
        let path = Utf8PathBuf::from_path_buf(destination.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let expected_kind = fs::write(&path, "source")
            .expect_err("directory destination should reject direct writes")
            .kind();
        let error = write_generated_source(&path, "source".to_owned())
            .expect_err("directory destination should fail");
        assert!(matches!(
            error,
            CliError::FileWrite { path: error_path, error }
                if error_path == path && error.kind() == expected_kind
        ));

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let blocked_write = tempdir().expect("temporary project should be created");
            let root = Utf8PathBuf::from_path_buf(blocked_write.path().to_path_buf())
                .expect("temporary path should be valid UTF-8");
            fs::create_dir_all(root.join("build/geam"))
                .expect("runner directory should be created");
            let directory = root.join("build/geam");
            let original = fs::metadata(&directory)
                .expect("runner directory metadata should be readable")
                .permissions();
            let mut restricted = original.clone();
            restricted.set_mode(0o500);
            fs::set_permissions(&directory, restricted)
                .expect("runner directory should become read-only");
            let result = reconcile_source(&root, &[]);
            fs::set_permissions(&directory, original)
                .expect("runner directory permissions should be restored");
            let error = result.expect_err("generated source write failure should be preserved");
            assert!(matches!(
                error,
                CliError::FileWrite { path, error }
                    if path == root.join("build/geam/runner.rs")
                        && error.kind() == std::io::ErrorKind::PermissionDenied
            ));
        }
    }
}
