#[derive(Default)]
pub struct State {
    pub calls: usize,
}

#[geam::provider(package = "application_exit_fixture", state = State, modules = [native])]
pub struct Component;

#[geam::module(path = "application_exit_fixture")]
mod native {
    use super::State;
    use geam::provider::{BigInt, Call, ExitStatus, HostResult};

    #[geam::function]
    fn exit(#[geam::call] call: &mut Call<State>, status: BigInt) -> HostResult<()> {
        let status = ExitStatus::try_from(&status)?;
        call.state_mut().calls += 1;
        call.exit(status)
    }

    #[geam::function(await)]
    async fn exit_async(#[geam::call] call: &mut Call<State>, status: BigInt) -> HostResult<()> {
        let status = ExitStatus::try_from(&status)?;
        call.with_state(|state| state.calls += 1).await?;
        call.exit(status)
    }

    #[geam::function]
    fn calls(#[geam::call] call: &Call<State>) -> BigInt {
        call.state().calls.into()
    }
}
