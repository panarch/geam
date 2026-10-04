use geam_core::provider::Value;

#[geam_macros::provider(
    package = "opaque_function_value",
    modules = [native],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "opaque_function_value/native", crate_path = geam_core)]
mod native {
    use super::Value;

    #[geam_macros::function]
    fn capability<Item>(function: Value<fn(Item) -> Item>) -> bool {
        function.callback().is_some()
    }
}

fn main() {}
