use ecow::EcoString;
use geam::provider::Call;
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
    use super::{BigInt, Call, EcoString, RunState};

    #[geam::function]
    fn next(#[geam::call] call: &mut Call<RunState>) -> EcoString {
        let state = call.state_mut();
        state.issued += 1;
        format!("request-{}", state.issued).into()
    }

    #[geam::function]
    fn issued(#[geam::call] call: &Call<RunState>) -> BigInt {
        BigInt::from(call.state().issued)
    }
}
