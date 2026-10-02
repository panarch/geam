use super::workspace_dependencies;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub const OUTPUT: &[u8] = b"guard locals, multi-subject patterns and original clip: ok\n";

pub fn project_root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/projects/guard_constructor_locals");
    static PREPARED: OnceLock<Result<(), String>> = OnceLock::new();
    workspace_dependencies::prepare(
        &PREPARED,
        &root,
        "gleam",
        &["deps", "download"],
        "locked guard constructor fixture dependencies",
    );
    root
}

pub fn copy_project(destination: &Path) {
    let source = project_root();
    fs::create_dir_all(destination).unwrap();
    for file in ["gleam.toml", "manifest.toml"] {
        fs::copy(source.join(file), destination.join(file)).unwrap();
    }
    copy_directory(&source.join("src"), &destination.join("src"));
    copy_directory(
        &source.join("build/packages"),
        &destination.join("build/packages"),
    );
}

fn copy_directory(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let destination = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_directory(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).unwrap();
        }
    }
}
