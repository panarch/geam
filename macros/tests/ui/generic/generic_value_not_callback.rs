use geam_core::provider::Value;

#[geam_macros::provider(
    package = "generic_values",
    modules = [generic_values],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "generic_values", crate_path = geam_core)]
mod generic_values {
    use super::Value;

    #[geam_macros::function]
    fn invoke<Item>(
        callback: Value<fn(Item) -> Item>,
        value: Value<Item>,
    ) -> Value<Item> {
        callback(value)
    }
}

fn main() {}
