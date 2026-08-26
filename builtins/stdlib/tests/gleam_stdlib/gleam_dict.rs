use geam_core::{HostModule, HostProviderSet, Value};
use geam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, host_providers};

use super::{ExpectedSurface, assert_surface, run_hosted_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "combine",
        "delete",
        "drop",
        "each",
        "filter",
        "fold",
        "from_list",
        "get",
        "has_key",
        "insert",
        "is_empty",
        "keys",
        "map_values",
        "merge",
        "new",
        "size",
        "take",
        "to_list",
        "upsert",
        "values",
    ],
    types: &[("Dict", 2)],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
combine: fn(Dict(k, v), Dict(k, v), with: fn(v, v) -> v) -> Dict(k, v)
delete: fn(from: Dict(k, v), delete: k) -> Dict(k, v)
drop: fn(from: Dict(k, v), drop: List(k)) -> Dict(k, v)
each: fn(Dict(k, v), fn(k, v) -> a) -> Nil
filter: fn(in: Dict(k, v), keeping: fn(k, v) -> Bool) -> Dict(k, v)
fold: fn(over: Dict(k, v), from: acc, with: fn(acc, k, v) -> acc) -> acc
from_list: fn(List(#(k, v))) -> Dict(k, v)
get: fn(Dict(k, v), k) -> Result(v, Nil)
has_key: fn(Dict(k, v), k) -> Bool
insert: fn(into: Dict(k, v), for: k, insert: v) -> Dict(k, v)
is_empty: fn(Dict(k, v)) -> Bool
keys: fn(Dict(k, v)) -> List(k)
map_values: fn(in: Dict(k, v), with: fn(k, v) -> a) -> Dict(k, a)
merge: fn(into: Dict(k, v), from: Dict(k, v)) -> Dict(k, v)
new: fn() -> Dict(k, v)
size: fn(Dict(k, v)) -> Int
take: fn(from: Dict(k, v), keeping: List(k)) -> Dict(k, v)
to_list: fn(Dict(k, v)) -> List(#(k, v))
upsert: fn(in: Dict(k, v), update: k, with: fn(Option(v)) -> v) -> Dict(k, v)
values: fn(Dict(k, v)) -> List(v)
"#,
};

#[test]
fn tracks_official_gleam_dict_public_surface() {
    assert_surface(
        "gleam_dict",
        "gleam/dict",
        &["gleam/option", "gleam/dict"],
        &SURFACE,
    );
}

#[test]
fn runs_official_gleam_dict_behavior() {
    let providers =
        host_providers::<GleamStdlibProfile>().expect("official stdlib providers should register");
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<GleamStdlibProfile>>::new(), providers)
            .expect("official stdlib provider modules should be unique");
    let value = run_hosted_fixture(
        "gleam_dict",
        &["gleam/option", "gleam/dict"],
        hosts,
        &mut GleamStdlibRunState::from_seed([0; 32]),
    );

    assert!(matches!(value, Value::External(_)));
    assert_eq!(
        value.inspect().to_string(),
        r#"dict.from_list([#("a", 3), #("b", 2)])"#,
    );
}
