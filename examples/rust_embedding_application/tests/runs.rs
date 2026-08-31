use std::process::Command;

#[test]
fn runs_the_managed_provider_backed_embedding() {
    let output = Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-application"))
        .output()
        .expect("the embedding application should run");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(output.stdout, b"total quantity: 7\n");
    assert_eq!(output.stderr, b"");
}
