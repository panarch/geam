use crate::error::CliError;
use crate::project::{compile_resolved_project, read_resolved_project};
use crate::provider::{ManagedProject, is_built_in_package};
use camino::Utf8Path;
use geam::required_host_functions;
use std::collections::BTreeSet;

pub(super) fn prepare(project_root: &Utf8Path, module: String) -> Result<(), CliError> {
    prepare_with(
        project_root,
        module,
        &crate::runner::SystemCargo,
        &crate::runner::SystemCargo,
    )
}

fn prepare_with(
    project_root: &Utf8Path,
    module: String,
    lock: &dyn crate::runner::CargoLock,
    checker: &dyn crate::runner::RunnerChecker,
) -> Result<(), CliError> {
    let project = read_resolved_project(project_root)?;
    let typed = compile_resolved_project(project_root, module.clone())?;
    let mut managed = ManagedProject::load(project_root, project.root_package())?;
    managed.retain_packages(&project.package_names());
    validate_required_providers(&typed, &managed)?;
    crate::runner::reconcile_source(project_root, &managed.provider_aliases())?;
    let manifest_changed = managed.write()?;
    crate::runner::reconcile_lock(project_root, manifest_changed, lock)?;
    checker.check(project_root, &module)
}

fn validate_required_providers(
    program: &geam::TypedProgram,
    managed: &ManagedProject,
) -> Result<(), CliError> {
    let packages = required_host_functions(program)
        .into_iter()
        .map(|requirement| requirement.package().to_string())
        .collect::<BTreeSet<_>>();
    for package in packages {
        if !is_built_in_package(&package) && !managed.has_provider(&package) {
            return Err(CliError::MissingProviderSelection { package });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::prepare_with;
    use crate::error::CliError;
    use crate::runner::{CargoLock, RunnerChecker};
    use camino::{Utf8Path, Utf8PathBuf};
    use std::cell::RefCell;
    use std::fs;
    use tempfile::{TempDir, tempdir};

    const MANAGED_HEADER: &str =
        "# Managed by Geam. Use `geam provider` commands to change providers.\n";

    #[derive(Default)]
    struct RecordingCargo {
        operations: RefCell<Vec<String>>,
    }

    impl CargoLock for RecordingCargo {
        fn generate_lockfile(&self, project_root: &Utf8Path) -> Result<(), CliError> {
            self.operations.borrow_mut().push("lock".to_owned());
            fs::write(project_root.join("Cargo.lock"), "fixture lock\n")
                .expect("fixture lock should be written");
            Ok(())
        }
    }

    impl RunnerChecker for RecordingCargo {
        fn check(&self, _project_root: &Utf8Path, module: &str) -> Result<(), CliError> {
            self.operations.borrow_mut().push(format!("check:{module}"));
            Ok(())
        }
    }

    struct FailingCheck;

    impl CargoLock for FailingCheck {
        fn generate_lockfile(&self, project_root: &Utf8Path) -> Result<(), CliError> {
            fs::write(project_root.join("Cargo.lock"), "fixture lock\n")
                .expect("fixture lock should be written");
            Ok(())
        }
    }

    impl RunnerChecker for FailingCheck {
        fn check(&self, _project_root: &Utf8Path, _module: &str) -> Result<(), CliError> {
            Err(CliError::ProcessFailure {
                command: "cargo run".to_owned(),
                status: Some(1),
                stderr: "fixture check failed".to_owned(),
            })
        }
    }

    struct FailingLock;

    impl CargoLock for FailingLock {
        fn generate_lockfile(&self, _project_root: &Utf8Path) -> Result<(), CliError> {
            Err(CliError::ProcessFailure {
                command: "cargo generate-lockfile".to_owned(),
                status: Some(1),
                stderr: "fixture lock failed".to_owned(),
            })
        }
    }

    #[test]
    fn prepares_pure_projects_and_reuses_unchanged_runner_inputs() {
        let project = project("application", "pub fn main() { 1 }\n");
        let root = utf8_path(&project);
        let cargo = RecordingCargo::default();

        prepare_with(&root, "application".to_owned(), &cargo, &cargo)
            .expect("pure project should prepare");
        assert_eq!(
            cargo.operations.borrow().as_slice(),
            ["lock", "check:application"],
        );
        let manifest = fs::read_to_string(root.join("Cargo.toml"))
            .expect("managed manifest should be readable");
        let source = fs::read_to_string(root.join("build/geam/runner.rs"))
            .expect("runner source should be readable");

        cargo.operations.borrow_mut().clear();
        prepare_with(&root, "application".to_owned(), &cargo, &cargo)
            .expect("repeated prepare should succeed");
        assert_eq!(cargo.operations.borrow().as_slice(), ["check:application"]);
        assert_eq!(
            fs::read_to_string(root.join("Cargo.toml"))
                .expect("managed manifest should remain readable"),
            manifest,
        );
        assert_eq!(
            fs::read_to_string(root.join("build/geam/runner.rs"))
                .expect("runner source should remain readable"),
            source,
        );
    }

    #[test]
    fn requires_explicit_selection_for_non_builtin_host_functions() {
        let project = project(
            "application",
            r#"
@external(erlang, "native", "required")
fn required() -> Int

pub fn main() { 1 }
"#,
        );
        let root = utf8_path(&project);

        assert_eq!(
            prepare_with(
                &root,
                "application".to_owned(),
                &RecordingCargo::default(),
                &RecordingCargo::default(),
            )
            .expect_err("unselected native package should fail")
            .to_string(),
            "Gleam package application requires a host provider; select one with `geam provider add geam-application`",
        );
        assert!(!root.join("Cargo.toml").exists());
    }

    #[test]
    fn accepts_builtin_and_explicit_provider_packages() {
        let builtin = project(
            "gleam_json",
            r#"
@external(erlang, "native", "required")
fn required() -> Int

pub fn main() { 1 }
"#,
        );
        prepare_with(
            &utf8_path(&builtin),
            "gleam_json".to_owned(),
            &RecordingCargo::default(),
            &RecordingCargo::default(),
        )
        .expect("built-in package should not require external selection");

        let explicit = project(
            "application",
            r#"
@external(erlang, "native", "required")
fn required() -> Int

pub fn main() { 1 }
"#,
        );
        let root = utf8_path(&explicit);
        write_managed_manifest(
            &root,
            "geam_provider_application = { package = \"geam-application\", path = \"/provider\" }\n",
        );
        prepare_with(
            &root,
            "application".to_owned(),
            &RecordingCargo::default(),
            &RecordingCargo::default(),
        )
        .expect("explicit provider package should prepare");
        assert!(
            fs::read_to_string(root.join("build/geam/runner.rs"))
                .expect("runner source should be readable")
                .contains("geam_provider_application::Component"),
        );
    }

    #[test]
    fn removes_provider_selections_absent_from_the_resolved_project() {
        let project = project("application", "pub fn main() { 1 }\n");
        let root = utf8_path(&project);
        write_managed_manifest(
            &root,
            "geam_provider_images = { package = \"geam-images\", version = \"=1.0.0\" }\n",
        );

        prepare_with(
            &root,
            "application".to_owned(),
            &RecordingCargo::default(),
            &RecordingCargo::default(),
        )
        .expect("stale selection should be removed");

        assert!(
            !fs::read_to_string(root.join("Cargo.toml"))
                .expect("managed manifest should be readable")
                .contains("geam_provider_images")
        );
        assert!(
            !fs::read_to_string(root.join("build/geam/runner.rs"))
                .expect("runner source should be readable")
                .contains("geam_provider_images")
        );
    }

    #[test]
    fn preserves_generated_runner_check_failures() {
        let project = project("application", "pub fn main() { 1 }\n");
        let root = utf8_path(&project);

        assert_eq!(
            prepare_with(
                &root,
                "application".to_owned(),
                &FailingCheck,
                &FailingCheck
            )
            .expect_err("runner check failure should be preserved")
            .to_string(),
            "`cargo run` failed with status Some(1): fixture check failed",
        );
    }

    #[test]
    fn preserves_each_preparation_phase_failure_at_its_owner() {
        let invalid_manifest = project("application", "pub fn main() { 1 }\n");
        fs::write(invalid_manifest.path().join("manifest.toml"), "invalid")
            .expect("invalid manifest should be written");
        let error = prepare_with(
            &utf8_path(&invalid_manifest),
            "application".to_owned(),
            &RecordingCargo::default(),
            &RecordingCargo::default(),
        )
        .expect_err("invalid resolved project should stop preparation");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::InvalidToml {
                kind: "",
                path: Utf8PathBuf::new(),
                reason: String::new(),
            }),
        );

        let invalid_source = project("application", "pub fn main( {\n");
        let error = prepare_with(
            &utf8_path(&invalid_source),
            "application".to_owned(),
            &RecordingCargo::default(),
            &RecordingCargo::default(),
        )
        .expect_err("invalid source should stop preparation");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::Project(geam::ProjectError::SourceIo {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            })),
        );

        let user_manifest = project("application", "pub fn main() { 1 }\n");
        fs::write(user_manifest.path().join("Cargo.toml"), "[workspace]\n")
            .expect("user Cargo manifest should be written");
        let error = prepare_with(
            &utf8_path(&user_manifest),
            "application".to_owned(),
            &RecordingCargo::default(),
            &RecordingCargo::default(),
        )
        .expect_err("user Cargo ownership should stop preparation");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::UserOwnedCargoManifest {
                path: Utf8PathBuf::new(),
            }),
        );

        let blocked_source = project("application", "pub fn main() { 1 }\n");
        fs::write(blocked_source.path().join("build"), "blocked")
            .expect("blocking build file should be written");
        let error = prepare_with(
            &utf8_path(&blocked_source),
            "application".to_owned(),
            &RecordingCargo::default(),
            &RecordingCargo::default(),
        )
        .expect_err("runner source failure should stop preparation");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );
        assert!(!blocked_source.path().join("Cargo.toml").exists());

        let blocked_manifest = project("application", "pub fn main() { 1 }\n");
        fs::create_dir(blocked_manifest.path().join("Cargo.toml.geam.tmp"))
            .expect("blocking manifest directory should be created");
        let error = prepare_with(
            &utf8_path(&blocked_manifest),
            "application".to_owned(),
            &RecordingCargo::default(),
            &RecordingCargo::default(),
        )
        .expect_err("manifest failure should stop preparation");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );
        assert!(!blocked_manifest.path().join("Cargo.toml").exists());

        let failed_lock = project("application", "pub fn main() { 1 }\n");
        let root = utf8_path(&failed_lock);
        let error = prepare_with(
            &root,
            "application".to_owned(),
            &FailingLock,
            &RecordingCargo::default(),
        )
        .expect_err("lock failure should stop preparation");
        assert_eq!(
            error.to_string(),
            "`cargo generate-lockfile` failed with status Some(1): fixture lock failed",
        );
        assert!(root.join("Cargo.toml").is_file());
        assert!(!root.join("Cargo.lock").exists());
    }

    fn project(package: &str, source: &str) -> TempDir {
        let project = tempdir().expect("temporary project should be created");
        fs::create_dir(project.path().join("src")).expect("source directory should be created");
        fs::write(
            project.path().join("gleam.toml"),
            format!("name = \"{package}\"\nversion = \"1.0.0\"\n"),
        )
        .expect("package config should be written");
        fs::write(
            project.path().join("manifest.toml"),
            "packages = []\n[requirements]\n",
        )
        .expect("manifest should be written");
        fs::write(project.path().join(format!("src/{package}.gleam")), source)
            .expect("source should be written");
        project
    }

    fn write_managed_manifest(root: &Utf8Path, provider: &str) {
        fs::write(
            root.join("Cargo.toml"),
            format!(
                "{MANAGED_HEADER}\n[package]\nname = \"application-geam-runner\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[package.metadata.geam.runner]\nschema = 1\n\n[[bin]]\nname = \"geam-runner\"\npath = \"build/geam/runner.rs\"\n\n[dependencies]\ngeam = \"={}\"\ntoml = \"0.9\"\n{provider}\n[workspace]\nresolver = \"3\"\n",
                env!("CARGO_PKG_VERSION"),
            ),
        )
        .expect("managed manifest should be written");
    }

    fn utf8_path(directory: &TempDir) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("temporary path should be valid UTF-8")
    }
}
