use geam_core::{HostModule, HostProviderSet, Value};
use geam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, host_providers};

use super::{ExpectedSurface, assert_surface, run_hosted_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "absolute_value",
        "add",
        "ceiling",
        "clamp",
        "compare",
        "divide",
        "exponential",
        "floor",
        "logarithm",
        "loosely_compare",
        "loosely_equals",
        "max",
        "min",
        "modulo",
        "multiply",
        "negate",
        "parse",
        "power",
        "product",
        "random",
        "round",
        "square_root",
        "subtract",
        "sum",
        "to_precision",
        "to_string",
        "truncate",
    ],
    types: &[],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
absolute_value: fn(Float) -> Float
add: fn(Float, Float) -> Float
ceiling: fn(Float) -> Float
clamp: fn(Float, min: Float, max: Float) -> Float
compare: fn(Float, with: Float) -> Order
divide: fn(Float, by: Float) -> Result(Float, Nil)
exponential: fn(Float) -> Float
floor: fn(Float) -> Float
logarithm: fn(Float) -> Result(Float, Nil)
loosely_compare: fn(Float, with: Float, tolerating: Float) -> Order
loosely_equals: fn(Float, with: Float, tolerating: Float) -> Bool
max: fn(Float, Float) -> Float
min: fn(Float, Float) -> Float
modulo: fn(Float, by: Float) -> Result(Float, Nil)
multiply: fn(Float, Float) -> Float
negate: fn(Float) -> Float
parse: fn(String) -> Result(Float, Nil)
power: fn(Float, of: Float) -> Result(Float, Nil)
product: fn(List(Float)) -> Float
random: fn() -> Float
round: fn(Float) -> Int
square_root: fn(Float) -> Result(Float, Nil)
subtract: fn(Float, Float) -> Float
sum: fn(List(Float)) -> Float
to_precision: fn(Float, Int) -> Float
to_string: fn(Float) -> String
truncate: fn(Float) -> Int
"#,
};

#[test]
fn tracks_official_gleam_float_public_surface() {
    assert_surface(
        "gleam_float",
        "gleam/float",
        &["gleam/order", "gleam/float"],
        &SURFACE,
    );
}

#[test]
fn runs_official_gleam_float_behavior() {
    let providers =
        host_providers::<GleamStdlibProfile>().expect("official stdlib providers should register");
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<GleamStdlibProfile>>::new(), providers)
            .expect("official stdlib provider modules should be unique");
    let value = run_hosted_fixture(
        "gleam_float",
        &["gleam/order", "gleam/float"],
        hosts,
        &mut GleamStdlibRunState::from_seed([4; 32]),
    );

    assert_eq!(value, Value::Nil);
}
