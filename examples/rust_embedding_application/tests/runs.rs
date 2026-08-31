use std::process::Command;

#[test]
fn runs_the_managed_provider_backed_embedding() {
    let output = Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-application"))
        .output()
        .expect("the embedding application should run");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        output.stdout,
        br"validating inventory
Inventory validation:
  AB-12: 3
  Row 2 rejected: invalid code
  C-7: 4
  Row 4 rejected: quantity must not be negative
Total quantity: 7
First valid item: AB-12 (3)
"
    );
    assert_eq!(output.stderr, b"");
}
