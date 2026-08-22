#[test]
fn provider_type_restrictions_are_compile_time_contracts() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/*.rs");
}
