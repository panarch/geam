use geam_core::provider::{Configuration, InitializationError};

pub struct RunState;

fn initialize(_: &Configuration) -> Result<RunState, InitializationError> {
    Ok(RunState)
}

#[geam_macros::provider(
    id = "unsupported-scalar",
    package = "counter",
    state = RunState,
    initialize = initialize,
    modules = [counter],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "counter", crate_path = geam_core)]
mod counter {
    #[geam_macros::function]
    fn next(value: i64) -> i64 {
        value + 1
    }
}

fn main() {}
