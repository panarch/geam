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
];

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "Continue",
        "Stop",
        "all",
        "any",
        "append",
        "chunk",
        "combination_pairs",
        "combinations",
        "contains",
        "count",
        "drop",
        "drop_while",
        "each",
        "filter",
        "filter_map",
        "find",
        "find_map",
        "first",
        "flat_map",
        "flatten",
        "fold",
        "fold_right",
        "fold_until",
        "group",
        "index_fold",
        "index_map",
        "interleave",
        "intersperse",
        "is_empty",
        "key_filter",
        "key_find",
        "key_pop",
        "key_set",
        "last",
        "length",
        "map",
        "map2",
        "map_fold",
        "max",
        "new",
        "partition",
        "permutations",
        "prepend",
        "reduce",
        "repeat",
        "rest",
        "reverse",
        "sample",
        "scan",
        "shuffle",
        "sized_chunk",
        "sort",
        "split",
        "split_while",
        "strict_zip",
        "take",
        "take_while",
        "transpose",
        "try_each",
        "try_fold",
        "try_map",
        "unique",
        "unzip",
        "window",
        "window_by_2",
        "wrap",
        "zip",
    ],
    types: &[("ContinueOrStop", 1)],
    type_aliases: &[],
    constructors: &[
        ("ContinueOrStop", "Continue", 1),
        ("ContinueOrStop", "Stop", 1),
    ],
    functions: r#"
all: fn(in: List(a), satisfying: fn(a) -> Bool) -> Bool
any: fn(in: List(a), satisfying: fn(a) -> Bool) -> Bool
append: fn(List(a), List(a)) -> List(a)
chunk: fn(in: List(a), by: fn(a) -> k) -> List(List(a))
combination_pairs: fn(List(a)) -> List(#(a, a))
combinations: fn(List(a), by: Int) -> List(List(a))
contains: fn(List(a), any: a) -> Bool
count: fn(List(a), where: fn(a) -> Bool) -> Int
drop: fn(from: List(a), up_to: Int) -> List(a)
drop_while: fn(in: List(a), satisfying: fn(a) -> Bool) -> List(a)
each: fn(List(a), fn(a) -> b) -> Nil
filter: fn(List(a), keeping: fn(a) -> Bool) -> List(a)
filter_map: fn(List(a), with: fn(a) -> Result(b, e)) -> List(b)
find: fn(in: List(a), one_that: fn(a) -> Bool) -> Result(a, Nil)
find_map: fn(in: List(a), with: fn(a) -> Result(b, c)) -> Result(b, Nil)
first: fn(List(a)) -> Result(a, Nil)
flat_map: fn(over: List(a), with: fn(a) -> List(b)) -> List(b)
flatten: fn(List(List(a))) -> List(a)
fold: fn(over: List(a), from: acc, with: fn(acc, a) -> acc) -> acc
fold_right: fn(over: List(a), from: acc, with: fn(acc, a) -> acc) -> acc
fold_until: fn(over: List(a), from: acc, with: fn(acc, a) -> ContinueOrStop(acc)) -> acc
group: fn(List(v), by: fn(v) -> k) -> Dict(k, List(v))
index_fold: fn(over: List(a), from: acc, with: fn(acc, a, Int) -> acc) -> acc
index_map: fn(List(a), with: fn(a, Int) -> b) -> List(b)
interleave: fn(List(List(a))) -> List(a)
intersperse: fn(List(a), with: a) -> List(a)
is_empty: fn(List(a)) -> Bool
key_filter: fn(in: List(#(k, v)), find: k) -> List(v)
key_find: fn(in: List(#(k, v)), find: k) -> Result(v, Nil)
key_pop: fn(List(#(k, v)), k) -> Result(#(v, List(#(k, v))), Nil)
key_set: fn(List(#(k, v)), k, v) -> List(#(k, v))
last: fn(List(a)) -> Result(a, Nil)
length: fn(of: List(a)) -> Int
map2: fn(List(a), List(b), with: fn(a, b) -> c) -> List(c)
map: fn(List(a), with: fn(a) -> b) -> List(b)
map_fold: fn(over: List(a), from: acc, with: fn(acc, a) -> #(acc, b)) -> #(acc, List(b))
max: fn(over: List(a), with: fn(a, a) -> Order) -> Result(a, Nil)
new: fn() -> List(a)
partition: fn(List(a), with: fn(a) -> Bool) -> #(List(a), List(a))
permutations: fn(List(a)) -> List(List(a))
prepend: fn(to: List(a), this: a) -> List(a)
reduce: fn(over: List(a), with: fn(a, a) -> a) -> Result(a, Nil)
repeat: fn(item: a, times: Int) -> List(a)
rest: fn(List(a)) -> Result(List(a), Nil)
reverse: fn(List(a)) -> List(a)
sample: fn(from: List(a), up_to: Int) -> List(a)
scan: fn(over: List(a), from: acc, with: fn(acc, a) -> acc) -> List(acc)
shuffle: fn(List(a)) -> List(a)
sized_chunk: fn(in: List(a), into: Int) -> List(List(a))
sort: fn(List(a), by: fn(a, a) -> Order) -> List(a)
split: fn(list: List(a), at: Int) -> #(List(a), List(a))
split_while: fn(list: List(a), satisfying: fn(a) -> Bool) -> #(List(a), List(a))
strict_zip: fn(List(a), with: List(b)) -> Result(List(#(a, b)), Nil)
take: fn(from: List(a), up_to: Int) -> List(a)
take_while: fn(in: List(a), satisfying: fn(a) -> Bool) -> List(a)
transpose: fn(List(List(a))) -> List(List(a))
try_each: fn(over: List(a), with: fn(a) -> Result(b, e)) -> Result(Nil, e)
try_fold: fn(over: List(a), from: acc, with: fn(acc, a) -> Result(acc, e)) -> Result(acc, e)
try_map: fn(over: List(a), with: fn(a) -> Result(b, e)) -> Result(List(b), e)
unique: fn(List(a)) -> List(a)
unzip: fn(List(#(a, b))) -> #(List(a), List(b))
window: fn(List(a), by: Int) -> List(List(a))
window_by_2: fn(List(a)) -> List(#(a, a))
wrap: fn(a) -> List(a)
zip: fn(List(a), with: List(b)) -> List(#(a, b))
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
fn tracks_official_gleam_list_public_surface() {
    assert_surface("gleam_list_basics", "gleam/list", DEPENDENCIES, &SURFACE);
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_list_basics() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_list_basics",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([0; 32]),
        ),
        Value::Nil,
    );
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_list_transforms() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_list_transforms",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([1; 32]),
        ),
        Value::Nil,
    );
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_list_folds() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_list_folds",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([2; 32]),
        ),
        Value::Nil,
    );
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_list_pairs() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_list_pairs",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([3; 32]),
        ),
        Value::Nil,
    );
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_list_shapes() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_list_shapes",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([4; 32]),
        ),
        Value::Nil,
    );
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_stdlib fixture"]
fn runs_official_gleam_list_random_collections() {
    assert_eq!(
        run_hosted_fixture(
            "gleam_list_random",
            DEPENDENCIES,
            hosts(),
            &mut GleamStdlibRunState::from_seed([5; 32]),
        ),
        Value::Nil,
    );
}
