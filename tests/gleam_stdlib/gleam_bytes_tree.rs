use geam::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, host_providers};
use geam::{HostModule, HostProviderSet, Value};

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
    "gleam/bytes_tree",
];

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "append",
        "append_string",
        "append_tree",
        "byte_size",
        "concat",
        "concat_bit_arrays",
        "from_bit_array",
        "from_string",
        "from_string_tree",
        "new",
        "prepend",
        "prepend_string",
        "prepend_tree",
        "to_bit_array",
    ],
    types: &[("BytesTree", 0)],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
append: fn(to: BytesTree, suffix: BitArray) -> BytesTree
append_string: fn(to: BytesTree, suffix: String) -> BytesTree
append_tree: fn(to: BytesTree, suffix: BytesTree) -> BytesTree
byte_size: fn(BytesTree) -> Int
concat: fn(List(BytesTree)) -> BytesTree
concat_bit_arrays: fn(List(BitArray)) -> BytesTree
from_bit_array: fn(BitArray) -> BytesTree
from_string: fn(String) -> BytesTree
from_string_tree: fn(StringTree) -> BytesTree
new: fn() -> BytesTree
prepend: fn(to: BytesTree, prefix: BitArray) -> BytesTree
prepend_string: fn(to: BytesTree, prefix: String) -> BytesTree
prepend_tree: fn(to: BytesTree, prefix: BytesTree) -> BytesTree
to_bit_array: fn(BytesTree) -> BitArray
"#,
};

fn hosts() -> HostProviderSet<GleamStdlibProfile> {
    let providers =
        host_providers::<GleamStdlibProfile>().expect("official stdlib providers should register");
    HostProviderSet::with_providers(Vec::<HostModule<GleamStdlibProfile>>::new(), providers)
        .expect("official stdlib provider modules should be unique")
}

#[test]
fn tracks_official_gleam_bytes_tree_public_surface() {
    assert_surface(
        "gleam_bytes_tree_basics",
        "gleam/bytes_tree",
        DEPENDENCIES,
        &SURFACE,
    );
}

#[test]
fn runs_official_gleam_bytes_tree_basics() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_bytes_tree_basics",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([0; 32]),
        ),
        Value::Nil,
    );
}

#[test]
fn runs_official_gleam_bytes_tree_structure() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_bytes_tree_structure",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([1; 32]),
        ),
        Value::Nil,
    );
}
