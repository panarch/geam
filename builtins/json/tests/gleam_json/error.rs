use super::run_fixture;

#[test]
fn preserves_official_json_parse_and_decode_errors() {
    run_fixture("gleam_json_errors");
}
