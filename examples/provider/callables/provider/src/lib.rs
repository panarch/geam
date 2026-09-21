use geam::provider::{BigInt, Call, Callback, Factory, HostResult, Value};

#[derive(Default)]
pub struct RunState {
    calls: usize,
}

#[geam::provider(package = "example_callables", state = RunState, modules = [callables])]
pub struct Component;

#[geam::module(path = "example_callables")]
mod callables {
    use super::{BigInt, Call, Callback, Factory, HostResult, RunState, Value};

    #[geam::callable(factory = Add)]
    fn add(
        #[geam::call] call: &mut Call<RunState>,
        #[geam::capture] offset: BigInt,
        value: BigInt,
    ) -> BigInt {
        call.state_mut().calls += 1;
        offset + value
    }

    #[geam::function]
    fn make_adder(
        #[geam::call] call: &mut Call<RunState>,
        #[geam::factory] factory: Factory<Add>,
        offset: BigInt,
    ) -> HostResult<Callback<fn(BigInt) -> BigInt>> {
        call.create(&factory, (offset,))
    }

    #[geam::callable(factory = Constant)]
    fn constant<Item>(#[geam::capture] value: Value<Item>) -> Value<Item> {
        value
    }

    #[geam::function]
    fn make_constant<Item>(
        #[geam::call] call: &mut Call<RunState>,
        #[geam::factory] factory: Factory<Constant<Item>>,
        value: Value<Item>,
    ) -> HostResult<Callback<fn() -> Value<Item>>> {
        call.create(&factory, (value,))
    }

    #[geam::callable(factory = Forward, await)]
    async fn forward<Argument, Output>(
        #[geam::call] call: &mut Call<RunState>,
        #[geam::capture] callback: Callback<fn(Value<Argument>) -> Value<Output>>,
        argument: Value<Argument>,
    ) -> HostResult<Value<Output>> {
        call.invoke(&callback, (argument,)).await
    }

    #[geam::function]
    fn wrap<Argument, Output>(
        #[geam::call] call: &mut Call<RunState>,
        #[geam::factory] factory: Factory<Forward<Argument, Output>>,
        callback: Callback<fn(Value<Argument>) -> Value<Output>>,
    ) -> HostResult<Callback<fn(Value<Argument>) -> Value<Output>>> {
        call.create(&factory, (callback,))
    }

    #[geam::custom(input = ReplyInput)]
    enum Reply<Item> {
        Reply(Callback<fn(Value<Item>) -> BigInt>),
    }

    #[geam::function]
    fn reply<Item>(callback: Callback<fn(Value<Item>) -> BigInt>) -> Reply<Item> {
        Reply::Reply(callback)
    }

    #[geam::function(await)]
    async fn deliver<Item>(
        #[geam::call] call: &mut Call<RunState>,
        reply: ReplyInput<Item>,
        value: Value<Item>,
    ) -> HostResult<BigInt> {
        let ReplyInput::Reply(callback) = reply;
        call.invoke(&callback, (value,)).await
    }

    #[geam::function]
    fn calls(#[geam::call] call: &Call<RunState>) -> BigInt {
        call.state().calls.into()
    }
}
