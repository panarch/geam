use std::process::Command;

#[test]
fn awaits_an_async_host_with_state_and_callback_reentry() {
    let output = Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-async-host"))
        .output()
        .expect("the async-host embedding example should run");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        output.stdout,
        b"value: 46\ncompleted: 1\ninput: 20\nafter host: 45\n"
    );
    assert_eq!(output.stderr, b"");
}
