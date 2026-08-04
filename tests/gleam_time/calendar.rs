use super::{ScriptedEvent, ScriptedSource, run_fixture};

#[test]
#[ignore = "requires `gleam deps download` in the gleam_time fixture"]
fn runs_official_calendar_month_validation_comparison_and_local_offset() {
    run_fixture(
        "gleam_time_calendar",
        ScriptedSource::new([ScriptedEvent::LocalOffset(3600)]),
    );
}
