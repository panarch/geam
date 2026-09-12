use super::{
    GleamStdlibRunState, GleamTimeRunState, ScriptedEvent, ScriptedSource, fixture_execution,
    fixture_expected,
};
use std::time::{Duration, UNIX_EPOCH};

#[test]
fn preserves_time_source_order_backward_clocks_repeated_runs_and_independent_state() {
    let mut execution = fixture_execution("gleam_time_effects");
    let expected = fixture_expected("gleam_time_effects");
    let mut first_state = scripted_state();
    let mut independent_state = scripted_state();

    let first = crate::execution_fixture::run(&mut execution, &mut first_state, &mut Vec::new())
        .expect("official Time effects fixture should run");
    let repeated = crate::execution_fixture::run(&mut execution, &mut first_state, &mut Vec::new())
        .expect("official Time effects fixture should repeat");
    let independent =
        crate::execution_fixture::run(&mut execution, &mut independent_state, &mut Vec::new())
            .expect("official Time effects fixture should use independent state");

    let mut transferred = super::transfer::fixture("gleam_time_effects");
    let mut transfer_first = super::transfer::RunState {
        stdlib: GleamStdlibRunState::from_seed([1; 32]),
        source: scripted_state().source().clone(),
        work: (),
    };
    let mut transfer_independent = super::transfer::RunState {
        stdlib: GleamStdlibRunState::from_seed([1; 32]),
        source: scripted_state().source().clone(),
        work: (),
    };
    for (index, actual) in [first, repeated, independent].into_iter().enumerate() {
        assert_eq!(actual.inspect().to_string(), expected);
        let state = if index == 2 {
            &mut transfer_independent
        } else {
            &mut transfer_first
        };
        let mut echo = super::transfer_fixture::ObservedEcho::default();
        transferred
            .run(state, &mut echo)
            .expect("repeated transferable Time effects");
        echo.assert_result(&actual, &[]);
    }
    assert!(first_state.source().events.is_empty());
    assert!(transfer_first.source.events.is_empty());
    assert_eq!(independent_state.source().events.len(), 4);
    assert_eq!(transfer_independent.source.events.len(), 4);
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
