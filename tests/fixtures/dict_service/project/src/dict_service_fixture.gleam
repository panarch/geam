import gleam/dict.{type Dict}

@external(erlang, "dict_service_fixture", "entries")
fn entries(empty: Bool) -> Dict(String, String)

@external(erlang, "dict_service_fixture", "groups")
fn groups() -> Dict(Int, List(String))

@external(erlang, "dict_service_fixture", "nested")
fn nested() -> #(Dict(String, String), List(Dict(String, String)))

pub fn main() {
  let assert 0 = dict.size(entries(True))
  let assert Error(Nil) = dict.get(entries(True), "LANG")
  let values = entries(False)
  let assert 2 = dict.size(values)
  let assert Ok("한국어\u{0}🙂") = dict.get(values, "LANG")
  let assert Ok("") = dict.get(values, "EMPTY")
  let assert Error(Nil) = dict.get(values, "missing")
  let expected = dict.from_list([#("EMPTY", ""), #("LANG", "한국어\u{0}🙂")])
  assert values == expected
  assert expected == values
  let assert Ok(42) = dict.get(dict.from_list([#(values, 42)]), expected)
  let changed = dict.insert(values, "LANG", "changed")
  let removed = dict.delete(changed, "EMPTY")
  let assert Ok("한국어\u{0}🙂") = dict.get(values, "LANG")
  let assert Ok("") = dict.get(values, "EMPTY")
  let assert Ok("changed") = dict.get(removed, "LANG")
  let assert Error(Nil) = dict.get(removed, "EMPTY")
  let assert 1 = dict.size(removed)

  let groups = groups()
  let assert 2 = dict.size(groups)
  let assert Ok(["one", "하나"]) = dict.get(groups, 1)
  let assert Ok([]) = dict.get(groups, 2)
  let assert Error(Nil) = dict.get(groups, 3)
  assert groups == dict.from_list([#(2, []), #(1, ["one", "하나"])])

  let nested = nested()
  let #(outer, children) = nested
  let assert 1 = dict.size(outer)
  let assert Ok("ready") = dict.get(outer, "outer")
  let assert [inner, empty] = children
  let assert 1 = dict.size(inner)
  let assert Ok("nested") = dict.get(inner, "inner")
  let assert 0 = dict.size(empty)
  let assert Error(Nil) = dict.get(empty, "inner")
  #(values |> dict.delete("EMPTY"), groups, nested)
}
