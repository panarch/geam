use super::{ScriptedEvent, ScriptedSource, run_fixture};

#[test]
fn runs_official_duration_units_normalization_arithmetic_and_conversions() {
    run_fixture(
        "gleam_time_duration",
        ScriptedSource::new(Vec::<ScriptedEvent>::new()),
    );
}
