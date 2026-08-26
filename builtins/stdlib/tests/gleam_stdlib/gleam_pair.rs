use geam_core::Value;

use super::{ExpectedSurface, assert_surface, run_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &["first", "map_first", "map_second", "new", "second", "swap"],
    types: &[],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
first: fn(#(a, b)) -> a
map_first: fn(of: #(a, b), with: fn(a) -> c) -> #(c, b)
map_second: fn(of: #(a, b), with: fn(b) -> c) -> #(a, c)
new: fn(a, b) -> #(a, b)
second: fn(#(a, b)) -> b
swap: fn(#(a, b)) -> #(b, a)
"#,
};

#[test]
fn tracks_official_gleam_pair_public_surface() {
    assert_surface("gleam_pair", "gleam/pair", &["gleam/pair"], &SURFACE);
}

#[test]
fn runs_official_gleam_pair_behavior() {
    assert_eq!(run_fixture("gleam_pair", &["gleam/pair"]), Value::Nil);
}
