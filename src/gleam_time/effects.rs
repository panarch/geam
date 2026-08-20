use super::GleamTimeRunState;
use super::test_support::{ScriptedSource, execution};
use crate::gleam_stdlib::GleamStdlibRunState;
use std::time::{Duration, UNIX_EPOCH};

const MAIN_SOURCE: &str = r#"
import gleam/time/calendar
import gleam/time/timestamp

pub fn main() {
  #(
    timestamp.current_parts(),
    calendar.current_offset(),
    timestamp.current_parts(),
    calendar.current_offset(),
  )
}
"#;

#[test]
fn executes_non_monotonic_time_and_changing_offsets_in_source_order() {
    let execution = execution::<ScriptedSource>(MAIN_SOURCE, "main");
    let mut first_state = scripted_state();
    let mut independent_state = scripted_state();

    let first = execution
        .run_main(&mut first_state, &mut Vec::new())
        .expect("scripted Time source should run");
    let repeated = execution
        .run_main(&mut first_state, &mut Vec::new())
        .expect("scripted Time source should run repeatedly");
    let independent = execution
        .run_main(&mut independent_state, &mut Vec::new())
        .expect("independent scripted Time source should run");

    assert_eq!(
        first.inspect().to_string(),
        "#(#(5, 0), 3600, #(-1, 999999999), -18000)",
    );
    assert_eq!(
        repeated.inspect().to_string(),
        "#(#(8, 7), 7200, #(-3, 999999997), 0)",
    );
    assert_eq!(independent, first);
    assert!(first_state.source().times.is_empty());
    assert!(first_state.source().offsets.is_empty());
}

fn scripted_state() -> GleamTimeRunState<ScriptedSource> {
    GleamTimeRunState::new(
        GleamStdlibRunState::from_seed([3; 32]),
        ScriptedSource {
            times: [
                Ok(UNIX_EPOCH + Duration::from_secs(5)),
                Ok(UNIX_EPOCH - Duration::from_nanos(1)),
                Ok(UNIX_EPOCH + Duration::new(8, 7)),
                Ok(UNIX_EPOCH - Duration::new(2, 3)),
            ]
            .into(),
            offsets: [Ok(3600), Ok(-18_000), Ok(7200), Ok(0)].into(),
        },
    )
}
