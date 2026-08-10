use crate::error::CliError;
use crate::process::run_checked;
use camino::{Utf8Path, Utf8PathBuf};
use geam::{ProjectError, TypedProgram, compile_typed_project};
use gleam_core::config::PackageConfig;
use gleam_core::manifest::Manifest;
use hexpm::version::Version;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::process::Command;

const CONFIG_FILE: &str = "gleam.toml";
const MANIFEST_FILE: &str = "manifest.toml";

pub(super) struct ResolvedProject {
    root_package: String,
    packages: BTreeMap<String, Version>,
}

impl ResolvedProject {
    pub(super) fn root_package(&self) -> &str {
        &self.root_package
    }

    pub(super) fn package_version(&self, package: &str) -> Option<&Version> {
        self.packages.get(package)
    }

    pub(super) fn package_names(&self) -> BTreeSet<String> {
        self.packages.keys().cloned().collect()
    }
}

pub(super) fn into_utf8_path(path: std::path::PathBuf) -> Result<Utf8PathBuf, CliError> {
    Utf8PathBuf::from_path_buf(path).map_err(CliError::NonUtf8Path)
}

pub(super) fn find_project_root(start: &Utf8Path) -> Result<Utf8PathBuf, CliError> {
    start
        .ancestors()
        .find(|directory| directory.join(CONFIG_FILE).is_file())
        .map(Utf8Path::to_path_buf)
        .ok_or_else(|| CliError::ProjectRootNotFound {
            start: start.to_path_buf(),
        })
}

pub(super) fn entry_module(
    project_root: &Utf8Path,
    requested: Option<String>,
) -> Result<String, CliError> {
    requested.map_or_else(
        || read_package_config(project_root).map(|config| config.name.to_string()),
        Ok,
    )
}

pub(super) fn read_resolved_project(project_root: &Utf8Path) -> Result<ResolvedProject, CliError> {
    read_resolved_project_with(project_root, &ProcessDependencyDownloader::gleam())
}

fn read_resolved_project_with(
    project_root: &Utf8Path,
    downloader: &dyn DependencyDownloader,
) -> Result<ResolvedProject, CliError> {
    match read_resolved_project_files(project_root) {
        Err(CliError::FileRead { path, error })
            if path == project_root.join(MANIFEST_FILE)
                && error.kind() == std::io::ErrorKind::NotFound =>
        {
            downloader.download(project_root)?;
            read_resolved_project_files(project_root)
        }
        result => result,
    }
}

fn read_resolved_project_files(project_root: &Utf8Path) -> Result<ResolvedProject, CliError> {
    let config = read_package_config(project_root)?;
    let manifest_path = project_root.join(MANIFEST_FILE);
    let manifest = read_toml::<Manifest>("Gleam manifest", &manifest_path)?;
    let mut packages = BTreeMap::from([(config.name.to_string(), config.version)]);
    for package in manifest.packages {
        if packages
            .insert(package.name.to_string(), package.version)
            .is_some()
        {
            return Err(CliError::InvalidToml {
                kind: "Gleam manifest",
                path: manifest_path,
                reason: format!("package {} is listed more than once", package.name),
            });
        }
    }
    Ok(ResolvedProject {
        root_package: config.name.to_string(),
        packages,
    })
}

pub(super) fn compile_resolved_project(
    project_root: &Utf8Path,
    root_module: String,
) -> Result<TypedProgram, CliError> {
    compile_resolved_project_with(
        project_root,
        root_module,
        &ProcessDependencyDownloader::gleam(),
    )
}

fn compile_resolved_project_with(
    project_root: &Utf8Path,
    root_module: String,
    downloader: &dyn DependencyDownloader,
) -> Result<TypedProgram, CliError> {
    match compile_typed_project(project_root, root_module.clone()) {
        Ok(program) => Ok(program),
        Err(error) if should_download_dependencies(&error) => {
            downloader.download(project_root)?;
            compile_typed_project(project_root, root_module).map_err(CliError::from)
        }
        Err(error) => Err(error.into()),
    }
}

