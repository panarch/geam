use geam_core::provider::{Call, Callback, HostResult, Value};
use num_bigint::BigInt;

#[derive(Default)]
pub struct RunState {
    calls: usize,
}

#[geam_macros::provider(
    package = "callback_borrow",
    state = RunState,
    modules = [callback_borrow],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "callback_borrow", crate_path = geam_core)]
mod callback_borrow {
    use super::{BigInt, Call, Callback, HostResult, RunState, Value};

    #[geam_macros::external(name = "Token")]
    #[derive(PartialEq, Eq, Hash)]
    struct Token;

    #[geam_macros::function]
    fn invoke<Item>(
        #[geam_macros::call] call: &mut Call<RunState>,
        callback: Callback<
            fn(Value<Item>, ((BigInt, self::Token), bool)) -> Value<Item>,
        >,
        value: Value<Item>,
    ) -> HostResult<Value<Item>> {
        let state = call.state_mut();
        let returned = call.invoke(callback, (value, ((1.into(), Token), true)))?;
        state.calls += 1;
        Ok(returned)
    }
}

fn main() {}
