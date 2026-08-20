use crate::error::CliError;
use crate::process::{run_checked, run_inherited};
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use std::process::Command;

const TARGET_DIRECTORY: &str = "build/geam/target";

pub(crate) trait CargoLock {
    fn generate_lockfile(&self, project_root: &Utf8Path) -> Result<(), CliError>;
}

pub(crate) trait RunnerChecker {
    fn check(&self, project_root: &Utf8Path, module: &str) -> Result<(), CliError>;
}

pub(crate) trait RunnerExecutor {
    fn execute(
        &self,
        project_root: &Utf8Path,
        module: &str,
        configurations: &[(String, Utf8PathBuf)],
    ) -> Result<(), CliError>;
}

pub(crate) struct SystemCargo;

impl CargoLock for SystemCargo {
    fn generate_lockfile(&self, project_root: &Utf8Path) -> Result<(), CliError> {
        finish_process(run_checked(
            Command::new("cargo")
                .arg("generate-lockfile")
                .arg("--manifest-path")
                .arg(project_root.join("Cargo.toml"))
                .current_dir(project_root)
                .env("CARGO_TARGET_DIR", project_root.join(TARGET_DIRECTORY)),
        ))
    }
}

impl RunnerChecker for SystemCargo {
    fn check(&self, project_root: &Utf8Path, module: &str) -> Result<(), CliError> {
        finish_process(run_checked(&mut runner_command(
            project_root,
            "check",
            module,
        )))
    }
}

impl RunnerExecutor for SystemCargo {
    fn execute(
        &self,
        project_root: &Utf8Path,
        module: &str,
        configurations: &[(String, Utf8PathBuf)],
    ) -> Result<(), CliError> {
        run_inherited(&mut execution_command(project_root, module, configurations))
    }
}

fn finish_process(result: Result<std::process::Output, CliError>) -> Result<(), CliError> {
    result.map(drop)
}

fn runner_command(project_root: &Utf8Path, mode: &str, module: &str) -> Command {
    let mut command = Command::new("cargo");
    command
        .arg("run")
        .arg("--quiet")
        .arg("--locked")
        .arg("--bin")
        .arg("geam-runner")
        .arg("--")
        .arg(mode)
        .arg(project_root)
        .arg(module)
        .current_dir(project_root)
        .env("CARGO_TARGET_DIR", project_root.join(TARGET_DIRECTORY));
    command
}

fn execution_command(
    project_root: &Utf8Path,
    module: &str,
    configurations: &[(String, Utf8PathBuf)],
) -> Command {
    let mut command = runner_command(project_root, "run", module);
    for (package, path) in configurations {
        command.arg(format!("{package}={path}"));
    }
    command
}

pub(crate) fn reconcile_lock(
    project_root: &Utf8Path,
    manifest_changed: bool,
    cargo: &dyn CargoLock,
) -> Result<(), CliError> {
    let lock = project_root.join("Cargo.lock");
    if manifest_changed {
        remove_stale_lock(&lock)?;
    }
    if manifest_changed || !lock.is_file() {
        cargo.generate_lockfile(project_root)?;
    }
    Ok(())
}

