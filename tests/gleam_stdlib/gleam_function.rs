use geam::Value;

use super::{ExpectedSurface, assert_surface, run_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &["identity"],
    types: &[],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
identity: fn(a) -> a
"#,
};

#[test]
fn tracks_official_gleam_function_public_surface() {
    assert_surface(
        "gleam_function",
        "gleam/function",
        &["gleam/function"],
        &SURFACE,
    );
}

#[test]
fn runs_official_gleam_function_behavior() {
    assert_eq!(
        run_fixture("gleam_function", &["gleam/function"]),
        Value::Nil,
    );
}
