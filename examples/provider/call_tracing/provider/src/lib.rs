use geam::provider::{Call, Callback, HostResult, StringValue, Value};

#[derive(Default)]
pub struct RunState {
    entries: Vec<StringValue>,
}

#[geam::provider(
    package = "example_call_tracing",
    state = RunState,
    modules = [call_tracing],
)]
pub struct Component;

#[geam::module(path = "example_call_tracing")]
mod call_tracing {
    use super::{Call, Callback, HostResult, RunState, StringValue, Value};

    #[geam::function]
    fn record(#[geam::call] call: &mut Call<RunState>, entry: StringValue) -> () {
        call.state_mut().entries.push(entry);
    }

    #[geam::function(await)]
    async fn record_later(
        #[geam::call] call: &mut Call<RunState>,
        entry: StringValue,
    ) -> HostResult<()> {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        call.with_state(move |state| state.entries.push(entry))
            .await?;
        Ok(())
    }

    #[geam::function(await)]
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
    fn entries(#[geam::call] call: &Call<RunState>) -> Vec<StringValue> {
        call.state().entries.clone()
    }
}
