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
    "gleam/dynamic",
    "gleam/dynamic/decode",
];

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "DecodeError",
        "at",
        "bit_array",
        "bool",
        "collapse_errors",
        "decode_error",
        "dict",
        "dynamic",
        "failure",
        "field",
        "float",
        "int",
        "list",
        "map",
        "map_errors",
        "new_primitive_decoder",
        "one_of",
        "optional",
        "optional_field",
        "optionally_at",
        "recursive",
        "run",
        "string",
        "subfield",
        "success",
        "then",
    ],
    types: &[("DecodeError", 0), ("Decoder", 1)],
    type_aliases: &["Dynamic"],
    constructors: &[("DecodeError", "DecodeError", 3)],
    functions: r#"
at: fn(List(segment), Decoder(a)) -> Decoder(a)
collapse_errors: fn(Decoder(a), String) -> Decoder(a)
decode_error: fn(expected: String, found: Dynamic) -> List(DecodeError)
dict: fn(Decoder(key), Decoder(value)) -> Decoder(Dict(key, value))
failure: fn(a, expected: String) -> Decoder(a)
field: fn(name, Decoder(t), fn(t) -> Decoder(final)) -> Decoder(final)
list: fn(of: Decoder(a)) -> Decoder(List(a))
map: fn(Decoder(a), fn(a) -> b) -> Decoder(b)
map_errors: fn(Decoder(a), fn(List(DecodeError)) -> List(DecodeError)) -> Decoder(a)
new_primitive_decoder: fn(String, fn(Dynamic) -> Result(t, t)) -> Decoder(t)
one_of: fn(Decoder(a), or: List(Decoder(a))) -> Decoder(a)
optional: fn(Decoder(a)) -> Decoder(Option(a))
optional_field: fn(name, t, Decoder(t), fn(t) -> Decoder(final)) -> Decoder(final)
optionally_at: fn(List(segment), a, Decoder(a)) -> Decoder(a)
recursive: fn(fn() -> Decoder(a)) -> Decoder(a)
run: fn(Dynamic, Decoder(t)) -> Result(t, List(DecodeError))
subfield: fn(List(name), Decoder(t), fn(t) -> Decoder(final)) -> Decoder(final)
success: fn(t) -> Decoder(t)
then: fn(Decoder(a), fn(a) -> Decoder(b)) -> Decoder(b)
"#,
};

fn hosts() -> HostProviderSet<GleamStdlibProfile> {
    let providers =
        host_providers::<GleamStdlibProfile>().expect("official stdlib providers should register");
    HostProviderSet::with_providers(Vec::<HostModule<GleamStdlibProfile>>::new(), providers)
        .expect("official stdlib provider modules should be unique")
}

#[test]
fn tracks_official_gleam_dynamic_decode_public_surface() {
    assert_surface(
        "gleam_dynamic_decode_primitives",
        "gleam/dynamic/decode",
        DEPENDENCIES,
        &SURFACE,
    );
}

#[test]
fn runs_official_gleam_dynamic_decode_primitives() {
    run_hosted_fixture(
        "gleam_dynamic_decode_primitives",
        DEPENDENCIES,
        hosts(),
        &mut GleamStdlibRunState::from_seed([0; 32]),
    );
}

#[test]
fn runs_official_gleam_dynamic_decode_collections() {
    run_hosted_fixture(
        "gleam_dynamic_decode_collections",
        DEPENDENCIES,
        hosts(),
        &mut GleamStdlibRunState::from_seed([0; 32]),
    );
}

#[test]
fn runs_official_gleam_dynamic_decode_paths() {
    run_hosted_fixture(
        "gleam_dynamic_decode_paths",
        DEPENDENCIES,
        hosts(),
        &mut GleamStdlibRunState::from_seed([0; 32]),
    );
}

#[test]
fn runs_official_gleam_dynamic_decode_combinators() {
    run_hosted_fixture(
        "gleam_dynamic_decode_combinators",
        DEPENDENCIES,
        hosts(),
        &mut GleamStdlibRunState::from_seed([0; 32]),
    );
}

#[test]
fn runs_official_gleam_dynamic_decode_recursive_values() {
    run_hosted_fixture(
        "gleam_dynamic_decode_recursive",
        DEPENDENCIES,
        hosts(),
        &mut GleamStdlibRunState::from_seed([0; 32]),
    );
}
