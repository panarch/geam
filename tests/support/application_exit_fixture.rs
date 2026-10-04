use std::fs;
use std::path::{Path, PathBuf};

pub fn copy(root: &Path) -> (PathBuf, PathBuf) {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture = repository.join("tests/fixtures/application_exit");
    let project = root.join("project");
    let provider = root.join("provider");
    fs::create_dir_all(project.join("src")).unwrap();
    fs::create_dir_all(provider.join("src")).unwrap();
    for name in [
        "gleam.toml",
        "manifest.toml",
        "src/application_exit_fixture.gleam",
    ] {
        fs::copy(fixture.join("project").join(name), project.join(name)).unwrap();
    }
    fs::copy(
        fixture.join("provider/src/lib.rs"),
        provider.join("src/lib.rs"),
    )
    .unwrap();
    fs::copy(
        fixture.join("provider/Cargo.lock"),
        provider.join("Cargo.lock"),
    )
    .unwrap();
    let mut manifest: toml::Table = fs::read_to_string(fixture.join("provider/Cargo.toml"))
        .unwrap()
        .parse()
        .unwrap();
    for dependencies in ["dependencies", "dev-dependencies"] {
        manifest[dependencies]["geam"]["path"] = repository.to_str().unwrap().into();
    }
    fs::write(
        provider.join("Cargo.toml"),
        toml::to_string(&manifest).unwrap(),
    )
    .unwrap();
    fs::create_dir_all(project.join(".cargo")).unwrap();
    let path = repository.to_str().unwrap();
    fs::write(
        project.join(".cargo/config.toml"),
        toml::to_string(&toml::toml! {
            [patch.crates-io.geam]
            path = path
            [net]
            offline = true
        })
        .unwrap(),
    )
    .unwrap();
    (project, provider)
}
