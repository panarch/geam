use std::path::PathBuf;
use std::process::Command;

#[test]
fn sibling_crate_fixture_compiles_links_and_runs() {
    let macros = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest = macros.join("tests/fixtures/cross_crate/Cargo.toml");
    let target = macros
        .parent()
        .expect("macros package should be inside the workspace")
        .join("target/macro-cross-crate-fixture");
    let mut command = Command::new(env!("CARGO"));
    command
        .args(["test", "--workspace", "--locked", "--quiet"])
        .arg("--manifest-path")
        .arg(&manifest)
        .env("CARGO_TARGET_DIR", target);
    remove_nested_cargo_instrumentation(&mut command);
    let output = command
        .output()
        .expect("cross-crate fixture Cargo process should start");

    assert!(
        output.status.success(),
        "cross-crate fixture failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

fn remove_nested_cargo_instrumentation(command: &mut Command) {
    command
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_LLVM_COV")
        .env_remove("CARGO_LLVM_COV_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .env_remove("RUSTDOCFLAGS");
}
