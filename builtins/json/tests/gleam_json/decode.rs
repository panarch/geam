use super::run_fixture;

#[test]
fn decodes_every_official_json_value_family() {
    run_fixture("gleam_json_decode");
}
