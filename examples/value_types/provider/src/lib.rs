use ecow::EcoString;
use geam::BitArrayValue;
use num_bigint::BigInt;

#[geam::provider(
    package = "example_value_types",
    modules = [scalars, tuples],
)]
pub struct Component;

#[geam::module(path = "example_value_types/scalars")]
mod scalars {
    use super::{BigInt, BitArrayValue, EcoString};

    #[geam::function]
    fn join(left: EcoString, right: EcoString) -> EcoString {
        format!("{left}:{right}").into()
    }

    #[geam::function]
    fn add(left: BigInt, right: BigInt) -> BigInt {
        left + right
    }

    #[geam::function]
    fn multiply(left: f64, right: f64) -> f64 {
        left * right
    }

    #[geam::function]
    fn keep_bits(value: BitArrayValue) -> BitArrayValue {
        value
    }

    #[geam::function]
    fn keep_codepoint(value: char) -> char {
        value
    }

    #[geam::function]
    fn invert(value: bool) -> bool {
        !value
    }

    #[geam::function]
    fn keep_nil(value: ()) -> () {
        value
    }
}

#[geam::module(path = "example_value_types/tuples")]
mod tuples {
    use super::{BigInt, EcoString};

    #[geam::function]
    fn wrap(value: EcoString) -> (EcoString,) {
        (value,)
    }

    #[geam::function]
    fn unwrap(value: (EcoString,)) -> EcoString {
        value.0
    }

    #[geam::function]
    fn swap(value: (EcoString, BigInt)) -> (BigInt, EcoString) {
        let (label, count) = value;
        (count, label)
    }

    #[geam::function]
    fn rotate(value: (EcoString, f64, bool)) -> (bool, EcoString, f64) {
        let (label, measurement, enabled) = value;
        (enabled, label, measurement)
    }

    #[geam::function]
    fn reassociate(value: (EcoString, (BigInt, bool))) -> ((EcoString, BigInt), bool) {
        let (label, (count, enabled)) = value;
        ((label, count), enabled)
    }
}
