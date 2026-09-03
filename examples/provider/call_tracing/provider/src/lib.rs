use geam::provider::{Call, Callback, EcoString, HostResult, Value};

#[derive(Default)]
pub struct RunState {
    entries: Vec<EcoString>,
}

#[geam::provider(
    package = "example_call_tracing",
    state = RunState,
    modules = [call_tracing],
)]
pub struct Component;

#[geam::module(path = "example_call_tracing")]
mod call_tracing {
    use super::{Call, Callback, EcoString, HostResult, RunState, Value};

    #[geam::function]
    fn record(#[geam::call] call: &mut Call<RunState>, entry: EcoString) -> () {
        call.state_mut().entries.push(entry);
    }

    #[geam::function]
    fn around<Item>(
        #[geam::call] call: &mut Call<RunState>,
        callback: Callback<fn() -> Value<Item>>,
    ) -> HostResult<Value<Item>> {
        call.state_mut().entries.push("before".into());
        let returned = call.invoke(callback, ())?;
        call.state_mut().entries.push("after".into());
        Ok(returned)
    }

    #[geam::function]
    fn entries(#[geam::call] call: &Call<RunState>) -> Vec<EcoString> {
        call.state().entries.clone()
    }
}
