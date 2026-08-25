use geam::provider::{Call, Callback, HostResult, Stored, Value};

#[geam::provider(
    package = "example_generic_box",
    modules = [generic_box],
)]
pub struct Component;

#[geam::module(path = "example_generic_box")]
mod generic_box {
    use super::{Call, Callback, HostResult, Stored, Value};

    #[geam::external(
        name = "Box",
        parameters = [Item],
        input = BoxInput,
    )]
    pub struct BoxValue<Item> {
        #[geam::stored]
        value: Stored<Item>,
    }

    #[geam::function]
    fn new<Item>(#[geam::call] call: &mut Call<()>, value: Value<Item>) -> BoxValue<Item> {
        BoxValue {
            value: call.store(value),
        }
    }

    #[geam::function]
    fn get<Item>(#[geam::call] call: &mut Call<()>, boxed: BoxInput<Item>) -> Value<Item> {
        call.restore(boxed.value())
    }

    #[geam::function]
    fn replace<Old, New>(
        #[geam::call] call: &mut Call<()>,
        _boxed: BoxInput<Old>,
        value: Value<New>,
    ) -> BoxValue<New> {
        BoxValue {
            value: call.store(value),
        }
    }

    #[geam::function]
    fn contains<Item>(
        #[geam::call] call: &mut Call<()>,
        boxed: BoxInput<Item>,
        expected: Value<Item>,
    ) -> bool {
        let value = call.restore(boxed.value());
        call.equal(&value, &expected)
    }

    #[geam::function]
    fn map<Input, Output>(
        #[geam::call] call: &mut Call<()>,
        boxed: BoxInput<Input>,
        mapper: Callback<fn(Value<Input>) -> Value<Output>>,
    ) -> HostResult<BoxValue<Output>> {
        let value = call.restore(boxed.value());
        let mapped = call.invoke(mapper, (value,))?;
        Ok(BoxValue {
            value: call.store(mapped),
        })
    }
}