fn should_download_dependencies(error: &ProjectError) -> bool {
    matches!(
        error,
        ProjectError::ManifestIo { error, .. } if error.kind() == std::io::ErrorKind::NotFound
    ) || matches!(error, ProjectError::MissingDownloadedPackage { .. })
}

trait DependencyDownloader {
    fn download(&self, project_root: &Utf8Path) -> Result<(), CliError>;
}

struct ProcessDependencyDownloader {
    program: OsString,
    arguments: Box<[OsString]>,
}

impl ProcessDependencyDownloader {
    fn gleam() -> Self {
        Self::new("gleam", ["deps", "download"])
    }

    fn new(
        program: impl Into<OsString>,
        arguments: impl IntoIterator<Item = impl Into<OsString>>,
    ) -> Self {
        Self {
            program: program.into(),
            arguments: arguments
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

impl DependencyDownloader for ProcessDependencyDownloader {
    fn download(&self, project_root: &Utf8Path) -> Result<(), CliError> {
        run_checked(
            Command::new(&self.program)
                .args(&self.arguments)
                .current_dir(project_root),
        )?;
        Ok(())
    }
}

fn read_package_config(project_root: &Utf8Path) -> Result<PackageConfig, CliError> {
    read_toml("Gleam package config", &project_root.join(CONFIG_FILE))
}

fn read_toml<Type: serde::de::DeserializeOwned>(
    kind: &'static str,
    path: &Utf8Path,
) -> Result<Type, CliError> {
    let source = fs::read_to_string(path).map_err(|error| CliError::FileRead {
        path: path.to_path_buf(),
        error,
    })?;
    toml::from_str(&source).map_err(|error| CliError::InvalidToml {
        kind,
        path: path.to_path_buf(),
        reason: error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::{
        DependencyDownloader, ProcessDependencyDownloader, compile_resolved_project,
        compile_resolved_project_with, entry_module, find_project_root, into_utf8_path,
        read_resolved_project, read_resolved_project_with, should_download_dependencies,
    };
    use crate::error::CliError;
    use camino::{Utf8Path, Utf8PathBuf};
    use geam::ProjectError;
    use std::cell::Cell;
    use std::fs;
    use tempfile::{TempDir, tempdir};

    struct RecordingDownloader {
        calls: Cell<usize>,
        manifest: &'static str,
    }

    impl DependencyDownloader for RecordingDownloader {
        fn download(&self, project_root: &Utf8Path) -> Result<(), CliError> {
            self.calls.set(self.calls.get() + 1);
            fs::write(project_root.join("manifest.toml"), self.manifest)
                .expect("recording downloader should write its fixture manifest");
            Ok(())
        }
    }

    struct FailingDownloader;

    impl DependencyDownloader for FailingDownloader {
        fn download(&self, _project_root: &Utf8Path) -> Result<(), CliError> {
            Err(CliError::ProcessFailure {
                command: "gleam deps download".to_owned(),
                status: Some(1),
                stderr: "fixture failure".to_owned(),
            })
        }
    }

    #[test]
    fn discovers_the_nearest_project_root() {
        let project = project("application", "1.2.3", "packages = []\n[requirements]\n");
        let nested = Utf8PathBuf::from_path_buf(project.path().join("one/two"))
            .expect("temporary path should be valid UTF-8");
        fs::create_dir_all(&nested).expect("nested directory should be created");

        assert_eq!(
            find_project_root(&nested).expect("project root should be found"),
            utf8_path(&project),
        );
    }

    #[test]
    fn rejects_missing_project_roots() {
        let directory = tempdir().expect("temporary directory should be created");
        let start = utf8_path(&directory);
        let error = find_project_root(&start).expect_err("project root should be absent");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::ProjectRootNotFound {
                start: Utf8PathBuf::new(),
            }),
        );
    }

    #[cfg(unix)]
    #[test]
    fn rejects_non_utf8_os_paths() {
        use std::os::unix::ffi::OsStringExt;

        let path = std::path::PathBuf::from(std::ffi::OsString::from_vec(vec![0xff]));
        let error = into_utf8_path(path.clone()).expect_err("non-UTF-8 path should fail");
        assert_eq!(
            error.to_string(),
            format!("path is not valid UTF-8: {path:?}")
        );
    }

    #[test]
    fn reads_entry_module_and_resolved_package_versions() {
        let project = project(
            "application",
            "1.2.3",
            r#"
packages = [
  { name = "images", version = "2.4.5", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" },
]

[requirements]
"#,
        );
        let root = utf8_path(&project);

        assert_eq!(
            entry_module(&root, None).expect("default entry should resolve"),
            "application",
        );
        assert_eq!(
            entry_module(&root, Some("worker".to_owned())).expect("explicit entry should resolve"),
            "worker",
        );
        let resolved = read_resolved_project(&root).expect("project should resolve");
        assert_eq!(resolved.root_package(), "application");
        assert_eq!(
            resolved
                .package_version("application")
                .map(ToString::to_string),
            Some("1.2.3".to_owned()),
        );
        assert_eq!(
            resolved.package_version("images").map(ToString::to_string),
            Some("2.4.5".to_owned()),
        );
        assert_eq!(resolved.package_version("missing"), None);
    }

    #[test]
    fn rejects_invalid_and_duplicate_project_metadata() {
        let missing = tempdir().expect("temporary directory should be created");
        let error = entry_module(&utf8_path(&missing), None)
            .expect_err("missing package config should be preserved");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileRead {
                path: Utf8PathBuf::new(),
                error: std::io::Error::new(std::io::ErrorKind::NotFound, ""),
            }),
        );

        let invalid = project("application", "not-a-version", "packages = []\n");
        let error = read_resolved_project(&utf8_path(&invalid))
            .err()
            .expect("invalid package config should be rejected");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::InvalidToml {
                kind: "Gleam package config",
                path: Utf8PathBuf::new(),
                reason: String::new(),
            }),
        );

        let invalid_manifest = project("application", "1.0.0", "invalid");
        let error = read_resolved_project(&utf8_path(&invalid_manifest))
            .err()
            .expect("invalid manifest should be rejected");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::InvalidToml {
                kind: "Gleam manifest",
                path: Utf8PathBuf::new(),
                reason: String::new(),
            }),
        );

        let duplicate = project(
            "application",
            "1.0.0",
            r#"
packages = [
  { name = "application", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "00" },
]
[requirements]
"#,
        );
        let error = read_resolved_project(&utf8_path(&duplicate))
            .err()
            .expect("duplicate package should be rejected");
        assert_eq!(
            error.to_string(),
            format!(
                "invalid Gleam manifest at {}: package application is listed more than once",
                utf8_path(&duplicate).join("manifest.toml"),
            ),
        );
    }

    #[test]
    fn retries_only_missing_resolution_inputs() {
        let project = project_without_manifest("application", "1.0.0");
        let downloader = RecordingDownloader {
            calls: Cell::new(0),
            manifest: "packages = []\n[requirements]\n",
        };
        let resolved = read_resolved_project_with(&utf8_path(&project), &downloader)
            .expect("missing resolved manifest should be downloaded once");
        assert_eq!(resolved.root_package(), "application");
        assert_eq!(downloader.calls.get(), 1);

        let project = project_without_manifest("application", "1.0.0");
        let downloader = RecordingDownloader {
            calls: Cell::new(0),
            manifest: "invalid",
        };
        let error = read_resolved_project_with(&utf8_path(&project), &downloader)
            .err()
            .expect("invalid downloaded resolution should be preserved");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::InvalidToml {
                kind: "Gleam manifest",
                path: Utf8PathBuf::new(),
                reason: String::new(),
            }),
        );
        assert_eq!(downloader.calls.get(), 1);

        let project = project_without_manifest("application", "1.0.0");
        let root = utf8_path(&project);
        let downloader = RecordingDownloader {
            calls: Cell::new(0),
            manifest: "packages = []\n[requirements]\n",
        };

        let program = compile_resolved_project_with(&root, "application".to_owned(), &downloader)
            .expect("missing manifest should be resolved once");
        assert_eq!(program.root_package(), "application");
        assert_eq!(downloader.calls.get(), 1);

        let project = project_without_manifest("application", "1.0.0");
        let error = read_resolved_project_with(&utf8_path(&project), &FailingDownloader)
            .err()
            .expect("resolved manifest download failure should be preserved");
        assert_eq!(
            error.to_string(),
            "`gleam deps download` failed with status Some(1): fixture failure",
        );

        let project = project_without_manifest("application", "1.0.0");
        let downloader = RecordingDownloader {
            calls: Cell::new(0),
            manifest: "invalid",
        };
        let error = compile_resolved_project_with(
            &utf8_path(&project),
            "application".to_owned(),
            &downloader,
        )
        .expect_err("invalid downloaded manifest should be preserved");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::Project(ProjectError::InvalidManifest {
                path: Utf8PathBuf::new(),
                reason: String::new(),
            })),
        );
        assert_eq!(downloader.calls.get(), 1);

        fs::write(root.join("manifest.toml"), "invalid").expect("manifest should be replaced");
        let error = compile_resolved_project_with(&root, "application".to_owned(), &downloader)
            .expect_err("invalid manifest should not trigger dependency download");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::Project(ProjectError::InvalidManifest {
                path: Utf8PathBuf::new(),
                reason: String::new(),
            })),
        );
        assert_eq!(downloader.calls.get(), 1);

        let project = project_without_manifest("application", "1.0.0");
        let error = compile_resolved_project_with(
            &utf8_path(&project),
            "application".to_owned(),
            &FailingDownloader,
        )
        .expect_err("dependency download failure should be preserved");
        assert_eq!(
            error.to_string(),
            "`gleam deps download` failed with status Some(1): fixture failure",
        );
    }

    #[test]
    fn recognizes_downloadable_project_errors() {
        assert!(should_download_dependencies(&ProjectError::ManifestIo {
            path: "manifest.toml".into(),
            error: std::io::Error::new(std::io::ErrorKind::NotFound, "missing"),
        }));
        assert!(should_download_dependencies(
            &ProjectError::MissingDownloadedPackage {
                package: "images".into(),
                path: "build/images".into(),
            }
        ));
        assert!(!should_download_dependencies(&ProjectError::ManifestIo {
            path: "manifest.toml".into(),
            error: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
        }));
    }

    #[test]
    fn runs_the_configured_dependency_downloader() {
        let project = project_without_manifest("application", "1.0.0");
        let downloader = ProcessDependencyDownloader::new("rustc", ["--version"]);
        downloader
            .download(&utf8_path(&project))
            .expect("configured process should run");

        let downloader = ProcessDependencyDownloader::new(
            "geam-command-that-does-not-exist",
            std::iter::empty::<&str>(),
        );
        let error = downloader
            .download(&utf8_path(&project))
            .expect_err("missing downloader process should be preserved");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::ProcessIo {
                command: String::new(),
                error: std::io::Error::new(std::io::ErrorKind::NotFound, ""),
            }),
        );
    }

    #[test]
    fn compiles_already_resolved_projects_without_downloading() {
        let project = project("application", "1.0.0", "packages = []\n[requirements]\n");
        let root = utf8_path(&project);

        let program = compile_resolved_project(&root, "application".to_owned())
            .expect("resolved project should compile directly");

        assert_eq!(program.root_module(), "application");
    }

    fn project(name: &str, version: &str, manifest: &str) -> TempDir {
        let project = project_without_manifest(name, version);
        fs::write(project.path().join("manifest.toml"), manifest)
            .expect("manifest should be written");
        project
    }

    fn project_without_manifest(name: &str, version: &str) -> TempDir {
        let project = tempdir().expect("temporary project should be created");
        fs::create_dir(project.path().join("src")).expect("source directory should be created");
        fs::write(
            project.path().join("gleam.toml"),
            format!("name = \"{name}\"\nversion = \"{version}\"\n"),
        )
        .expect("project config should be written");
        fs::write(
            project.path().join("src/application.gleam"),
            "pub fn main() { 1 }",
        )
        .expect("source should be written");
        project
    }

    fn utf8_path(project: &TempDir) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8")
    }
}
