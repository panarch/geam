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

#[test]
fn dispatches_prepare_and_run_after_resolving_the_project() {
    let project = gleam_project();

    let prepare = geam(&project, ["prepare"]);
    assert!(!prepare.status.success());
    assert!(String::from_utf8_lossy(&prepare.stderr).contains("runner generation is unavailable"));

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
    assert!(String::from_utf8_lossy(&run.stderr).contains("runner generation is unavailable"));
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
    Command::new(env!("CARGO_BIN_EXE_geam"))
        .args(arguments)
        .current_dir(directory.path())
        .output()
        .expect("Geam CLI should start")
}

fn gleam_project() -> TempDir {
    let project = tempdir().expect("temporary project should be created");
    fs::create_dir(project.path().join("src")).expect("source directory should be created");
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
