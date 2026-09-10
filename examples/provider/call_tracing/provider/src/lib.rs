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

    #[geam::function(resumable)]
    async fn record_later(
        #[geam::call] call: &mut Call<RunState>,
        entry: EcoString,
    ) -> HostResult<()> {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        call.with_state(move |state| state.entries.push(entry))
            .await?;
        Ok(())
    }

    #[geam::function(resumable)]
    async fn around<Item>(
        #[geam::call] call: &mut Call<RunState>,
        callback: Callback<fn() -> Value<Item>>,
    ) -> HostResult<Value<Item>> {
        call.with_state(|state| state.entries.push("before".into()))
            .await?;
        let returned = call.invoke(&callback, ()).await?;
        call.with_state(|state| state.entries.push("after".into()))
            .await?;
        Ok(returned)
    }

    #[geam::function]
    fn entries(#[geam::call] call: &Call<RunState>) -> Vec<EcoString> {
        call.state().entries.clone()
    }
}
