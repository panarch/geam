use std::process::Command;

#[test]
fn native_callbacks_match_in_dynamic_and_prepared_execution() {
    let output = Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-callables"))
        .output()
        .expect("the callable embedding application should run");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        output.stdout,
        b"dynamic: 15, 17, 19\nprepared: 15, 17, 19\n"
    );
    assert_eq!(output.stderr, b"");
}
