use std::process::Command;

const CHILD: &str = "GEAM_TRYBUILD_CHILD";

#[test]
fn provider_type_restrictions_are_compile_time_contracts() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/*.rs");
}

#[test]
fn generic_value_type_restrictions_are_compile_time_contracts() {
    if std::env::var_os(CHILD).is_none() {
        run_without_nested_cargo_instrumentation(
            "generic_value_type_restrictions_are_compile_time_contracts",
        );
        return;
    }

    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/generic/*.rs");
}

fn run_without_nested_cargo_instrumentation(test: &str) {
    let target = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the macros package should be inside the workspace")
        .join("target/trybuild-uninstrumented");
    let mut command = Command::new(
        std::env::current_exe().expect("the UI test executable path should be available"),
    );
    command
        .args(["--exact", test, "--nocapture"])
        .env(CHILD, "1")
        .env("CARGO_TARGET_DIR", target)
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_LLVM_COV")
        .env_remove("CARGO_LLVM_COV_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .env_remove("RUSTDOCFLAGS");
    let output = command
        .output()
        .expect("the isolated UI test process should start");

    assert!(
        output.status.success(),
        "isolated UI tests failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
