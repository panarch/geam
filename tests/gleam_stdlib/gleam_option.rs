use geam::{HostModule, HostProviderSet, StatelessHostProfile, Value};

use super::{ExpectedSurface, assert_surface, run_hosted_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "None",
        "Some",
        "all",
        "flatten",
        "from_result",
        "is_none",
        "is_some",
        "lazy_or",
        "lazy_unwrap",
        "map",
        "or",
        "then",
        "to_result",
        "unwrap",
        "values",
    ],
    types: &[("Option", 1)],
    type_aliases: &[],
    constructors: &[("Option", "None", 0), ("Option", "Some", 1)],
    functions: r#"
all: fn(List(Option(a))) -> Option(List(a))
flatten: fn(Option(Option(a))) -> Option(a)
from_result: fn(Result(a, e)) -> Option(a)
is_none: fn(Option(a)) -> Bool
is_some: fn(Option(a)) -> Bool
lazy_or: fn(Option(a), fn() -> Option(a)) -> Option(a)
lazy_unwrap: fn(Option(a), or: fn() -> a) -> a
map: fn(over: Option(a), with: fn(a) -> b) -> Option(b)
or: fn(Option(a), Option(a)) -> Option(a)
then: fn(Option(a), apply: fn(a) -> Option(b)) -> Option(b)
to_result: fn(Option(a), e) -> Result(a, e)
unwrap: fn(Option(a), or: a) -> a
values: fn(List(Option(a))) -> List(a)
"#,
};

#[test]
fn tracks_official_gleam_option_public_surface() {
    assert_surface("gleam_option", "gleam/option", &["gleam/option"], &SURFACE);
}

#[test]
fn runs_official_gleam_option_behavior() {
    let hosts = HostProviderSet::<StatelessHostProfile>::new(Vec::<HostModule>::new())
        .expect("the empty host set should be valid");
    assert_eq!(
        run_hosted_fixture("gleam_option", &["gleam/option"], hosts, &mut ()),
        Value::Nil,
    );
}
