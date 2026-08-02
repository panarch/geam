use geam::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, host_providers};
use geam::{HostModule, HostProviderSet};

use super::{ExpectedSurface, assert_surface, run_hosted_fixture};

const DEPENDENCIES: &[&str] = &[
    "gleam/order",
    "gleam/float",
    "gleam/int",
    "gleam/option",
    "gleam/dict",
    "gleam/list",
    "gleam/string_tree",
    "gleam/string",
    "gleam/bit_array",
];

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "append",
        "base16_decode",
        "base16_encode",
        "base64_decode",
        "base64_encode",
        "base64_url_decode",
        "base64_url_encode",
        "bit_size",
        "byte_size",
        "compare",
        "concat",
        "from_string",
        "inspect",
        "is_utf8",
        "pad_to_bytes",
        "slice",
        "starts_with",
        "to_string",
    ],
    types: &[],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
append: fn(to: BitArray, suffix: BitArray) -> BitArray
base16_decode: fn(String) -> Result(BitArray, Nil)
base16_encode: fn(BitArray) -> String
base64_decode: fn(String) -> Result(BitArray, Nil)
base64_encode: fn(BitArray, Bool) -> String
base64_url_decode: fn(String) -> Result(BitArray, Nil)
base64_url_encode: fn(BitArray, Bool) -> String
bit_size: fn(BitArray) -> Int
byte_size: fn(BitArray) -> Int
compare: fn(BitArray, with: BitArray) -> order.Order
concat: fn(List(BitArray)) -> BitArray
from_string: fn(String) -> BitArray
inspect: fn(BitArray) -> String
is_utf8: fn(BitArray) -> Bool
pad_to_bytes: fn(BitArray) -> BitArray
slice: fn(from: BitArray, at: Int, take: Int) -> Result(BitArray, Nil)
starts_with: fn(BitArray, BitArray) -> Bool
to_string: fn(BitArray) -> Result(String, Nil)
"#,
};

fn hosts() -> HostProviderSet<GleamStdlibProfile> {
    let providers =
        host_providers::<GleamStdlibProfile>().expect("official stdlib providers should register");
    HostProviderSet::with_providers(Vec::<HostModule<GleamStdlibProfile>>::new(), providers)
        .expect("official stdlib provider modules should be unique")
}

fn run(root_module: &str) {
    run_hosted_fixture(
        root_module,
        DEPENDENCIES,
        hosts(),
        &mut GleamStdlibRunState::from_seed([0; 32]),
    );
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn tracks_official_gleam_bit_array_public_surface() {
    assert_surface(
        "gleam_bit_array_basics",
        "gleam/bit_array",
        DEPENDENCIES,
        &SURFACE,
    );
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_bit_array_basics() {
    run("gleam_bit_array_basics");
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_bit_array_codecs() {
    run("gleam_bit_array_codecs");
}
