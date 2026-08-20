use crate::error::CliError;
use crate::project::{compile_resolved_project, read_resolved_project};
use crate::provider::{ManagedProject, ProviderSelectionReconciler, SystemProviderReconciler};
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::BTreeMap;
use std::io::IsTerminal;

#[cfg(test)]
#[path = "standalone/integration.rs"]
mod integration;

pub(super) fn prepare(project_root: &Utf8Path, module: String) -> Result<(), CliError> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    let mut providers = SystemProviderReconciler::new(stdin.is_terminal(), &mut input, &mut output);
    prepare_with(
        project_root,
        module,
        &crate::runner::SystemCargo,
        &crate::runner::SystemCargo,
        &mut providers,
    )
}

pub(super) fn run(
    project_root: &Utf8Path,
    current_directory: &Utf8Path,
    module: String,
    configuration_specs: Vec<String>,
) -> Result<(), CliError> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    let mut providers = SystemProviderReconciler::new(stdin.is_terminal(), &mut input, &mut output);
    run_with(
        project_root,
        current_directory,
        module,
        configuration_specs,
        &crate::runner::SystemCargo,
        &crate::runner::SystemCargo,
        &mut providers,
    )
}

fn prepare_with(
    project_root: &Utf8Path,
    module: String,
    lock: &dyn crate::runner::CargoLock,
    checker: &dyn crate::runner::RunnerChecker,
    providers: &mut dyn ProviderSelectionReconciler,
) -> Result<(), CliError> {
    reconcile(project_root, &module, lock, providers)?;
    checker.check(project_root, &module)
}

fn run_with(
    project_root: &Utf8Path,
    current_directory: &Utf8Path,
    module: String,
    configuration_specs: Vec<String>,
    lock: &dyn crate::runner::CargoLock,
    executor: &dyn crate::runner::RunnerExecutor,
    providers: &mut dyn ProviderSelectionReconciler,
) -> Result<(), CliError> {
    let managed = reconcile(project_root, &module, lock, providers)?;
    let configurations =
        resolve_provider_configurations(current_directory, &managed, configuration_specs)?;
    executor.execute(project_root, &module, &configurations)
}

fn reconcile(
    project_root: &Utf8Path,
    module: &str,
    lock: &dyn crate::runner::CargoLock,
    providers: &mut dyn ProviderSelectionReconciler,
) -> Result<ManagedProject, CliError> {
    let project = read_resolved_project(project_root)?;
    let typed = compile_resolved_project(project_root, module.to_owned())?;
    let mut managed = ManagedProject::load(project_root, project.root_package())?;
    managed.retain_packages(&project.package_names());
    if managed.has_providers() {
        let manifest_changed = managed.write()?;
        crate::runner::reconcile_lock(project_root, manifest_changed, lock)?;
    }
    providers.reconcile(project_root, &project, &typed, &mut managed)?;
    crate::runner::reconcile_source(project_root, &managed.provider_aliases())?;
    let manifest_changed = managed.write()?;
    crate::runner::reconcile_lock(project_root, manifest_changed, lock)?;
    Ok(managed)
}

