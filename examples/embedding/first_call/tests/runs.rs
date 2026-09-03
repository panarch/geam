use std::process::Command;

#[test]
fn calls_the_generated_gleam_function() {
    let output = Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-first-call"))
        .output()
        .expect("the first-call embedding example should run");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(output.stdout, b"42\n");
    assert_eq!(output.stderr, b"");
}
