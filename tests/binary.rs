use std::fs;
use std::path::Path;
#[cfg(unix)]
use std::process::Stdio;
use std::process::{Command, Output};
use tempfile::{TempDir, tempdir};

#[test]
fn reports_missing_projects_through_the_binary_boundary() {
    let directory = tempdir().expect("temporary directory should be created");

    let output = geam(&directory, ["prepare"]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("no gleam.toml was found"));
}

#[cfg(unix)]
#[test]
fn reports_current_directory_failures_through_the_binary_boundary() {
    let directory = tempdir().expect("temporary directory should be created");
    let deleted = directory.path().join("deleted");
    fs::create_dir(&deleted).expect("working directory should be created");

    let mut command = Command::new("/bin/sh");
    command
        .args([
            "-c",
            "cd \"$1\" && rmdir \"$1\" && exec \"$2\" prepare",
            "sh",
        ])
        .arg(&deleted)
        .arg(env!("CARGO_BIN_EXE_geam"));
    remove_nested_cargo_instrumentation(&mut command);
    let output = command.output().expect("Geam CLI should start");

    assert!(!output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "geam: failed to determine the current directory\n",
    );
}

#[test]
fn prepares_and_runs_pure_projects_without_printing_main_results() {
    let project = gleam_project();

    let prepare = geam(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );
    assert!(project.path().join("Cargo.lock").is_file());
    assert!(project.path().join("build/geam/runner.rs").is_file());

    let run = geam(&project, ["run", "--module", "application"]);
    assert!(
        run.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&run.stderr),
    );
    assert!(run.stdout.is_empty());
    assert!(
        run.stderr.is_empty(),
        "run wrote unexpected stderr: {}",
        String::from_utf8_lossy(&run.stderr),
    );
}

