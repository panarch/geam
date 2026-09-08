use std::process::Command;

#[test]
fn work_representation_and_scope_are_compile_time_contracts() {
    const CHILD: &str = "GEAM_RUNTIME_API_TRYBUILD_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let output = Command::new(std::env::current_exe().expect("UI test executable"))
            .args([
                "--exact",
                "work_representation_and_scope_are_compile_time_contracts",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .env(
                "CARGO_TARGET_DIR",
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../../target/trybuild-uninstrumented"
                ),
            )
            .env_remove("CARGO_ENCODED_RUSTFLAGS")
            .env_remove("CARGO_LLVM_COV")
            .env_remove("CARGO_LLVM_COV_TARGET_DIR")
            .env_remove("RUSTFLAGS")
            .env_remove("RUSTDOCFLAGS")
            .output()
            .expect("isolated UI test");
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }
    let cases = trybuild::TestCases::new();
    cases.pass("tests/ui/accepted.rs");
    cases.compile_fail("tests/ui/rejected/*.rs");
}
