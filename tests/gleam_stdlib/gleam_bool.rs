use geam::Value;

use super::{ExpectedSurface, assert_surface, run_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "and",
        "exclusive_nor",
        "exclusive_or",
        "guard",
        "lazy_guard",
        "nand",
        "negate",
        "nor",
        "or",
        "to_string",
    ],
    types: &[],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
and: fn(Bool, Bool) -> Bool
exclusive_nor: fn(Bool, Bool) -> Bool
exclusive_or: fn(Bool, Bool) -> Bool
guard: fn(when: Bool, return: a, otherwise: fn() -> a) -> a
lazy_guard: fn(when: Bool, return: fn() -> a, otherwise: fn() -> a) -> a
nand: fn(Bool, Bool) -> Bool
negate: fn(Bool) -> Bool
nor: fn(Bool, Bool) -> Bool
or: fn(Bool, Bool) -> Bool
to_string: fn(Bool) -> String
"#,
};

#[test]
fn tracks_official_gleam_bool_public_surface() {
    assert_surface("gleam_bool", "gleam/bool", &["gleam/bool"], &SURFACE);
}

#[test]
fn runs_official_gleam_bool_behavior() {
    assert_eq!(run_fixture("gleam_bool", &["gleam/bool"]), Value::Nil);
}