fn resolve_provider_configurations(
    current_directory: &Utf8Path,
    managed: &ManagedProject,
    specs: Vec<String>,
) -> Result<Vec<(String, Utf8PathBuf)>, CliError> {
    let mut configurations = BTreeMap::new();
    for spec in specs {
        let Some((package, path)) = spec.split_once('=') else {
            return Err(CliError::InvalidProviderConfiguration {
                spec,
                reason: "expected GLEAM_PACKAGE=PATH".to_owned(),
            });
        };
        if package.is_empty() || path.is_empty() {
            return Err(CliError::InvalidProviderConfiguration {
                spec,
                reason: "package and path must both be non-empty".to_owned(),
            });
        }
        if !managed.has_provider(package) {
            return Err(CliError::UnknownProviderConfiguration {
                package: package.to_owned(),
            });
        }
        let path = current_directory.join(path);
        if configurations.insert(package.to_owned(), path).is_some() {
            return Err(CliError::DuplicateProviderConfiguration {
                package: package.to_owned(),
            });
        }
    }
    Ok(configurations.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::resolve_provider_configurations;
    use crate::error::CliError;
    use crate::project::ResolvedProject;
    use crate::provider::{ManagedProject, ProviderSelectionReconciler};
    use crate::runner::{CargoLock, RunnerChecker, RunnerExecutor};
    use camino::{Utf8Path, Utf8PathBuf};
    use std::cell::{Cell, RefCell};
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

    impl RunnerExecutor for RecordingCargo {
        fn execute(
            &self,
            _project_root: &Utf8Path,
            module: &str,
            configurations: &[(String, Utf8PathBuf)],
        ) -> Result<(), CliError> {
            self.operations.borrow_mut().push(format!(
                "run:{module}:{}",
                configurations
                    .iter()
                    .map(|(package, path)| format!("{package}={path}"))
                    .collect::<Vec<_>>()
                    .join(","),
            ));
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

    struct FailingRun;

    impl RunnerExecutor for FailingRun {
        fn execute(
            &self,
            _project_root: &Utf8Path,
            _module: &str,
            _configurations: &[(String, Utf8PathBuf)],
        ) -> Result<(), CliError> {
            Err(CliError::InheritedProcessFailure {
                command: "cargo run".to_owned(),
                status: Some(1),
            })
        }
    }

    struct UnchangedProviders;

    impl ProviderSelectionReconciler for UnchangedProviders {
        fn reconcile(
            &mut self,
            _project_root: &Utf8Path,
            _project: &ResolvedProject,
            _program: &geam::TypedProgram,
            _managed: &mut ManagedProject,
        ) -> Result<(), CliError> {
            Ok(())
        }
    }

    struct LockedProviders<'test> {
        observed: &'test Cell<bool>,
    }

    impl ProviderSelectionReconciler for LockedProviders<'_> {
        fn reconcile(
            &mut self,
            project_root: &Utf8Path,
            _project: &ResolvedProject,
            _program: &geam::TypedProgram,
            managed: &mut ManagedProject,
        ) -> Result<(), CliError> {
            assert_eq!(
                fs::read_to_string(project_root.join("Cargo.lock"))
                    .expect("root lock should exist before provider resolution"),
                "fixture lock\n",
            );
            assert!(managed.has_provider("application"));
            assert!(!managed.has_provider("removed"));
            assert!(
                !fs::read_to_string(project_root.join("Cargo.toml"))
                    .expect("pruned managed manifest should be readable")
                    .contains("geam_provider_removed"),
            );
            self.observed.set(true);
            Ok(())
        }
    }

    fn prepare_with(
        project_root: &Utf8Path,
        module: String,
        lock: &dyn CargoLock,
        checker: &dyn RunnerChecker,
    ) -> Result<(), CliError> {
        super::prepare_with(project_root, module, lock, checker, &mut UnchangedProviders)
    }

    fn run_with(
        project_root: &Utf8Path,
        current_directory: &Utf8Path,
        module: String,
        configuration_specs: Vec<String>,
        lock: &dyn CargoLock,
        executor: &dyn RunnerExecutor,
    ) -> Result<(), CliError> {
        super::run_with(
            project_root,
            current_directory,
            module,
            configuration_specs,
            lock,
            executor,
            &mut UnchangedProviders,
        )
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
    fn restores_the_root_lock_after_pruning_before_resolving_approved_providers() {
        let project = project(
            "application",
            r#"
@external(erlang, "native", "required")
fn required() -> Int

pub fn main() { required() }
"#,
        );
        let root = utf8_path(&project);
        write_managed_manifest(
            &root,
            "geam_provider_application = { package = \"geam-application\", path = \"/application\" }\ngeam_provider_removed = { package = \"geam-removed\", path = \"/removed\" }\n",
        );
        let cargo = RecordingCargo::default();
        let observed = Cell::new(false);
        let mut providers = LockedProviders {
            observed: &observed,
        };

        super::prepare_with(
            &root,
            "application".to_owned(),
            &cargo,
            &cargo,
            &mut providers,
        )
        .expect("missing root lock should be restored before provider resolution");

        assert!(observed.get());
        assert_eq!(
            cargo.operations.borrow().as_slice(),
            ["lock", "check:application"],
        );
        assert_eq!(
            fs::read_to_string(root.join("Cargo.lock"))
                .expect("reconciled root lock should remain readable"),
            "fixture lock\n",
        );
    }

    #[cfg(unix)]
    #[test]
    fn preserves_pre_resolution_manifest_failures() {
        let project = project("application", "pub fn main() { 1 }\n");
        let root = utf8_path(&project);
        write_managed_manifest(
            &root,
            "geam_provider_application = { package = \"geam-application\", path = \"/application\" }\ngeam_provider_removed = { package = \"geam-removed\", path = \"/removed\" }\n",
        );
        fs::create_dir(root.join("Cargo.toml.geam.tmp"))
            .expect("temporary manifest blocker should be created");
        let cargo = RecordingCargo::default();
        let observed = Cell::new(false);
        let mut providers = LockedProviders {
            observed: &observed,
        };

        let error = super::prepare_with(
            &root,
            "application".to_owned(),
            &cargo,
            &cargo,
            &mut providers,
        )
        .expect_err("pruned manifest write failure should stop before provider resolution");

        assert!(matches!(
            error,
            CliError::FileWrite {
                ref path,
                error: _,
            } if path == &root.join("Cargo.toml.geam.tmp")
        ));
        assert!(matches!(
            error,
            CliError::FileWrite {
                path: _,
                ref error,
            } if error.kind() == std::io::ErrorKind::IsADirectory
        ));
        assert!(!observed.get());
        assert!(cargo.operations.borrow().is_empty());
        assert!(!root.join("Cargo.lock").exists());
    }

    #[test]
    fn preserves_pre_resolution_root_lock_failures() {
        let project = project("application", "pub fn main() { 1 }\n");
        let root = utf8_path(&project);
        write_managed_manifest(
            &root,
            "geam_provider_application = { package = \"geam-application\", path = \"/application\" }\n",
        );
        let observed = Cell::new(false);
        let mut providers = LockedProviders {
            observed: &observed,
        };

        let error = super::prepare_with(
            &root,
            "application".to_owned(),
            &FailingLock,
            &RecordingCargo::default(),
            &mut providers,
        )
        .expect_err("missing root lock failure should stop before provider resolution");

        assert!(matches!(
            error,
            CliError::ProcessFailure {
                ref command,
                status: Some(1),
                ref stderr,
            } if command == "cargo generate-lockfile" && stderr == "fixture lock failed"
        ));
        assert!(!observed.get());
        assert!(!root.join("Cargo.lock").exists());
    }

    #[test]
    fn runs_reconciled_projects_without_a_separate_check() {
        let project = project(
            "application",
            r#"
@external(erlang, "native", "required")
fn required() -> Int

pub fn main() { 1 }
"#,
        );
        let root = utf8_path(&project);
        write_managed_manifest(
            &root,
            "geam_provider_application = { package = \"geam-application\", path = \"/provider\" }\n",
        );
        let invocation = root.join("nested");
        fs::create_dir(&invocation).expect("invocation directory should be created");
        let cargo = RecordingCargo::default();

        run_with(
            &root,
            &invocation,
            "application".to_owned(),
            vec!["application=../config.toml".to_owned()],
            &cargo,
            &cargo,
        )
        .expect("configured standalone project should run");

        assert_eq!(
            cargo.operations.borrow().as_slice(),
            [
                "lock",
                &format!(
                    "run:application:application={}",
                    root.join("nested/../config.toml")
                ),
            ],
        );

        cargo.operations.borrow_mut().clear();
        let error = run_with(
            &root,
            &invocation,
            "application".to_owned(),
            vec!["application".to_owned()],
            &cargo,
            &cargo,
        )
        .expect_err("invalid configuration should stop before runner execution");
        assert_eq!(
            error.to_string(),
            "provider configuration application is invalid: expected GLEAM_PACKAGE=PATH",
        );
        assert!(cargo.operations.borrow().is_empty());
    }

    #[test]
    fn validates_provider_configuration_specs_before_execution() {
        let project = project("application", "pub fn main() { 1 }\n");
        let root = utf8_path(&project);
        write_managed_manifest(
            &root,
            "geam_provider_images = { package = \"geam-images\", path = \"/provider\" }\n",
        );
        let managed =
            ManagedProject::load(&root, "application").expect("managed project should load");

        let invalid = resolve_provider_configurations(&root, &managed, vec!["images".to_owned()])
            .expect_err("configuration without a path should fail");
        assert_eq!(
            invalid.to_string(),
            "provider configuration images is invalid: expected GLEAM_PACKAGE=PATH",
        );
        for spec in ["=config.toml", "images="] {
            assert_eq!(
                resolve_provider_configurations(&root, &managed, vec![spec.to_owned()],)
                    .expect_err("empty configuration part should fail")
                    .to_string(),
                format!(
                    "provider configuration {spec} is invalid: package and path must both be non-empty"
                ),
            );
        }
        assert_eq!(
            resolve_provider_configurations(
                &root,
                &managed,
                vec!["search=config.toml".to_owned()],
            )
            .expect_err("unknown provider configuration should fail")
            .to_string(),
            "no selected provider accepts configuration for Gleam package search",
        );
        assert_eq!(
            resolve_provider_configurations(
                &root,
                &managed,
                vec![
                    "images=first.toml".to_owned(),
                    "images=second.toml".to_owned(),
                ],
            )
            .expect_err("duplicate provider configuration should fail")
            .to_string(),
            "provider configuration for Gleam package images was supplied more than once",
        );
        assert_eq!(
            resolve_provider_configurations(
                &root,
                &managed,
                vec!["images=config=local.toml".to_owned()],
            )
            .expect("paths may contain equals signs"),
            [("images".to_owned(), root.join("config=local.toml"),)],
        );
    }

    #[test]
    fn preserves_generated_runner_execution_failures() {
        let project = project("application", "pub fn main() { 1 }\n");
        let root = utf8_path(&project);

        assert_eq!(
            run_with(
                &root,
                &root,
                "application".to_owned(),
                Vec::new(),
                &RecordingCargo::default(),
                &FailingRun,
            )
            .expect_err("runner failure should be preserved")
            .to_string(),
            "`cargo run` failed with status Some(1) after writing its output directly",
        );
    }

    #[test]
    fn preserves_provider_reconciliation_failures_before_writing_runner_inputs() {
        let project = project(
            "application",
            r#"
@external(erlang, "native", "required")
fn required() -> Int

pub fn main() { 1 }
"#,
        );
        let root = utf8_path(&project);

        struct FailingProviders;

        impl ProviderSelectionReconciler for FailingProviders {
            fn reconcile(
                &mut self,
                _project_root: &Utf8Path,
                _project: &ResolvedProject,
                _program: &geam::TypedProgram,
                _managed: &mut ManagedProject,
            ) -> Result<(), CliError> {
                Err(CliError::ProviderApprovalRequired {
                    package: "application".to_owned(),
                    command: "geam provider add geam-application@1.0.0".to_owned(),
                })
            }
        }

        assert_eq!(
            super::prepare_with(
                &root,
                "application".to_owned(),
                &RecordingCargo::default(),
                &RecordingCargo::default(),
                &mut FailingProviders,
            )
            .expect_err("provider reconciliation should fail")
            .to_string(),
            "Gleam package application requires native provider approval; run Geam interactively or select it explicitly with `geam provider add geam-application@1.0.0`",
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
