#[test]
fn retains_an_opaque_session_between_calls() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-session"))
        .output()
        .expect("session example should start");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"original: 40\nnext: 42\n");
    assert!(output.stderr.is_empty());
}
