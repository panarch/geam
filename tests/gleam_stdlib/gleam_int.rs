use geam::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, host_providers};
use geam::{HostModule, HostProviderSet, Value};

use super::{ExpectedSurface, assert_surface, run_hosted_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "absolute_value",
        "add",
        "base_parse",
        "bitwise_and",
        "bitwise_exclusive_or",
        "bitwise_not",
        "bitwise_or",
        "bitwise_shift_left",
        "bitwise_shift_right",
        "clamp",
        "compare",
        "divide",
        "floor_divide",
        "is_even",
        "is_odd",
        "max",
        "min",
        "modulo",
        "multiply",
        "negate",
        "parse",
        "power",
        "product",
        "random",
        "range",
        "remainder",
        "square_root",
        "subtract",
        "sum",
        "to_base16",
        "to_base2",
        "to_base36",
        "to_base8",
        "to_base_string",
        "to_float",
        "to_string",
    ],
    types: &[],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
absolute_value: fn(Int) -> Int
add: fn(Int, Int) -> Int
base_parse: fn(String, Int) -> Result(Int, Nil)
bitwise_and: fn(Int, Int) -> Int
bitwise_exclusive_or: fn(Int, Int) -> Int
bitwise_not: fn(Int) -> Int
bitwise_or: fn(Int, Int) -> Int
bitwise_shift_left: fn(Int, Int) -> Int
bitwise_shift_right: fn(Int, Int) -> Int
clamp: fn(Int, min: Int, max: Int) -> Int
compare: fn(Int, with: Int) -> Order
divide: fn(Int, by: Int) -> Result(Int, Nil)
floor_divide: fn(Int, by: Int) -> Result(Int, Nil)
is_even: fn(Int) -> Bool
is_odd: fn(Int) -> Bool
max: fn(Int, Int) -> Int
min: fn(Int, Int) -> Int
modulo: fn(Int, by: Int) -> Result(Int, Nil)
multiply: fn(Int, Int) -> Int
negate: fn(Int) -> Int
parse: fn(String) -> Result(Int, Nil)
power: fn(Int, of: Float) -> Result(Float, Nil)
product: fn(List(Int)) -> Int
random: fn(Int) -> Int
range: fn(from: Int, to: Int, with: acc, run: fn(acc, Int) -> acc) -> acc
remainder: fn(Int, by: Int) -> Result(Int, Nil)
square_root: fn(Int) -> Result(Float, Nil)
subtract: fn(Int, Int) -> Int
sum: fn(List(Int)) -> Int
to_base16: fn(Int) -> String
to_base2: fn(Int) -> String
to_base36: fn(Int) -> String
to_base8: fn(Int) -> String
to_base_string: fn(Int, Int) -> Result(String, Nil)
to_float: fn(Int) -> Float
to_string: fn(Int) -> String
"#,
};

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn tracks_official_gleam_int_public_surface() {
    assert_surface(
        "gleam_int",
        "gleam/int",
        &["gleam/order", "gleam/float", "gleam/int"],
        &SURFACE,
    );
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_int_behavior() {
    let providers =
        host_providers::<GleamStdlibProfile>().expect("official stdlib providers should register");
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<GleamStdlibProfile>>::new(), providers)
            .expect("official stdlib provider modules should be unique");
    let value = run_hosted_fixture(
        "gleam_int",
        &["gleam/order", "gleam/float", "gleam/int"],
        hosts,
        &mut GleamStdlibRunState::from_seed([6; 32]),
    );

    assert!(matches!(value, Value::Int(_)));
}
