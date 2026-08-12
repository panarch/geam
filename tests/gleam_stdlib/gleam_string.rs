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
    "gleam/string",
];

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "append",
        "byte_size",
        "capitalise",
        "compare",
        "concat",
        "contains",
        "crop",
        "drop_end",
        "drop_start",
        "ends_with",
        "first",
        "from_utf_codepoints",
        "inspect",
        "is_empty",
        "join",
        "last",
        "length",
        "lowercase",
        "pad_end",
        "pad_start",
        "pop_grapheme",
        "remove_prefix",
        "remove_suffix",
        "repeat",
        "replace",
        "reverse",
        "slice",
        "split",
        "split_once",
        "starts_with",
        "to_graphemes",
        "to_option",
        "to_utf_codepoints",
        "trim",
        "trim_end",
        "trim_start",
        "uppercase",
        "utf_codepoint",
        "utf_codepoint_to_int",
    ],
    types: &[],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
append: fn(to: String, suffix: String) -> String
byte_size: fn(String) -> Int
capitalise: fn(String) -> String
compare: fn(String, String) -> order.Order
concat: fn(List(String)) -> String
contains: fn(does: String, contain: String) -> Bool
crop: fn(from: String, before: String) -> String
drop_end: fn(from: String, up_to: Int) -> String
drop_start: fn(from: String, up_to: Int) -> String
ends_with: fn(String, String) -> Bool
first: fn(String) -> Result(String, Nil)
from_utf_codepoints: fn(List(UtfCodepoint)) -> String
inspect: fn(anything) -> String
is_empty: fn(String) -> Bool
join: fn(List(String), with: String) -> String
last: fn(String) -> Result(String, Nil)
length: fn(String) -> Int
lowercase: fn(String) -> String
pad_end: fn(String, to: Int, with: String) -> String
pad_start: fn(String, to: Int, with: String) -> String
pop_grapheme: fn(String) -> Result(#(String, String), Nil)
remove_prefix: fn(from: String, matching: String) -> String
remove_suffix: fn(from: String, matching: String) -> String
repeat: fn(String, times: Int) -> String
replace: fn(in: String, each: String, with: String) -> String
reverse: fn(String) -> String
slice: fn(from: String, at_index: Int, length: Int) -> String
split: fn(String, on: String) -> List(String)
split_once: fn(String, on: String) -> Result(#(String, String), Nil)
starts_with: fn(String, String) -> Bool
to_graphemes: fn(String) -> List(String)
to_option: fn(String) -> Option(String)
to_utf_codepoints: fn(String) -> List(UtfCodepoint)
trim: fn(String) -> String
trim_end: fn(String) -> String
trim_start: fn(String) -> String
uppercase: fn(String) -> String
utf_codepoint: fn(Int) -> Result(UtfCodepoint, Nil)
utf_codepoint_to_int: fn(UtfCodepoint) -> Int
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
fn tracks_official_gleam_string_public_surface() {
    assert_surface(
        "gleam_string_basics",
        "gleam/string",
        DEPENDENCIES,
        &SURFACE,
    );
}

#[test]
fn runs_official_gleam_string_basics() {
    run("gleam_string_basics");
}

#[test]
fn runs_official_gleam_string_slicing_and_trimming() {
    run("gleam_string_slice");
}

#[test]
fn runs_official_gleam_string_unicode() {
    run("gleam_string_unicode");
}

#[test]
fn runs_official_gleam_string_inspection() {
    run("gleam_string_inspect");
}
