use super::{ScriptedEvent, ScriptedSource, run_fixture};
use std::time::{Duration, UNIX_EPOCH};

#[test]
fn runs_official_timestamp_arithmetic_calendar_and_system_time() {
    run_fixture(
        "gleam_time_timestamp",
        ScriptedSource::new([ScriptedEvent::SystemTime(
            UNIX_EPOCH + Duration::new(1_700_000_000, 123_456_789),
        )]),
    );
}

#[test]
fn runs_official_rfc3339_formatting_parsing_and_normalization() {
    run_fixture(
        "gleam_time_rfc3339",
        ScriptedSource::new(Vec::<ScriptedEvent>::new()),
    );
}