fn remove_stale_lock(path: &Utf8Path) -> Result<(), CliError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(CliError::FileWrite {
            path: path.to_path_buf(),
            error,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CargoLock, RunnerChecker, SystemCargo, execution_command, reconcile_lock, runner_command,
    };
    use crate::error::CliError;
    use camino::{Utf8Path, Utf8PathBuf};
    use std::cell::RefCell;
    use std::fs;
    use tempfile::tempdir;

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

    struct FailingCargo;

    impl CargoLock for FailingCargo {
        fn generate_lockfile(&self, _project_root: &Utf8Path) -> Result<(), CliError> {
            Err(CliError::ProcessFailure {
                command: "cargo generate-lockfile".to_owned(),
                status: Some(1),
                stderr: "fixture stop".to_owned(),
            })
        }
    }

    #[test]
    fn constructs_check_and_run_commands_with_project_owned_targets() {
        let project = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");

        let check = runner_command(&root, "check", "application");
        assert_eq!(
            check
                .get_args()
                .map(|argument| argument.to_string_lossy().into_owned())
                .collect::<Vec<_>>(),
            [
                "run",
                "--quiet",
                "--locked",
                "--bin",
                "geam-runner",
                "--",
                "check",
                root.as_str(),
                "application",
            ],
        );

        let run = execution_command(
            &root,
            "worker",
            &[("images".to_owned(), root.join("config.toml"))],
        );
        assert_eq!(
            run.get_args()
                .map(|argument| argument.to_string_lossy().into_owned())
                .collect::<Vec<_>>(),
            [
                "run".to_owned(),
                "--quiet".to_owned(),
                "--locked".to_owned(),
                "--bin".to_owned(),
                "geam-runner".to_owned(),
                "--".to_owned(),
                "run".to_owned(),
                root.to_string(),
                "worker".to_owned(),
                format!("images={}", root.join("config.toml")),
            ],
        );
        assert!(run.get_envs().any(|(key, value)| {
            key == "CARGO_TARGET_DIR" && value == Some(root.join("build/geam/target").as_os_str())
        }),);
    }

    #[test]
    fn updates_only_changed_or_missing_locks() {
        let project = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let cargo = RecordingCargo::default();

        reconcile_lock(&root, true, &cargo).expect("initial lock should reconcile");
        assert_eq!(cargo.operations.borrow().as_slice(), ["lock"]);

        cargo.operations.borrow_mut().clear();
        reconcile_lock(&root, false, &cargo).expect("unchanged lock should reconcile");
        assert!(cargo.operations.borrow().is_empty());

        fs::remove_file(root.join("Cargo.lock")).expect("fixture lock should be removed");
        reconcile_lock(&root, false, &cargo).expect("missing lock should be regenerated");
        assert_eq!(cargo.operations.borrow().as_slice(), ["lock"]);
    }

    #[test]
    fn preserves_lock_lifecycle_failures() {
        let failure = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(failure.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        fs::write(root.join("Cargo.lock"), "stale lock\n")
            .expect("stale fixture lock should be written");
        let error = reconcile_lock(&root, true, &FailingCargo)
            .expect_err("lock failure should be preserved");
        assert!(matches!(
            error,
            CliError::ProcessFailure { command, status: Some(1), stderr }
                if command == "cargo generate-lockfile" && stderr == "fixture stop"
        ));
        assert!(
            !root.join("Cargo.lock").exists(),
            "failed lock generation must leave the next reconciliation recoverable",
        );

        fs::create_dir(root.join("Cargo.lock")).expect("blocking lock directory should be created");
        let lock = root.join("Cargo.lock");
        let expected_kind = fs::remove_file(&lock)
            .expect_err("directory should reject file removal")
            .kind();
        let error = reconcile_lock(&root, true, &FailingCargo)
            .expect_err("an unremovable stale lock should fail before Cargo");
        assert!(matches!(
            error,
            CliError::FileWrite { path, error }
                if path == lock && error.kind() == expected_kind
        ));
    }

    #[test]
    fn preserves_system_cargo_process_failures() {
        let project = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");

        let generation = SystemCargo
            .generate_lockfile(&root)
            .expect_err("missing manifest should reject lock generation");
        assert!(matches!(
            generation,
            CliError::ProcessFailure { command, status: Some(101), stderr }
                if command
                    == format!(
                        "cargo generate-lockfile --manifest-path {}",
                        root.join("Cargo.toml")
                    )
                    && stderr.contains("manifest path")
        ));

        let check = SystemCargo
            .check(&root, "application")
            .expect_err("missing manifest should reject runner checking");
        assert!(matches!(
            check,
            CliError::ProcessFailure { command, status: Some(101), stderr }
                if command
                    == format!(
                        "cargo run --quiet --locked --bin geam-runner -- check {root} application"
                    )
                    && stderr.contains("could not find `Cargo.toml`")
        ));
    }
}
