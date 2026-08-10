use crate::error::CliError;
use crate::process::run_checked;
use camino::Utf8Path;
use std::fs;
use std::process::Command;

const RUNNER_SOURCE: &str = "build/geam/runner.rs";
const TARGET_DIRECTORY: &str = "build/geam/target";

pub(super) trait CargoLock {
    fn generate_lockfile(&self, project_root: &Utf8Path) -> Result<(), CliError>;
}

pub(super) trait RunnerChecker {
    fn check(&self, project_root: &Utf8Path, module: &str) -> Result<(), CliError>;
}

pub(super) struct SystemCargo;

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
        finish_process(run_checked(
            Command::new("cargo")
                .arg("run")
                .arg("--quiet")
                .arg("--locked")
                .arg("--bin")
                .arg("geam-runner")
                .arg("--")
                .arg("check")
                .arg(project_root)
                .arg(module)
                .current_dir(project_root)
                .env("CARGO_TARGET_DIR", project_root.join(TARGET_DIRECTORY)),
        ))
    }
}

fn finish_process(result: Result<std::process::Output, CliError>) -> Result<(), CliError> {
    result.map(drop)
}

pub(super) fn reconcile_source(
    project_root: &Utf8Path,
    provider_aliases: &[String],
) -> Result<bool, CliError> {
    write_source(project_root, provider_aliases)
}

