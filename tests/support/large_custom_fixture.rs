use std::fs;
use std::path::Path;

pub fn create_project(destination: &Path) {
    fs::create_dir_all(destination.join("src")).unwrap();
    fs::write(
        destination.join("gleam.toml"),
        "name = \"custom_repro\"\nversion = \"1.0.0\"\n",
    )
    .unwrap();
    fs::write(
        destination.join("src/custom_repro.gleam"),
        concat!(
            include_str!(
                "../../macros/tests/fixtures/cross_crate/declarations/gleam/declarations.gleam"
            ),
            "\n",
            include_str!("../../macros/tests/fixtures/cross_crate/declarations/gleam/checks.gleam"),
        ),
    )
    .unwrap();
}

pub fn create_provider(destination: &Path) {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repository_path = repository.to_str().unwrap();
    let core = repository.join("core");
    let macros = repository.join("macros");
    let core = core.to_str().unwrap();
    let macros = macros.to_str().unwrap();
    fs::create_dir_all(destination.join("src")).unwrap();
    fs::write(
        destination.join("Cargo.toml"),
        toml::to_string(&toml::toml! {
            [package]
            name = "geam-large-custom-fixture"
            version = "1.0.0"
            edition = "2024"
            [package.metadata.geam.provider]
            schema = 2
            gleam-package = "custom_repro"
            gleam-version = ">= 1.0.0 and < 2.0.0"
            component = "plain"
            execution-service = false
            requires-services = []
            [dependencies]
            num-bigint = "0.4.6"
            [dependencies.geam]
            path = repository_path
            default-features = false
            features = ["provider"]
            [dependencies.geam-core]
            path = core
            [dependencies.geam-macros]
            path = macros
            [workspace]
        })
        .unwrap(),
    )
    .unwrap();
    fs::write(
        destination.join("src/lib.rs"),
        "mod large_customs;\npub use large_customs::Component;\n",
    )
    .unwrap();
    fs::copy(
        repository.join("macros/tests/fixtures/cross_crate/declarations/src/large_customs.rs"),
        destination.join("src/large_customs.rs"),
    )
    .unwrap();
}

pub fn configure_cargo(destination: &Path) {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repository_path = repository.to_str().unwrap();
    let target = repository.join("target/prepared-acceptance");
    let target_path = target.to_str().unwrap();
    fs::create_dir_all(destination.join(".cargo")).unwrap();
    fs::write(
        destination.join(".cargo/config.toml"),
        toml::to_string(&toml::toml! {
            [patch.crates-io.geam]
            path = repository_path
            [net]
            offline = true
            [build]
            target-dir = target_path
        })
        .unwrap(),
    )
    .unwrap();
}
