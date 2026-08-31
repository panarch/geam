use super::package::EmbeddingProject;
use crate::error::CliError;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use std::io::Write;

pub(super) fn initialize(project: &EmbeddingProject) -> Result<(), CliError> {
    let config = project.project_root().join("gleam.toml");
    match fs::read(&config) {
        Ok(_) => return project.validate_gleam_config(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(CliError::FileRead {
                path: config,
                error,
            });
        }
    }
    let source_directory = project.project_root().join("src");
    let source = source_directory.join(format!("{}.gleam", project.root_module()));
    let files = [
        (
            project.project_root().join(".gitignore"),
            "/build/\n".to_owned(),
        ),
        (
            source,
            "pub fn double(value: Int) -> Int {\n  value * 2\n}\n".to_owned(),
        ),
        (
            config,
            format!(
                "name = \"{}\"\nversion = \"0.1.0\"\ntarget = \"erlang\"\n",
                project.root_module()
            ),
        ),
    ];
    let mut missing = Vec::new();
    for (path, expected) in files {
        match fs::read(&path) {
            Ok(current) if current == expected.as_bytes() => {}
            Ok(_) => return Err(CliError::EmbeddingFileConflict {
                path,
                reason: "initialization would replace an existing file; keep a valid conventional Gleam project instead".to_owned(),
            }),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => missing.push((path, expected)),
            Err(error) => return Err(CliError::FileRead { path, error }),
        }
    }
    fs::create_dir_all(&source_directory).map_err(|error| CliError::FileWrite {
        path: source_directory,
        error,
    })?;
    create_files(missing)
}

fn create_files(files: Vec<(Utf8PathBuf, String)>) -> Result<(), CliError> {
    for (path, source) in files {
        create_file(&path, source.as_bytes())?;
    }
    Ok(())
}

fn create_file(path: &Utf8Path, source: &[u8]) -> Result<(), CliError> {
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .and_then(|mut file| file.write_all(source))
        .map_err(|error| CliError::FileWrite {
            path: path.to_path_buf(),
            error,
        })
}

