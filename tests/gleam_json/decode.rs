use super::run_fixture;

#[test]
#[ignore = "requires `gleam deps download` in the gleam_json fixture"]
fn decodes_every_official_json_value_family() {
    run_fixture("gleam_json_decode");
}
