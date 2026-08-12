use super::{
    GleamStdlibRunState, GleamTimeRunState, ScriptedEvent, ScriptedSource, fixture_execution,
    fixture_expected,
};
use std::time::{Duration, UNIX_EPOCH};

#[test]
fn preserves_time_source_order_backward_clocks_repeated_runs_and_independent_state() {
    let execution = fixture_execution("gleam_time_effects");
    let expected = fixture_expected("gleam_time_effects");
    let mut first_state = scripted_state();
    let mut independent_state = scripted_state();

    let first = execution
        .run_main(&mut first_state, &mut Vec::new())
        .expect("official Time effects fixture should run");
    let repeated = execution
        .run_main(&mut first_state, &mut Vec::new())
        .expect("official Time effects fixture should repeat");
    let independent = execution
        .run_main(&mut independent_state, &mut Vec::new())
        .expect("official Time effects fixture should use independent state");

    for actual in [first, repeated, independent] {
        assert_eq!(actual.inspect().to_string(), expected);
    }
}

fn scripted_state() -> GleamTimeRunState<ScriptedSource> {
    GleamTimeRunState::new(
        GleamStdlibRunState::from_seed([1; 32]),
        ScriptedSource::new([
            ScriptedEvent::SystemTime(UNIX_EPOCH + Duration::from_secs(5)),
            ScriptedEvent::LocalOffset(3600),
            ScriptedEvent::SystemTime(UNIX_EPOCH - Duration::from_nanos(1)),
            ScriptedEvent::LocalOffset(-18_000),
            ScriptedEvent::SystemTime(UNIX_EPOCH + Duration::from_secs(5)),
            ScriptedEvent::LocalOffset(3600),
            ScriptedEvent::SystemTime(UNIX_EPOCH - Duration::from_nanos(1)),
            ScriptedEvent::LocalOffset(-18_000),
        ]),
    )
}
