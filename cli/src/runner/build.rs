use super::SystemCargo;
use super::source::{APPLICATION_SOURCE, GENERATED_HEADER, PROGRAM_SOURCE, read_generated_source};
use crate::cargo::{CargoMetadataLoader, CargoMetadataMode, SystemCargoMetadata};
use crate::error::CliError;
use crate::process::run_checked_with_progress;
use crate::progress::Progress;
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::{Message, PackageId};
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};

pub(crate) trait ExecutableBuilder {
    fn build(
        &self,
        project_root: &Utf8Path,
        module: &str,
        package: &str,
        profile: BuildProfile,
        progress: &mut Progress<'_>,
    ) -> Result<Utf8PathBuf, CliError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BuildProfile {
    Debug,
    Release,
}

pub(crate) struct BuildSession {
    lock: File,
}

impl BuildSession {
    pub(crate) fn acquire(project_root: &Utf8Path) -> Result<Self, CliError> {
        let directory = project_root.join("build/geam");
        std::fs::create_dir_all(&directory).map_err(|error| CliError::FileWrite {
            path: directory.clone(),
            error,
        })?;
        let path = directory.join("build.lock");
        let lock = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .map_err(|error| CliError::FileWrite {
                path: path.clone(),
                error,
            })?;
        lock.try_lock()
            .map_err(|error| CliError::StandaloneBuildLock { path, error })?;
        Ok(Self { lock })
    }
}

impl Drop for BuildSession {
    fn drop(&mut self) {
        // A concurrently forked child may briefly hold a duplicate of this file.
        let _ = self.lock.unlock();
    }
}

impl ExecutableBuilder for SystemCargo {
    fn build(
        &self,
        project_root: &Utf8Path,
        module: &str,
        package: &str,
        profile: BuildProfile,
        progress: &mut Progress<'_>,
    ) -> Result<Utf8PathBuf, CliError> {
        let program = project_root.join(PROGRAM_SOURCE);
        read_generated_source(&program)?;
        let temporary = temporary_program(&project_root.join("build/geam"))?;
        progress.report(format_args!("Preparing executable data for {module}"))?;
        run_checked_with_progress(
            &mut preparation_command(project_root, module, &temporary, profile),
            progress,
            Stdio::inherit(),
        )?;
        // The helper writes a complete source file; publish it only after success.
        let prepared =
            std::fs::read_to_string(&temporary).map_err(|error| CliError::PreparedProgramRead {
                path: temporary.to_path_buf(),
                error,
            })?;
        if !prepared.starts_with(GENERATED_HEADER) {
            return Err(invalid_output(
                package,
                "preparer did not write generated program data",
            ));
        }
        temporary
            .persist(&program)
            .map_err(|error| CliError::FileWrite {
                path: program,
                error: error.error,
            })?;
        let metadata = SystemCargoMetadata.load(
            project_root,
            &project_root.join("Cargo.toml"),
            CargoMetadataMode::Locked,
            progress,
        )?;
        let root = metadata
            .root_package()
            .ok_or_else(|| invalid_output(package, "managed package is absent"))?;
        progress.report(format_args!("Compiling executable {package}"))?;
        let output = run_checked_with_progress(
            &mut build_command(project_root, package, profile),
            progress,
            Stdio::piped(),
        )?;
        executable(
            &output.stdout,
            &root.id,
            package,
            &project_root.join(APPLICATION_SOURCE),
        )
    }
}

fn temporary_program(directory: &Utf8Path) -> Result<tempfile::TempPath, CliError> {
    tempfile::Builder::new()
        .prefix("program-")
        .suffix(".rs")
        .tempfile_in(directory)
        .map(tempfile::NamedTempFile::into_temp_path)
        .map_err(|error| CliError::FileWrite {
            path: directory.to_owned(),
            error,
        })
}

fn preparation_command(
    root: &Utf8Path,
    module: &str,
    destination: &Path,
    profile: BuildProfile,
) -> Command {
    let mut command = cargo_command(root, "run", "geam-runner", profile);
    command
        .arg("--")
        .arg("prepare")
        .arg(root)
        .arg(module)
        .arg(destination);
    command
}

fn build_command(root: &Utf8Path, package: &str, profile: BuildProfile) -> Command {
    let mut command = cargo_command(root, "build", package, profile);
    command.arg("--message-format=json-render-diagnostics");
    command
}

fn cargo_command(root: &Utf8Path, action: &str, binary: &str, profile: BuildProfile) -> Command {
    let mut command = Command::new("cargo");
    command.arg(action).arg("--locked").arg("--bin").arg(binary);
    if profile == BuildProfile::Release {
        command.arg("--release");
    }
    command.current_dir(root).env(
        "CARGO_TARGET_DIR",
        root.join(super::cargo::TARGET_DIRECTORY),
    );
    command
}

fn executable(
    bytes: &[u8],
    package_id: &PackageId,
    package: &str,
    source: &Utf8Path,
) -> Result<Utf8PathBuf, CliError> {
    let mut executable = None;
    let mut finished = false;
    for message in Message::parse_stream(bytes) {
        match message.map_err(|error| invalid_output(package, error.to_string()))? {
            Message::CompilerArtifact(artifact)
                if artifact.package_id == *package_id
                    && artifact.target.name == package
                    && artifact.target.is_bin()
                    && artifact.target.src_path == source =>
            {
                let path = artifact.executable.ok_or_else(|| {
                    invalid_output(package, "selected artifact has no executable")
                })?;
                if executable.replace(path).is_some() {
                    return Err(invalid_output(
                        package,
                        "selected executable was reported more than once",
                    ));
                }
            }
            Message::BuildFinished(result) => {
                if !result.success {
                    return Err(invalid_output(
                        package,
                        "Cargo reported an unsuccessful build",
                    ));
                }
                finished = true;
            }
            Message::TextLine(_) => {
                return Err(invalid_output(
                    package,
                    "expected structured Cargo messages",
                ));
            }
            _ => {}
        }
    }
    if !finished {
        return Err(invalid_output(
            package,
            "Cargo did not report build completion",
        ));
    }
    executable
        .ok_or_else(|| invalid_output(package, "Cargo did not report the selected executable"))
}

fn invalid_output(package: &str, reason: impl Into<String>) -> CliError {
    CliError::InvalidBuildOutput {
        package: package.to_owned(),
        reason: reason.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BuildProfile, BuildSession, ExecutableBuilder, SystemCargo, build_command, executable,
        preparation_command,
    };
    use crate::error::CliError;
    use crate::progress::Progress;
    use camino::{Utf8Path, Utf8PathBuf};
    use cargo_metadata::PackageId;
    use serde_json::json;
    use std::fs;
    use std::io::{self, Write};
    use std::process::Command;

    #[test]
    fn helper_and_application_share_package_lock_target_and_profile() {
        let root = Utf8Path::new("project with spaces");
        for (profile, flag) in [
            (BuildProfile::Debug, None),
            (BuildProfile::Release, Some("--release")),
        ] {
            let prepare = preparation_command(
                root,
                "tools/report",
                std::path::Path::new("output.rs"),
                profile,
            );
            let build = build_command(root, "my_app", profile);
            let mut expected = vec!["run", "--locked", "--bin", "geam-runner"];
            expected.extend(flag);
            expected.extend([
                "--",
                "prepare",
                "project with spaces",
                "tools/report",
                "output.rs",
            ]);
            assert_eq!(prepare.get_args().collect::<Vec<_>>(), expected);
            let mut expected = vec!["build", "--locked", "--bin", "my_app"];
            expected.extend(flag);
            expected.push("--message-format=json-render-diagnostics");
            assert_eq!(build.get_args().collect::<Vec<_>>(), expected);
            for command in [prepare, build] {
                assert_eq!(command.get_program(), "cargo");
                assert_eq!(command.get_current_dir(), Some(root.as_std_path()));
                assert_eq!(
                    command.get_envs().collect::<Vec<_>>(),
                    [(
                        std::ffi::OsStr::new("CARGO_TARGET_DIR"),
                        Some(root.join("build/geam/target").as_os_str())
                    )]
                );
            }
        }
    }

    #[test]
    fn admits_only_the_selected_successful_cargo_executable() {
        let id = PackageId {
            repr: "path+file:///project#app@0.0.0".into(),
        };
        let source = Utf8Path::new("/project/build/geam/application.rs");
        let artifact = json!({
            "reason": "compiler-artifact", "package_id": id.repr, "manifest_path": "/project/Cargo.toml",
            "target": { "kind": ["bin"], "crate_types": ["bin"], "name": "app", "src_path": source, "edition": "2024", "doc": true, "doctest": false, "test": true },
            "profile": { "opt_level": "0", "debuginfo": 2, "debug_assertions": true, "overflow_checks": true, "test": false },
            "features": [], "filenames": ["/project/build/geam/target/debug/app"],
            "executable": "/project/build/geam/target/debug/app", "fresh": true
        });
        let finished = json!({"reason": "build-finished", "success": true});
        let messages = |values: &[serde_json::Value]| {
            values
                .iter()
                .map(|value| format!("{value}\n"))
                .collect::<String>()
        };
        assert_eq!(
            executable(
                messages(&[artifact.clone(), finished.clone()]).as_bytes(),
                &id,
                "app",
                source
            )
            .unwrap(),
            "/project/build/geam/target/debug/app"
        );
        for (field, value) in [
            ("package_id", json!("other")),
            ("target", {
                let mut target = artifact["target"].clone();
                target["name"] = json!("other");
                target
            }),
            ("target", {
                let mut target = artifact["target"].clone();
                target["kind"] = json!(["lib"]);
                target
            }),
            ("target", {
                let mut target = artifact["target"].clone();
                target["src_path"] = json!("/other.rs");
                target
            }),
        ] {
            let mut other = artifact.clone();
            other[field] = value;
            assert_eq!(
                executable(
                    messages(&[other.clone(), finished.clone()]).as_bytes(),
                    &id,
                    "app",
                    source
                )
                .unwrap_err()
                .to_string(),
                "invalid Cargo build output for app: Cargo did not report the selected executable"
            );
            assert_eq!(
                executable(
                    messages(&[other, artifact.clone(), finished.clone()]).as_bytes(),
                    &id,
                    "app",
                    source
                )
                .unwrap(),
                "/project/build/geam/target/debug/app"
            );
        }
        let mut absent = artifact.clone();
        absent["executable"] = serde_json::Value::Null;
        for (input, expected) in [
            (
                messages(&[absent, finished.clone()]),
                "selected artifact has no executable",
            ),
            (
                messages(&[artifact.clone(), artifact.clone(), finished.clone()]),
                "selected executable was reported more than once",
            ),
            (
                messages(&[json!({"reason":"build-finished", "success":false})]),
                "Cargo reported an unsuccessful build",
            ),
            ("not JSON\n".into(), "expected structured Cargo messages"),
            (
                messages(&[artifact]),
                "Cargo did not report build completion",
            ),
            (
                messages(&[finished]),
                "Cargo did not report the selected executable",
            ),
        ] {
            assert_eq!(
                executable(input.as_bytes(), &id, "app", source)
                    .unwrap_err()
                    .to_string(),
                format!("invalid Cargo build output for app: {expected}")
            );
        }
        let error = executable(&[0xff], &id, "app", source).unwrap_err();
        assert!(
            matches!(error, CliError::InvalidBuildOutput { package, reason } if package == "app" && reason.contains("valid UTF-8"))
        );
    }

    #[test]
    fn build_lock_is_exclusive_reusable_and_preserves_unrelated_files() {
        let directory = tempfile::tempdir().unwrap();
        let root = Utf8Path::from_path(directory.path()).unwrap();
        let first = BuildSession::acquire(root).unwrap();
        fs::write(root.join("build/geam/notes"), "keep").unwrap();
        let error = BuildSession::acquire(root).err().unwrap();
        assert!(
            matches!(error, CliError::StandaloneBuildLock { path, error: std::fs::TryLockError::WouldBlock } if path == root.join("build/geam/build.lock"))
        );
        drop(first);
        let second = BuildSession::acquire(root).unwrap();
        assert_eq!(fs::read(root.join("build/geam/notes")).unwrap(), b"keep");
        drop(second);
        fs::remove_file(root.join("build/geam/build.lock")).unwrap();
        fs::create_dir(root.join("build/geam/build.lock")).unwrap();
        assert!(
            matches!(BuildSession::acquire(root).err().unwrap(), CliError::FileWrite { path, .. } if path == root.join("build/geam/build.lock"))
        );
        fs::remove_dir_all(root.join("build")).unwrap();
        fs::write(root.join("build"), "blocked").unwrap();
        assert!(
            matches!(BuildSession::acquire(root).err().unwrap(), CliError::FileWrite { path, .. } if path == root.join("build/geam"))
        );
        assert!(
            matches!(SystemCargo.build(root, "root", "app", BuildProfile::Debug, &mut Progress::Hidden).unwrap_err(), CliError::FileRead { path, .. } if path == root.join("build/geam/program.rs"))
        );
        fs::remove_file(root.join("build")).unwrap();
        assert!(
            matches!(SystemCargo.build(root, "root", "app", BuildProfile::Debug, &mut Progress::Hidden).unwrap_err(), CliError::FileWrite { path, .. } if path == root.join("build/geam"))
        );
        fs::create_dir_all(root.join("build/geam")).unwrap();
        fs::write(root.join("build/geam/program.rs"), "user data").unwrap();
        assert!(
            matches!(SystemCargo.build(root, "root", "app", BuildProfile::Debug, &mut Progress::Hidden).unwrap_err(), CliError::StandaloneFileConflict { path } if path == root.join("build/geam/program.rs"))
        );
    }

    #[test]
    fn builds_real_cargo_targets_and_does_not_report_stale_artifacts_on_failure() {
        let directory = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(directory.path().canonicalize().unwrap()).unwrap();
        fs::create_dir_all(root.join("build/geam")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            r#"
[package]
name = "build_fixture"
version = "0.0.0"
edition = "2024"
[[bin]]
name = "geam-runner"
path = "build/geam/runner.rs"
[[bin]]
name = "my_app"
path = "build/geam/application.rs"
[workspace]
"#
            .trim_start(),
        )
        .unwrap();
        fs::write(root.join("build/geam/runner.rs"), r#"
fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    assert_eq!(args[1], "prepare");
    assert_eq!(std::env::current_dir().unwrap(), std::path::Path::new(&args[2]));
    let module = args[3].to_str().unwrap();
    let destination = std::path::Path::new(&args[4]);
    match module {
        "fail" => std::process::exit(7),
        "missing" => { std::fs::remove_file(destination).unwrap(); return; },
        "empty" => return,
        "persist" => { std::fs::create_dir("build/geam/program.rs").unwrap(); },
        "broken_metadata" => { std::fs::write("Cargo.toml", "invalid TOML").unwrap(); },
        "metadata" => {
            std::fs::create_dir("member").unwrap();
            std::fs::write("member/Cargo.toml", "[package]\nname = 'build_fixture'\nversion = '0.0.0'\nedition = '2024'\n[lib]\npath = 'lib.rs'\n").unwrap();
            std::fs::write("member/lib.rs", "").unwrap();
            std::fs::write("Cargo.toml", "[workspace]\nmembers = ['member']\nresolver = '3'\n").unwrap();
        },
        _ => {}
    }
    std::fs::write(destination, format!("// Generated by Geam. Do not edit.\npub const VALUE: &str = {:?};\n", format!("{module}:{}", cfg!(debug_assertions)))).unwrap();
}
"#.trim_start()).unwrap();
        fs::write(
            root.join("build/geam/application.rs"),
            "mod program;\nfn main() { println!(\"{}\", program::VALUE); }\n",
        )
        .unwrap();
        let lock = Command::new("cargo")
            .args(["generate-lockfile", "--offline"])
            .current_dir(&root)
            .output()
            .unwrap();
        let diagnostic = String::from_utf8_lossy(&lock.stderr);
        assert!(lock.status.success(), "{diagnostic}");
        let original_lock = fs::read(root.join("Cargo.lock")).unwrap();
        let original_manifest = fs::read(root.join("Cargo.toml")).unwrap();
        for phase in ["Preparing executable data", "Compiling executable"] {
            let mut output = FailedPhase {
                phase,
                bytes: Vec::new(),
            };
            let error = SystemCargo
                .build(
                    &root,
                    "root",
                    "my_app",
                    BuildProfile::Debug,
                    &mut Progress::Visible(&mut output),
                )
                .unwrap_err();
            assert!(
                matches!(error, CliError::PreparationProgressIo(error) if error.kind() == io::ErrorKind::BrokenPipe)
            );
        }
        for (profile, name, text) in [
            (BuildProfile::Debug, "debug", "root:true\n"),
            (BuildProfile::Debug, "debug", "tools/report:true\n"),
            (BuildProfile::Release, "release", "root:false\n"),
        ] {
            let module = if text.starts_with("tools") {
                "tools/report"
            } else {
                "root"
            };
            let path = SystemCargo
                .build(&root, module, "my_app", profile, &mut Progress::Hidden)
                .unwrap();
            assert_eq!(
                path,
                root.join(format!(
                    "build/geam/target/{name}/my_app{}",
                    std::env::consts::EXE_SUFFIX
                ))
            );
            let run = Command::new(&path).output().unwrap();
            assert!(run.status.success());
            assert_eq!(run.stdout, text.as_bytes());
            assert!(run.stderr.is_empty());
            assert_eq!(fs::read(root.join("Cargo.lock")).unwrap(), original_lock);
        }
        let program = root.join("build/geam/program.rs");
        let previous = fs::read(&program).unwrap();
        for (module, expected) in [
            ("fail", "process"),
            ("empty", "invalid"),
            ("missing", "read"),
        ] {
            let error = SystemCargo
                .build(
                    &root,
                    module,
                    "my_app",
                    BuildProfile::Debug,
                    &mut Progress::Hidden,
                )
                .unwrap_err();
            if expected == "process" {
                assert!(
                    matches!(error, CliError::ProcessFailure { status, .. } if status == Some(7))
                );
            } else if expected == "invalid" {
                assert_eq!(
                    error.to_string(),
                    "invalid Cargo build output for my_app: preparer did not write generated program data"
                );
            } else {
                assert!(
                    matches!(error, CliError::PreparedProgramRead { path, error } if path.parent() == Some(root.join("build/geam").as_std_path()) && error.kind() == io::ErrorKind::NotFound)
                );
            }
            assert_eq!(fs::read(&program).unwrap(), previous);
        }
        fs::write(root.join("build/geam/application.rs"), "invalid Rust").unwrap();
        assert!(matches!(
            SystemCargo
                .build(
                    &root,
                    "root",
                    "my_app",
                    BuildProfile::Debug,
                    &mut Progress::Hidden
                )
                .unwrap_err(),
            CliError::ProcessFailure {
                status,
                stderr,
                ..
            } if status == Some(101) && stderr.contains("invalid") && stderr.replace('\\', "/").contains("build/geam/application.rs")
        ));
        assert!(
            matches!(SystemCargo.build(&root, "broken_metadata", "my_app", BuildProfile::Debug, &mut Progress::Hidden).unwrap_err(), CliError::ProcessFailure { status, stderr, .. } if status == Some(101) && stderr.contains("Cargo.toml"))
        );
        fs::write(root.join("Cargo.toml"), original_manifest).unwrap();
        fs::remove_file(&program).unwrap();
        assert!(
            matches!(SystemCargo.build(&root, "persist", "my_app", BuildProfile::Debug, &mut Progress::Hidden).unwrap_err(), CliError::FileWrite { path, .. } if path == program)
        );
        fs::remove_dir(&program).unwrap();
        assert_eq!(
            SystemCargo
                .build(
                    &root,
                    "metadata",
                    "my_app",
                    BuildProfile::Debug,
                    &mut Progress::Hidden
                )
                .unwrap_err()
                .to_string(),
            "invalid Cargo build output for my_app: managed package is absent"
        );
        assert_eq!(
            fs::read_dir(root.join("build/geam"))
                .unwrap()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_name().to_string_lossy().starts_with("program-"))
                .count(),
            0
        );
    }

    struct FailedPhase {
        phase: &'static str,
        bytes: Vec<u8>,
    }

    impl Write for FailedPhase {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            if String::from_utf8_lossy(&self.bytes).contains(self.phase) {
                Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "closed phase output",
                ))
            } else {
                Ok(())
            }
        }
    }
}