pub(super) fn reconcile_lock(
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

fn render_source(provider_aliases: &[String]) -> String {
    let mut provider_aliases = provider_aliases.to_vec();
    provider_aliases.sort();
    provider_aliases.dedup();
    let store_fields = provider_aliases
        .iter()
        .map(|alias| {
            format!("    {alias}: <{alias}::Component as geam::HostProviderComponent>::Stores,\n")
        })
        .collect::<String>();
    let state_fields = provider_aliases
        .iter()
        .map(|alias| {
            format!("    {alias}: <{alias}::Component as geam::HostProviderComponent>::RunState,\n")
        })
        .collect::<String>();
    let component_profiles = provider_aliases
        .iter()
        .map(|alias| {
            format!(
                "\nimpl geam::HostComponentProfile<{alias}::Component> for Profile {{\n    fn component_stores(stores: &Self::ExternalStores) -> &<{alias}::Component as geam::HostProviderComponent>::Stores {{\n        &stores.{alias}\n    }}\n\n    fn component_state(state: &mut Self::RunState) -> &mut <{alias}::Component as geam::HostProviderComponent>::RunState {{\n        &mut state.{alias}\n    }}\n}}\n"
            )
        })
        .collect::<String>();
    let component_registrations = provider_aliases
        .iter()
        .map(|alias| {
            format!(
                "    providers.extend(<{alias}::Component as geam::HostProviderComponentRegistration<Profile>>::providers()?);\n"
            )
        })
        .collect::<String>();

    format!(
        "// Generated by Geam. Do not edit.\n\n#[derive(Default)]\nstruct Stores {{\n    stdlib: geam::gleam_stdlib::GleamStdlibStores,\n    json: geam::gleam_json::GleamJsonStores,\n{store_fields}}}\n\nstruct RunState {{\n    stdlib: geam::gleam_stdlib::GleamStdlibRunState,\n    time: geam::gleam_time::SystemTimeSource,\n    io: Vec<geam::gleam_stdlib::IoOutput>,\n{state_fields}}}\n\nstruct Profile;\n\nimpl geam::HostProfile for Profile {{\n    type RunState = RunState;\n    type ExternalStores = Stores;\n}}\n\nimpl geam::gleam_stdlib::GleamStdlibHostProfile for Profile {{\n    type Io = Vec<geam::gleam_stdlib::IoOutput>;\n\n    fn gleam_stdlib_stores(stores: &Self::ExternalStores) -> &geam::gleam_stdlib::GleamStdlibStores {{\n        &stores.stdlib\n    }}\n\n    fn gleam_stdlib_run_state(state: &mut Self::RunState) -> &mut geam::gleam_stdlib::GleamStdlibRunState {{\n        &mut state.stdlib\n    }}\n\n    fn gleam_stdlib_io(state: &mut Self::RunState) -> &mut Self::Io {{\n        &mut state.io\n    }}\n}}\n\nimpl geam::gleam_json::GleamJsonHostProfile for Profile {{\n    fn gleam_json_stores(stores: &Self::ExternalStores) -> &geam::gleam_json::GleamJsonStores {{\n        &stores.json\n    }}\n}}\n\nimpl geam::gleam_time::GleamTimeHostProfile for Profile {{\n    type Source = geam::gleam_time::SystemTimeSource;\n\n    fn gleam_time_source(state: &mut Self::RunState) -> &mut Self::Source {{\n        &mut state.time\n    }}\n}}\n{component_profiles}\nfn host_providers() -> Result<geam::HostProviderSet<Profile>, geam::HostRegistrationError> {{\n    let mut providers = geam::gleam_stdlib::host_providers::<Profile>()?;\n    providers.extend(geam::gleam_json::host_providers::<Profile>()?);\n    providers.extend(geam::gleam_time::host_providers::<Profile>()?);\n{component_registrations}    geam::HostProviderSet::with_providers(Vec::<geam::HostModule<Profile>>::new(), providers)\n}}\n\nfn check(project_root: String, module: String) -> Result<(), Box<dyn std::error::Error>> {{\n    let typed = geam::compile_typed_host_project(project_root, module, host_providers()?)?;\n    let plan = geam::plan_host_program(typed)?;\n    let _execution = geam::HostedExecution::try_from_module_plan(plan)?;\n    Ok(())\n}}\n\nfn main() -> Result<(), Box<dyn std::error::Error>> {{\n    let mut arguments = std::env::args().skip(1);\n    let mode = arguments.next().ok_or_else(invalid_arguments)?;\n    let project_root = arguments.next().ok_or_else(invalid_arguments)?;\n    let module = arguments.next().ok_or_else(invalid_arguments)?;\n    if mode != \"check\" || arguments.next().is_some() {{\n        return Err(invalid_arguments().into());\n    }}\n    check(project_root, module)\n}}\n\nfn invalid_arguments() -> std::io::Error {{\n    std::io::Error::new(\n        std::io::ErrorKind::InvalidInput,\n        \"expected internal runner arguments: check PROJECT_ROOT MODULE\",\n    )\n}}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::{
        CargoLock, RunnerChecker, SystemCargo, reconcile_lock, reconcile_source, render_source,
        write_generated_source,
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
    fn renders_static_profiles_in_sorted_component_order() {
        let source = render_source(&[
            "geam_provider_zeta".to_owned(),
            "geam_provider_alpha".to_owned(),
            "geam_provider_alpha".to_owned(),
        ]);

        assert!(source.starts_with("// Generated by Geam. Do not edit.\n"));
        assert_eq!(source.matches("    geam_provider_alpha: <").count(), 2);
        assert_eq!(source.matches("    geam_provider_zeta: <").count(), 2);
        assert!(
            source
                .find("    geam_provider_alpha: <")
                .expect("alpha should render")
                < source
                    .find("    geam_provider_zeta: <")
                    .expect("zeta should render"),
        );
        assert!(source.contains("impl geam::gleam_stdlib::GleamStdlibHostProfile for Profile"));
        assert!(source.contains("impl geam::gleam_json::GleamJsonHostProfile for Profile"));
        assert!(source.contains("impl geam::gleam_time::GleamTimeHostProfile for Profile"));
        assert!(source.contains(
            "<geam_provider_alpha::Component as geam::HostProviderComponentRegistration<Profile>>::providers()?"
        ));
        assert!(source.contains(
            "<geam_provider_zeta::Component as geam::HostProviderComponentRegistration<Profile>>::providers()?"
        ));
        assert_eq!(
            source,
            render_source(&[
                "geam_provider_alpha".to_owned(),
                "geam_provider_zeta".to_owned(),
            ])
        );
    }

    #[test]
    fn writes_only_changed_sources_and_updates_only_changed_or_missing_locks() {
        let project = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let cargo = RecordingCargo::default();
        let providers = ["geam_provider_images".to_owned()];

        assert!(reconcile_source(&root, &providers).expect("initial source should reconcile"));
        reconcile_lock(&root, true, &cargo).expect("initial lock should reconcile");
        assert_eq!(cargo.operations.borrow().as_slice(), ["lock"]);
        let source = fs::read_to_string(root.join("build/geam/runner.rs"))
            .expect("runner source should be readable");

        cargo.operations.borrow_mut().clear();
        assert!(!reconcile_source(&root, &providers).expect("unchanged source should reconcile"));
        reconcile_lock(&root, false, &cargo).expect("unchanged lock should reconcile");
        assert!(cargo.operations.borrow().is_empty());
        assert_eq!(
            fs::read_to_string(root.join("build/geam/runner.rs"))
                .expect("runner source should remain readable"),
            source,
        );

        fs::remove_file(root.join("Cargo.lock")).expect("fixture lock should be removed");
        reconcile_lock(&root, false, &cargo).expect("missing lock should be regenerated");
        assert_eq!(cargo.operations.borrow().as_slice(), ["lock"]);
    }

    #[test]
    fn preserves_runner_filesystem_and_lock_failures() {
        let blocked = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(blocked.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        fs::write(root.join("build"), "blocked").expect("blocking file should be written");
        let error = reconcile_source(&root, &[]).expect_err("blocked runner directory should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        let unreadable = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(unreadable.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        fs::create_dir_all(root.join("build/geam/runner.rs"))
            .expect("blocking source directory should be created");
        let error = reconcile_source(&root, &[]).expect_err("unreadable runner source should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileRead {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

        let destination = tempdir().expect("temporary destination should be created");
        let path = Utf8PathBuf::from_path_buf(destination.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let error = write_generated_source(&path, "source".to_owned())
            .expect_err("directory destination should fail");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );

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
            assert_eq!(
                std::mem::discriminant(&error),
                std::mem::discriminant(&CliError::FileWrite {
                    path: Utf8PathBuf::new(),
                    error: std::io::Error::other(""),
                }),
            );
        }

        let failure = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(failure.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        fs::write(root.join("Cargo.lock"), "stale lock\n")
            .expect("stale fixture lock should be written");
        reconcile_source(&root, &[]).expect("runner source should be prepared");
        let error = reconcile_lock(&root, true, &FailingCargo)
            .expect_err("lock failure should be preserved");
        assert_eq!(
            error.to_string(),
            "`cargo generate-lockfile` failed with status Some(1): fixture stop"
        );
        assert!(
            !root.join("Cargo.lock").exists(),
            "failed lock generation must leave the next reconciliation recoverable",
        );

        fs::create_dir(root.join("Cargo.lock")).expect("blocking lock directory should be created");
        let error = reconcile_lock(&root, true, &FailingCargo)
            .expect_err("an unremovable stale lock should fail before Cargo");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&CliError::FileWrite {
                path: Utf8PathBuf::new(),
                error: std::io::Error::other(""),
            }),
        );
    }

    #[test]
    fn preserves_system_cargo_process_failures() {
        let project = tempdir().expect("temporary project should be created");
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");

        let errors = [
            SystemCargo
                .generate_lockfile(&root)
                .expect_err("missing manifest should reject lock generation"),
            SystemCargo
                .check(&root, "application")
                .expect_err("missing manifest should reject runner checking"),
        ];
        for error in errors {
            assert_eq!(
                std::mem::discriminant(&error),
                std::mem::discriminant(&CliError::ProcessFailure {
                    command: String::new(),
                    status: None,
                    stderr: String::new(),
                }),
            );
            assert!(error.to_string().contains("failed with status Some("));
        }
    }
}
