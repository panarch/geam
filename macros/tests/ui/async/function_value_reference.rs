use geam_core::provider::{BigInt, FunctionValue};

#[geam_macros::module(path = "function_value_reference", crate_path = geam_core)]
mod native {
    use super::{BigInt, FunctionValue};

    #[geam_macros::function]
    fn borrowed(_function: &FunctionValue<fn(BigInt) -> BigInt>) -> () {}
}

fn main() {}
