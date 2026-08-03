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
    "gleam/set",
];

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "contains",
        "delete",
        "difference",
        "drop",
        "each",
        "filter",
        "fold",
        "from_list",
        "insert",
        "intersection",
        "is_disjoint",
        "is_empty",
        "is_subset",
        "map",
        "new",
        "size",
        "symmetric_difference",
        "take",
        "to_list",
        "union",
    ],
    types: &[("Set", 1)],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
contains: fn(in: Set(member), this: member) -> Bool
delete: fn(from: Set(member), this: member) -> Set(member)
difference: fn(from: Set(member), minus: Set(member)) -> Set(member)
drop: fn(from: Set(member), drop: List(member)) -> Set(member)
each: fn(Set(member), fn(member) -> a) -> Nil
filter: fn(in: Set(member), keeping: fn(member) -> Bool) -> Set(member)
fold: fn(over: Set(member), from: acc, with: fn(acc, member) -> acc) -> acc
from_list: fn(List(member)) -> Set(member)
insert: fn(into: Set(member), this: member) -> Set(member)
intersection: fn(of: Set(member), and: Set(member)) -> Set(member)
is_disjoint: fn(Set(member), from: Set(member)) -> Bool
is_empty: fn(Set(member)) -> Bool
is_subset: fn(Set(member), of: Set(member)) -> Bool
map: fn(Set(member), with: fn(member) -> mapped) -> Set(mapped)
new: fn() -> Set(member)
size: fn(Set(member)) -> Int
symmetric_difference: fn(of: Set(member), and: Set(member)) -> Set(member)
take: fn(from: Set(member), keeping: List(member)) -> Set(member)
to_list: fn(Set(member)) -> List(member)
union: fn(of: Set(member), and: Set(member)) -> Set(member)
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
fn tracks_official_gleam_set_public_surface() {
    assert_surface("gleam_set_basics", "gleam/set", DEPENDENCIES, &SURFACE);
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_set_basics() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_set_basics",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([0; 32]),
        ),
        Value::Nil,
    );
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_set_operations() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_set_operations",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([1; 32]),
        ),
        Value::Nil,
    );
}
