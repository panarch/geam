use geam::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, host_providers};
use geam::{HostModule, HostProviderSet, Value};

use super::{ExpectedSurface, assert_surface, run_hosted_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "array",
        "bit_array",
        "bool",
        "classify",
        "float",
        "int",
        "list",
        "nil",
        "properties",
        "string",
    ],
    types: &[("Dynamic", 0)],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
array: fn(List(Dynamic)) -> Dynamic
bit_array: fn(BitArray) -> Dynamic
bool: fn(Bool) -> Dynamic
classify: fn(Dynamic) -> String
float: fn(Float) -> Dynamic
int: fn(Int) -> Dynamic
list: fn(List(Dynamic)) -> Dynamic
nil: fn() -> Dynamic
properties: fn(List(#(Dynamic, Dynamic))) -> Dynamic
string: fn(String) -> Dynamic
"#,
};

#[test]
fn tracks_official_gleam_dynamic_public_surface() {
    assert_surface(
        "gleam_dynamic",
        "gleam/dynamic",
        &["gleam/option", "gleam/dict", "gleam/dynamic"],
        &SURFACE,
    );
}

#[test]
fn runs_official_gleam_dynamic_behavior() {
    let providers =
        host_providers::<GleamStdlibProfile>().expect("official stdlib providers should register");
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<GleamStdlibProfile>>::new(), providers)
            .expect("official stdlib provider modules should be unique");
    let value = run_hosted_fixture(
        "gleam_dynamic",
        &["gleam/option", "gleam/dict", "gleam/dynamic"],
        hosts,
        &mut GleamStdlibRunState::from_seed([0; 32]),
    );

    assert!(matches!(value, Value::Tuple(_)));
}
