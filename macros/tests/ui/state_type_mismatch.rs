use geam_core::provider::{Configuration, InitializationError};

pub struct RunState;
struct OtherState;

fn initialize(_: &Configuration) -> Result<RunState, InitializationError> {
    Ok(RunState)
}

#[geam_macros::provider(
    id = "mismatched-state",
    package = "counter",
    state = RunState,
    initialize = initialize,
    modules = [counter],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "counter", crate_path = geam_core)]
mod counter {
    use super::OtherState;

    fn helper() -> bool {
        true
    }

    #[geam_macros::function]
    fn next(#[geam_macros::state] _: &OtherState) -> bool {
        helper()
    }
}

fn main() {}
