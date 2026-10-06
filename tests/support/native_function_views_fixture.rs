use super::workspace_dependencies;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub fn copy(root: &Path) -> PathBuf {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture = repository.join("tests/fixtures/native_function_views");
    static DEPENDENCIES: OnceLock<Result<(), String>> = OnceLock::new();
    workspace_dependencies::prepare(
        &DEPENDENCIES,
        &fixture.join("project"),
        "gleam",
        &["deps", "download"],
        "locked native function view source dependencies",
    );
    for part in ["project", "provider"] {
        copy_source(&fixture.join(part), &root.join(part));
    }
    copy_source(
        &fixture.join("project/build/packages"),
        &root.join("project/build/packages"),
    );
    let cargo = root.join("provider/Cargo.toml");
    let mut manifest: toml::Value = toml::from_str(&fs::read_to_string(&cargo).unwrap()).unwrap();
    manifest["patch"]["crates-io"]["geam"]["path"] = repository.to_str().unwrap().into();
    fs::write(cargo, toml::to_string(&manifest).unwrap()).unwrap();
    let repository_path = repository.to_str().unwrap();
    let project = root.join("project");
    fs::create_dir_all(project.join(".cargo")).unwrap();
    fs::write(
        project.join(".cargo/config.toml"),
        toml::to_string(&toml::toml! {
            [patch.crates-io.geam]
            path = repository_path
            [net]
            offline = true
        })
        .unwrap(),
    )
    .unwrap();
    project
}

fn copy_source(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        if matches!(
            entry.file_name().to_str(),
            Some("build" | "target" | ".cargo")
        ) {
            continue;
        }
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_source(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}
