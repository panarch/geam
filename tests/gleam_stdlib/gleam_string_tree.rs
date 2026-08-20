use geam::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, host_providers};
use geam::{HostModule, HostProviderSet};

use super::{ExpectedSurface, assert_surface, run_hosted_fixture};

const DEPENDENCIES: &[&str] = &[
    "gleam/option",
    "gleam/dict",
    "gleam/order",
    "gleam/float",
    "gleam/int",
    "gleam/list",
    "gleam/string_tree",
];

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "append",
        "append_tree",
        "byte_size",
        "concat",
        "from_string",
        "from_strings",
        "is_empty",
        "is_equal",
        "join",
        "lowercase",
        "new",
        "prepend",
        "prepend_tree",
        "replace",
        "reverse",
        "split",
        "to_string",
        "uppercase",
    ],
    types: &[("StringTree", 0)],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
append: fn(to: StringTree, suffix: String) -> StringTree
append_tree: fn(to: StringTree, suffix: StringTree) -> StringTree
byte_size: fn(StringTree) -> Int
concat: fn(List(StringTree)) -> StringTree
from_string: fn(String) -> StringTree
from_strings: fn(List(String)) -> StringTree
is_empty: fn(StringTree) -> Bool
is_equal: fn(StringTree, StringTree) -> Bool
join: fn(List(StringTree), with: String) -> StringTree
lowercase: fn(StringTree) -> StringTree
new: fn() -> StringTree
prepend: fn(to: StringTree, prefix: String) -> StringTree
prepend_tree: fn(to: StringTree, prefix: StringTree) -> StringTree
replace: fn(in: StringTree, each: String, with: String) -> StringTree
reverse: fn(StringTree) -> StringTree
split: fn(StringTree, on: String) -> List(StringTree)
to_string: fn(StringTree) -> String
uppercase: fn(StringTree) -> StringTree
"#,
};

fn hosts() -> HostProviderSet<GleamStdlibProfile> {
    let providers =
        host_providers::<GleamStdlibProfile>().expect("official stdlib providers should register");
    HostProviderSet::with_providers(Vec::<HostModule<GleamStdlibProfile>>::new(), providers)
        .expect("official stdlib provider modules should be unique")
}

#[test]
fn tracks_official_gleam_string_tree_public_surface() {
    assert_surface(
        "gleam_string_tree",
        "gleam/string_tree",
        DEPENDENCIES,
        &SURFACE,
    );
}

#[test]
fn runs_official_gleam_string_tree() {
    let value = run_hosted_fixture(
        "gleam_string_tree",
        DEPENDENCIES,
        hosts(),
        &mut GleamStdlibRunState::from_seed([0; 32]),
    );

    assert_eq!(
        value.inspect().to_string(),
        r#"string_tree.from_string("zero-one-two-three")"#,
    );
}
