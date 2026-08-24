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

    fn retain<Item, Context: 'static>(_: Value<Item, Context>) {}

    #[geam_macros::function]
    fn retain_value<Item>(value: Value<Item>) -> () {
        retain(value);
    }
}

fn main() {}
