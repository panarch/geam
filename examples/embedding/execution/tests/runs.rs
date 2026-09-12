#[test]
fn cancels_a_running_gleam_entry_and_keeps_the_module_usable() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-execution"))
        .output()
        .expect("execution example should start");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        b"Rust made progress while Gleam was running\nafter cancellation: 42\n"
    );
    assert!(output.stderr.is_empty());
}
