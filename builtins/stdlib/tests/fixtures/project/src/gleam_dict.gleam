import gleam/dict
import gleam/option.{None, Some}

fn contains(values: List(a), target: a) -> Bool {
  case values {
    [] -> False
    [value, ..rest] -> value == target || contains(rest, target)
  }
}

pub fn main() {
  let empty = dict.new()
  assert dict.is_empty(empty)
  assert dict.size(empty) == 0

  let base = dict.from_list([#("a", 1), #("b", 2), #("a", 3)])
  assert dict.size(base) == 2
  assert dict.has_key(base, "a")
  assert !dict.has_key(base, "missing")
  assert dict.get(base, "a") == Ok(3)
  assert dict.get(base, "missing") == Error(Nil)
  assert dict.from_list(dict.to_list(base)) == base
  assert contains(dict.keys(base), "a")
  assert contains(dict.keys(base), "b")
  assert contains(dict.values(base), 2)
  assert contains(dict.values(base), 3)

  let inserted = dict.insert(base, "c", 4)
  assert dict.get(inserted, "c") == Ok(4)
  let mapped =
    dict.map_values(base, fn(key, value) {
      case key {
        "a" -> value + 10
        _ -> value + 20
      }
    })
  assert dict.get(mapped, "a") == Ok(13)
  assert dict.get(mapped, "b") == Ok(22)
  assert dict.filter(base, fn(_, value) { value > 2 })
    == dict.from_list([#("a", 3)])
  assert dict.take(base, ["b", "missing"]) == dict.from_list([#("b", 2)])
  assert dict.merge(base, dict.from_list([#("b", 20), #("c", 30)]))
    == dict.from_list([#("a", 3), #("b", 20), #("c", 30)])
  assert dict.delete(base, "a") == dict.from_list([#("b", 2)])
  assert dict.delete(base, "missing") == base
  assert dict.drop(base, ["a", "missing"]) == dict.from_list([#("b", 2)])
  assert dict.upsert(base, "a", fn(value) {
      case value {
        Some(value) -> value + 1
        None -> 0
      }
    })
    == dict.from_list([#("a", 4), #("b", 2)])
  assert dict.upsert(base, "c", fn(value) {
      case value {
        Some(value) -> value + 1
        None -> 5
      }
    })
    == dict.from_list([#("a", 3), #("b", 2), #("c", 5)])
  assert dict.fold(base, 0, fn(total, _, value) { total + value }) == 5
  assert dict.each(base, fn(_, _) { Nil }) == Nil
  assert dict.combine(
      base,
      dict.from_list([#("a", 7), #("c", 9)]),
      fn(left, right) { left + right },
    )
    == dict.from_list([#("a", 10), #("b", 2), #("c", 9)])

  base
}
// @geam:expect dict.from_list([#("a", 3), #("b", 2)])
