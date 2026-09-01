use geam::provider::{Call, Configuration, EcoString, InitializationError};

pub struct RunState {
    next: i64,
}

fn initialize(configuration: &Configuration) -> Result<RunState, InitializationError> {
    let next = configuration
        .get("start")
        .and_then(|value| value.as_integer())
        .ok_or_else(|| InitializationError::new("configuration key `start` must be an Integer"))?;
    Ok(RunState { next })
}

#[geam::provider(
    id = "geam-counter",
    package = "counter",
    state = RunState,
    initialize = initialize,
    modules = [counter],
)]
pub struct Component;

#[geam::module(path = "counter")]
mod counter {
    use super::{Call, EcoString, RunState};

    #[geam::function]
    fn next(#[geam::call] call: &mut Call<RunState>, label: EcoString) -> EcoString {
        let state = call.state_mut();
        let next = state.next;
        state.next += 1;
        format!("{label}:{next}").into()
    }
}
