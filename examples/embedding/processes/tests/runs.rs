#[test]
fn retains_live_pid_and_subject_handles_across_rust_calls() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-processes"))
        .output()
        .expect("process example should start");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"first: 20\nsecond: 42\nstopped: true\n");
    assert!(output.stderr.is_empty());
}
