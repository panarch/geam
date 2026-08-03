use super::run_fixture;

#[test]
#[ignore = "requires `gleam deps download` in the gleam_json fixture"]
fn preserves_official_json_parse_and_decode_errors() {
    run_fixture("gleam_json_errors");
}
