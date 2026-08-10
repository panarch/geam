use std::fs;
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
fn reports_entry_resolution_failures_through_the_binary_boundary() {
    let project = tempdir().expect("temporary directory should be created");
    fs::write(project.path().join("gleam.toml"), "invalid")
        .expect("invalid Gleam config should be written");

    for arguments in [["prepare"], ["run"]] {
        let output = geam(&project, arguments);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("invalid Gleam package config"),);
    }
}

#[test]
fn reports_project_compilation_failures_through_the_binary_boundary() {
    let project = gleam_project();

    for command in ["prepare", "run"] {
        let output = geam(&project, [command, "--module", "missing"]);
        assert!(!output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "geam: root module missing was not supplied by package application\n",
        );
    }
}

#[test]
fn prepares_pure_projects_and_keeps_run_pending() {
    let project = gleam_project();

    let prepare = geam(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );
    assert!(project.path().join("Cargo.lock").is_file());
    assert!(project.path().join("build/geam/runner.rs").is_file());

    let run = geam(
        &project,
        [
            "run",
            "--module",
            "application",
            "--provider-config",
            "images=config.toml",
        ],
    );
    assert!(!run.status.success());
    assert!(String::from_utf8_lossy(&run.stderr).contains("standalone execution is unavailable"));
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

#[test]
fn prepares_an_independent_path_provider_through_the_generated_runner() {
    let project = provider_backed_project();
    let provider = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/provider_sdk/provider");

    let add = geam(
        &project,
        [
            "provider",
            "add",
            "--path",
            provider
                .to_str()
                .expect("provider path should be valid UTF-8"),
        ],
    );
    assert!(
        add.status.success(),
        "provider add failed: {}",
        String::from_utf8_lossy(&add.stderr),
    );

    let prepare = geam(&project, ["prepare"]);
    assert!(
        prepare.status.success(),
        "provider-backed prepare failed: {}",
        String::from_utf8_lossy(&prepare.stderr),
    );
    let manifest = fs::read_to_string(project.path().join("Cargo.toml"))
        .expect("managed manifest should be readable");
    let runner = fs::read_to_string(project.path().join("build/geam/runner.rs"))
        .expect("runner source should be readable");
    assert!(manifest.contains("geam_provider_provider_sdk_example"));
    assert!(runner.contains("geam_provider_provider_sdk_example::Component"));

    let repeated = geam(&project, ["prepare", "--module", "application"]);
    assert!(
        repeated.status.success(),
        "repeated prepare failed: {}",
        String::from_utf8_lossy(&repeated.stderr),
    );
    assert_eq!(
        fs::read_to_string(project.path().join("Cargo.toml"))
            .expect("managed manifest should remain readable"),
        manifest,
    );
    assert_eq!(
        fs::read_to_string(project.path().join("build/geam/runner.rs"))
            .expect("runner source should remain readable"),
        runner,
    );
}

fn geam<const N: usize>(directory: &TempDir, arguments: [&str; N]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_geam"));
    command.args(arguments).current_dir(directory.path());
    remove_nested_cargo_instrumentation(&mut command);
    command.output().expect("Geam CLI should start")
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

fn provider_backed_project() -> TempDir {
    let project = gleam_project();
    fs::write(
        project.path().join("gleam.toml"),
        r#"name = "application"
version = "1.0.0"

[dependencies]
provider_sdk_example = { path = "packages/provider_sdk_example" }
"#,
    )
    .expect("Gleam config should be written");
    fs::write(
        project.path().join("manifest.toml"),
        r#"packages = [
  { name = "provider_sdk_example", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/provider_sdk_example" },
]

[requirements]
provider_sdk_example = { path = "packages/provider_sdk_example" }
"#,
    )
    .expect("Gleam manifest should be written");
    fs::write(
        project.path().join("src/application.gleam"),
        r#"import provider/sdk

pub fn main() {
  sdk.decorate("item", fn(value) { value <> "!" })
}
"#,
    )
    .expect("application source should be written");
    let package = project.path().join("packages/provider_sdk_example");
    fs::create_dir_all(package.join("src/provider"))
        .expect("provider package source directory should be created");
    fs::write(
        package.join("gleam.toml"),
        "name = \"provider_sdk_example\"\nversion = \"1.0.0\"\n",
    )
    .expect("provider package config should be written");
    fs::write(
        package.join("src/provider/sdk.gleam"),
        r#"@external(erlang, "provider_sdk", "Catalog")
pub type Catalog

@external(erlang, "provider_sdk", "decorate")
pub fn decorate(value: String, transform: fn(String) -> String) -> String

@external(erlang, "provider_sdk", "catalog_new")
pub fn catalog_new() -> Catalog

@external(erlang, "provider_sdk", "catalog_insert")
pub fn catalog_insert(catalog: Catalog, key: String, value: String) -> Catalog

@external(erlang, "provider_sdk", "catalog_hash")
pub fn catalog_hash(catalog: Catalog) -> Int

pub type Summary {
  Summary(count: Int, items: List(String))
}

@external(erlang, "provider_sdk", "summarize")
pub fn summarize(value: String, transform: fn(String) -> String) -> Summary
"#,
    )
    .expect("provider package source should be written");
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
