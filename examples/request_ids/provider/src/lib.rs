use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Default)]
pub struct RunState {
    issued: i64,
}

#[geam::provider(
    package = "example_request_ids",
    state = RunState,
    modules = [request_ids],
)]
pub struct Component;

#[geam::module(path = "example_request_ids")]
mod request_ids {
    use super::{BigInt, EcoString, RunState};

    #[geam::function]
    fn next(#[geam::state] state: &mut RunState) -> EcoString {
        state.issued += 1;
        format!("request-{}", state.issued).into()
    }

    #[geam::function]
    fn issued(#[geam::state] state: &RunState) -> BigInt {
        BigInt::from(state.issued)
    }
}
