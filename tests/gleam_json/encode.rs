use super::{run_fixture, run_fixture_repeated};

#[test]
fn runs_every_official_json_encoder() {
    run_fixture("gleam_json_encode");
}

#[test]
fn roundtrips_and_escapes_official_json_values_across_repeated_runs() {
    run_fixture_repeated("gleam_json_roundtrip");
}
