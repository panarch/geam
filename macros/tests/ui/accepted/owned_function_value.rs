use geam_core::provider::{BigInt, FunctionValue, StringValue, Value};

fn require_static<Owned: 'static>(_value: Owned) {}

#[geam_macros::provider(
    package = "owned_function_value",
    modules = [native],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "owned_function_value/native", crate_path = geam_core)]
mod native {
    use super::{BigInt, FunctionValue, StringValue, Value, require_static};

    #[geam_macros::function(await)]
    async fn retain_owned(
        function: FunctionValue<fn(BigInt) -> BigInt>,
    ) -> FunctionValue<fn(BigInt) -> BigInt> {
        require_static(function.clone());
        function
    }

    #[geam_macros::function]
    fn retain<Item>(
        function: FunctionValue<fn(Value<Item>) -> StringValue>,
    ) -> FunctionValue<fn(Value<Item>) -> StringValue> {
        function.clone()
    }
}

fn main() {}
