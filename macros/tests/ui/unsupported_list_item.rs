use num_bigint::BigInt;

#[geam_macros::provider(
    package = "lists",
    modules = [lists],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "lists", crate_path = geam_core)]
mod lists {
    use super::BigInt;

    #[geam_macros::function]
    fn length(values: geam_core::List<i64>) -> BigInt {
        values.len().into()
    }
}

fn main() {}
