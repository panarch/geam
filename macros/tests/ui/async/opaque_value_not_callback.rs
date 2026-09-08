use geam_core::provider::Value;

#[geam_macros::provider(
    package = "async_opaque_value",
    modules = [native],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "async_opaque_value/native", crate_path = geam_core)]
mod native {
    use super::Value;

    #[geam_macros::function]
    async fn invoke<Item>(
        callback: Value<fn(Item) -> Item>,
        value: Value<Item>,
    ) -> Value<Item> {
        std::future::ready(()).await;
        callback(value)
    }
}

fn main() {}
