use geam_core::{HostModule, HostProviderSet};
use geam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, host_providers};

use super::{ExpectedSurface, assert_surface, run_hosted_fixture};

const DEPENDENCIES: &[&str] = &[
    "gleam/option",
    "gleam/dict",
    "gleam/order",
    "gleam/float",
    "gleam/int",
    "gleam/list",
    "gleam/string_tree",
    "gleam/string",
    "gleam/uri",
];

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "Uri",
        "empty",
        "merge",
        "origin",
        "parse",
        "parse_query",
        "path_segments",
        "percent_decode",
        "percent_encode",
        "query_to_string",
        "to_string",
    ],
    types: &[("Uri", 0)],
    type_aliases: &[],
    constructors: &[("Uri", "Uri", 7)],
    functions: r#"
merge: fn(Uri, Uri) -> Result(Uri, Nil)
origin: fn(Uri) -> Result(String, Nil)
parse: fn(String) -> Result(Uri, Nil)
parse_query: fn(String) -> Result(List(#(String, String)), Nil)
path_segments: fn(String) -> List(String)
percent_decode: fn(String) -> Result(String, Nil)
percent_encode: fn(String) -> String
query_to_string: fn(List(#(String, String))) -> String
to_string: fn(Uri) -> String
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
fn tracks_official_gleam_uri_public_surface() {
    assert_surface("gleam_uri_parse", "gleam/uri", DEPENDENCIES, &SURFACE);
}

#[test]
fn runs_the_unchanged_official_gleam_uri_parser() {
    run("gleam_uri_parse");
}

#[test]
fn runs_official_gleam_uri_query_and_percent_codecs() {
    run("gleam_uri_query");
}

#[test]
fn runs_official_gleam_uri_operations() {
    run("gleam_uri_operations");
}