#[cfg(test)]
mod tests {
    use super::{create_files, initialize};
    use crate::embedding::package::EmbeddingProject;
    use crate::error::CliError;
    use camino::Utf8PathBuf;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn preserves_initialization_io_failures_and_completes_partial_starters() {
        let directory = tempdir().expect("fixture directory");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 fixture");
        fs::create_dir(root.join("src")).expect("Rust source directory");
        fs::write(root.join("src/lib.rs"), "").expect("Rust source");
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = 'retry'\nversion = '0.1.0'\n[workspace]\n",
        )
        .expect("Cargo manifest");
        let project = EmbeddingProject::load(&root).expect("select project");
        fs::create_dir_all(root.join("gleam/gleam.toml")).expect("conflicting config directory");
        assert!(
            matches!(initialize(&project), Err(CliError::FileRead { path, .. }) if path == project.project_root().join("gleam.toml"))
        );
        fs::remove_dir(root.join("gleam/gleam.toml")).expect("remove conflicting directory");
        fs::create_dir(root.join("gleam/.gitignore")).expect("conflicting ignore directory");
        assert!(
            matches!(initialize(&project), Err(CliError::FileRead { path, .. }) if path == project.project_root().join(".gitignore"))
        );
        fs::remove_dir(root.join("gleam/.gitignore")).expect("remove conflicting directory");
        fs::write(root.join("gleam/.gitignore"), "/build/\n").expect("partial starter ignore rule");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(root.join("gleam"), fs::Permissions::from_mode(0o500))
                .expect("read-only source directory");
            let error = initialize(&project).expect_err("source directory creation should fail");
            fs::set_permissions(root.join("gleam"), fs::Permissions::from_mode(0o700))
                .expect("restore source permissions");
            assert!(
                matches!(error, CliError::FileWrite { path, .. } if path == project.project_root().join("src"))
            );
            assert!(!root.join("gleam/gleam.toml").exists());
        }
        initialize(&project).expect("complete partial starter");
        assert_eq!(
            fs::read_to_string(root.join("gleam/src/retry.gleam")).expect("completed source"),
            "pub fn double(value: Int) -> Int {\n  value * 2\n}\n"
        );
        assert!(root.join("gleam/gleam.toml").is_file());
    }

    #[test]
    fn creates_a_pure_conventional_library_and_keeps_handwritten_source_on_repeat() {
        let directory = tempdir().expect("fixture directory");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 fixture");
        fs::create_dir(root.join("src")).expect("Rust source directory");
        fs::write(root.join("src/lib.rs"), "// application\n").expect("Rust source");
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = 'initial-app'\nversion = '0.1.0'\n[workspace]\n",
        )
        .expect("Cargo manifest");
        let project = EmbeddingProject::load(&root).expect("select project");
        initialize(&project).expect("initialize project");
        assert_eq!(
            fs::read_to_string(root.join("gleam/gleam.toml")).expect("Gleam config"),
            "name = \"initial_app\"\nversion = \"0.1.0\"\ntarget = \"erlang\"\n"
        );
        assert_eq!(
            fs::read_to_string(root.join("gleam/.gitignore")).expect("ignore rule"),
            "/build/\n"
        );
        let source = root.join("gleam/src/initial_app.gleam");
        assert_eq!(
            fs::read_to_string(&source).expect("starter"),
            "pub fn double(value: Int) -> Int {\n  value * 2\n}\n"
        );
        fs::write(&source, "pub fn changed() { 42 }\n").expect("user source");
        initialize(&project).expect("existing valid project");
        assert_eq!(
            fs::read_to_string(&source).expect("preserved source"),
            "pub fn changed() { 42 }\n"
        );
        assert_eq!(
            fs::read_to_string(root.join("src/lib.rs")).expect("Rust source"),
            "// application\n"
        );
        fs::write(root.join("gleam/gleam.toml"), "name = 'other'\n").expect("conflicting config");
        assert_eq!(
            initialize(&project)
                .expect_err("conflicting project name")
                .to_string(),
            format!(
                "invalid Rust embedding project for package initial-app at {}: {}/gleam.toml declares Gleam package `other`; expected `initial_app` from the Cargo package name",
                project.manifest(),
                project.project_root()
            )
        );
    }

    #[test]
    fn rejects_conflicting_initial_files_without_creating_a_configuration() {
        let directory = tempdir().expect("fixture directory");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).expect("UTF-8 fixture");
        fs::create_dir(root.join("src")).expect("Rust source directory");
        fs::write(root.join("src/lib.rs"), "").expect("Rust source");
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = 'conflict'\nversion = '0.1.0'\n[workspace]\n",
        )
        .expect("Cargo manifest");
        fs::create_dir_all(root.join("gleam/src")).expect("Gleam directory");
        fs::write(root.join("gleam/src/conflict.gleam"), "// mine\n").expect("existing source");
        let project = EmbeddingProject::load(&root).expect("select project");
        let error = initialize(&project).expect_err("existing source conflict");
        assert_eq!(
            error.to_string(),
            format!(
                "refusing to replace existing embedding file {}: initialization would replace an existing file; keep a valid conventional Gleam project instead",
                project.project_root().join("src/conflict.gleam")
            )
        );
        assert!(!root.join("gleam/gleam.toml").exists());
        assert_eq!(
            fs::read_to_string(root.join("gleam/src/conflict.gleam")).expect("source remains"),
            "// mine\n"
        );
        assert!(
            matches!(create_files(vec![(root.join("src/lib.rs"), "replacement".to_owned())]), Err(CliError::FileWrite { error, .. }) if error.kind() == std::io::ErrorKind::AlreadyExists)
        );
    }
}
