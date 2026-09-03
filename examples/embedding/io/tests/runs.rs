use std::process::Command;

#[test]
fn keeps_gleam_io_caller_owned() {
    let output = Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-io"))
        .output()
        .expect("the IO embedding example should run");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(output.stdout, b"Hello, Rust!\nreturned: Hello, Rust!\n");
    assert_eq!(output.stderr, b"");
}