#[test]
fn streams_gleam_io_and_echo_in_source_order_before_runtime_failure() {
    let project = io_project();

    let run = geam(&project, ["run"]);
    assert!(
        run.status.success(),
        "IO run failed: {}",
        String::from_utf8_lossy(&run.stderr),
    );
    assert_eq!(String::from_utf8_lossy(&run.stdout), "stdout");
    let stderr = String::from_utf8_lossy(&run.stderr);
    let first = stderr
        .find("stderr-one\n")
        .expect("first stderr IO should be present");
    let echo = stderr
        .find(" middle\nNil\n")
        .expect("echo output should be present");
    let last = stderr
        .find("stderr-two\n")
        .expect("last stderr IO should be present");
    assert!(first < echo && echo < last);

    #[cfg(unix)]
    {
        let mut child = geam_command(&project, ["run"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Geam CLI should start with piped output");
        drop(child.stdout.take());
        let failed_output = child
            .wait_with_output()
            .expect("Geam CLI output failure should complete");
        assert!(!failed_output.status.success());
        let stderr = String::from_utf8_lossy(&failed_output.stderr);
        assert!(stderr.contains("geam runner:"));
        assert!(stderr.contains("after writing its output directly"));
    }

    fs::write(
        project.path().join("src/application.gleam"),
        r#"import gleam/io

pub fn main() {
  io.print_error("before panic\n")
  panic as "stop"
}
"#,
    )
    .expect("failing source should be written");
    let failed = geam(&project, ["run"]);
    assert!(!failed.status.success());
    assert!(failed.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&failed.stderr);
    assert!(stderr.starts_with("before panic\n"));
    assert!(stderr.contains("stop"));
}

#[test]
fn dispatches_provider_add_and_remove_through_the_binary_boundary() {
    let project = gleam_project_with_dependency("images", "1.2.3");
    let provider = provider_package();

    let add = geam(
        &project,
        [
            "provider",
            "add",
            "--path",
            provider
                .path()
                .to_str()
                .expect("path should be valid UTF-8"),
        ],
    );
    assert!(
        add.status.success(),
        "provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let remove = geam(&project, ["provider", "remove", "images"]);
    assert!(
        remove.status.success(),
        "provider remove failed: {}",
        String::from_utf8_lossy(&remove.stderr),
    );
}

fn geam<const N: usize>(directory: &TempDir, arguments: [&str; N]) -> Output {
    geam_at(directory.path(), arguments)
}

fn geam_at<const N: usize>(directory: &Path, arguments: [&str; N]) -> Output {
    geam_command_at(directory, arguments)
        .output()
        .expect("Geam CLI should start")
}

fn geam_command<const N: usize>(directory: &TempDir, arguments: [&str; N]) -> Command {
    geam_command_at(directory.path(), arguments)
}

fn geam_command_at<const N: usize>(directory: &Path, arguments: [&str; N]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_geam"));
    command.args(arguments).current_dir(directory);
    remove_nested_cargo_instrumentation(&mut command);
    command
}

fn remove_nested_cargo_instrumentation(command: &mut Command) {
    // Keep the CLI process profiled while preventing its nested Cargo build
    // from compiling a second instrumented copy of Geam into the report.
    command
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_LLVM_COV")
        .env_remove("CARGO_LLVM_COV_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .env_remove("RUSTDOCFLAGS");
}

fn gleam_project() -> TempDir {
    let project = tempdir().expect("temporary project should be created");
    fs::create_dir(project.path().join("src")).expect("source directory should be created");
    write_cargo_config(&project);
    fs::write(
        project.path().join("gleam.toml"),
        "name = \"application\"\nversion = \"1.0.0\"\n",
    )
    .expect("Gleam config should be written");
    fs::write(
        project.path().join("manifest.toml"),
        "packages = []\n[requirements]\n",
    )
    .expect("Gleam manifest should be written");
    fs::write(
        project.path().join("src/application.gleam"),
        "pub fn main() { 1 }\n",
    )
    .expect("Gleam source should be written");
    project
}

fn io_project() -> TempDir {
    let project = gleam_project();
    fs::write(
        project.path().join("gleam.toml"),
        r#"name = "application"
version = "1.0.0"

[dependencies]
gleam_stdlib = { path = "packages/gleam_stdlib" }
"#,
    )
    .expect("Gleam config should be written");
    fs::write(
        project.path().join("manifest.toml"),
        r#"packages = [
  { name = "gleam_stdlib", version = "1.0.3", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/gleam_stdlib" },
]

[requirements]
gleam_stdlib = { path = "packages/gleam_stdlib" }
"#,
    )
    .expect("Gleam manifest should be written");
    fs::write(
        project.path().join("src/application.gleam"),
        r#"import gleam/io

pub fn main() {
  io.print("stdout")
  io.print_error("stderr-one\n")
  echo Nil as "middle"
  io.print_error("stderr-two\n")
  Nil
}
"#,
    )
    .expect("application source should be written");
    let package = project.path().join("packages/gleam_stdlib");
    fs::create_dir_all(package.join("src/gleam"))
        .expect("stdlib package source directory should be created");
    fs::write(
        package.join("gleam.toml"),
        "name = \"gleam_stdlib\"\nversion = \"1.0.3\"\n",
    )
    .expect("stdlib package config should be written");
    fs::write(
        package.join("src/gleam/io.gleam"),
        r#"@external(erlang, "gleam_stdlib", "print")
pub fn print(output: String) -> Nil

@external(erlang, "gleam_stdlib", "print_error")
pub fn print_error(output: String) -> Nil

@external(erlang, "gleam_stdlib", "println")
pub fn println(output: String) -> Nil

@external(erlang, "gleam_stdlib", "println_error")
pub fn println_error(output: String) -> Nil
"#,
    )
    .expect("stdlib IO source should be written");
    project
}

fn write_cargo_config(project: &TempDir) {
    fs::create_dir(project.path().join(".cargo"))
        .expect("Cargo config directory should be created");
    let geam_path = toml::Value::String(env!("CARGO_MANIFEST_DIR").to_owned()).to_string();
    fs::write(
        project.path().join(".cargo/config.toml"),
        format!("[patch.crates-io]\ngeam = {{ path = {geam_path} }}\n\n[net]\noffline = true\n"),
    )
    .expect("Cargo config should be written");
}

fn gleam_project_with_dependency(package: &str, version: &str) -> TempDir {
    let project = gleam_project();
    fs::write(
        project.path().join("gleam.toml"),
        format!(
            "name = \"application\"\nversion = \"1.0.0\"\n\n[dependencies]\n{package} = \"{version}\"\n",
        ),
    )
    .expect("Gleam config should be written");
    fs::write(
        project.path().join("manifest.toml"),
        format!(
            "packages = [\n  {{ name = \"{package}\", version = \"{version}\", build_tools = [\"gleam\"], requirements = [], source = \"hex\", outer_checksum = \"00\" }},\n]\n\n[requirements]\n",
        ),
    )
    .expect("Gleam manifest should be written");
    project
}

fn provider_package() -> TempDir {
    let provider = tempdir().expect("provider package should be created");
    fs::create_dir(provider.path().join("src")).expect("source directory should be created");
    fs::write(
        provider.path().join("Cargo.toml"),
        r#"[package]
name = "geam-images"
version = "1.0.0"
edition = "2024"

[package.metadata.geam.provider]
schema = 1
gleam-package = "images"
gleam-version = ">= 1.0.0 and < 2.0.0"
"#,
    )
    .expect("provider manifest should be written");
    fs::write(
        provider.path().join("src/lib.rs"),
        "pub struct Component;\n",
    )
    .expect("provider source should be written");
    provider
}
