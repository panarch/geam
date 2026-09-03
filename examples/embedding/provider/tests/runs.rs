use std::process::Command;

#[test]
fn calls_a_selected_external_provider() {
    let output = Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-provider"))
        .output()
        .expect("the provider embedding example should run");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(output.stdout, b"matched: true\n");
    assert_eq!(output.stderr, b"");
}
