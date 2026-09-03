use std::process::Command;

#[test]
fn calls_a_locked_gleam_package() {
    let output = Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-package"))
        .output()
        .expect("the package embedding example should run");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(output.stdout, b"first: Gleam\nempty: none\n");
    assert_eq!(output.stderr, b"");
}
