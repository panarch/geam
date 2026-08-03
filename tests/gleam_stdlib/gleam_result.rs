use geam::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, host_providers};
use geam::{HostModule, HostProviderSet, Value};

use super::{ExpectedSurface, assert_surface, run_hosted_fixture};

const DEPENDENCIES: &[&str] = &[
    "gleam/option",
    "gleam/dict",
    "gleam/order",
    "gleam/float",
    "gleam/int",
    "gleam/list",
    "gleam/result",
];

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "all",
        "flatten",
        "is_error",
        "is_ok",
        "lazy_or",
        "lazy_unwrap",
        "map",
        "map_error",
        "or",
        "partition",
        "replace",
        "replace_error",
        "try",
        "try_recover",
        "unwrap",
        "unwrap_error",
        "values",
    ],
    types: &[],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
all: fn(List(Result(a, e))) -> Result(List(a), e)
flatten: fn(Result(Result(a, e), e)) -> Result(a, e)
is_error: fn(Result(a, e)) -> Bool
is_ok: fn(Result(a, e)) -> Bool
lazy_or: fn(Result(a, e), fn() -> Result(a, e)) -> Result(a, e)
lazy_unwrap: fn(Result(a, e), or: fn() -> a) -> a
map: fn(over: Result(a, e), with: fn(a) -> b) -> Result(b, e)
map_error: fn(over: Result(a, e), with: fn(e) -> f) -> Result(a, f)
or: fn(Result(a, e), Result(a, e)) -> Result(a, e)
partition: fn(List(Result(a, e))) -> #(List(a), List(e))
replace: fn(Result(a, e), b) -> Result(b, e)
replace_error: fn(Result(a, e), f) -> Result(a, f)
try: fn(Result(a, e), apply: fn(a) -> Result(b, e)) -> Result(b, e)
try_recover: fn(Result(a, e), with: fn(e) -> Result(a, f)) -> Result(a, f)
unwrap: fn(Result(a, e), or: a) -> a
unwrap_error: fn(Result(a, e), or: e) -> e
values: fn(List(Result(a, e))) -> List(a)
"#,
};

fn hosts() -> HostProviderSet<GleamStdlibProfile> {
    let providers =
        host_providers::<GleamStdlibProfile>().expect("official stdlib providers should register");
    HostProviderSet::with_providers(Vec::<HostModule<GleamStdlibProfile>>::new(), providers)
        .expect("official stdlib provider modules should be unique")
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn tracks_official_gleam_result_public_surface() {
    assert_surface(
        "gleam_result_basics",
        "gleam/result",
        DEPENDENCIES,
        &SURFACE,
    );
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_result_basics() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_result_basics",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([0; 32]),
        ),
        Value::Nil,
    );
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_result_combinators() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_result_combinators",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([1; 32]),
        ),
        Value::Nil,
    );
}
