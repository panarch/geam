use std::process::Command;

#[test]
fn passes_structured_data_and_reuses_a_retained_list() {
    let output = Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-data"))
        .output()
        .expect("the data embedding example should run");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        output.stdout,
        br"accepted: A-1 (3)
rejected: quantity must not be negative
accepted: C-3 (4)
total: 7
"
    );
    assert_eq!(output.stderr, b"");
}
