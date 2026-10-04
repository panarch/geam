import gleam/erlang/process

pub fn main() {
  // Each subject deliberately has no materialized argument value.
  let subject = process.new_subject()
  let selector = process.new_selector() |> process.select(subject)
  let mapping = fn(_) { Nil }
  let mapped = selector |> process.map_selector(mapping)
  let _empty = process.new_selector() |> process.map_selector(fn(_) { Nil })
  let _direct =
    process.new_selector() |> process.select_map(subject, fn(_) { Nil })
  let alias = selector
  assert selector == alias
  assert mapped == process.map_selector(alias, mapping)

  // List(item) is inhabited by [], even while item remains symbolic.
  let list_subject = process.new_subject()
  process.send(list_subject, [])
  let lists = process.new_selector() |> process.select(list_subject)
  let assert Ok(Nil) =
    process.selector_receive(lists |> process.map_selector(fn(_) { Nil }), 0)

  let concrete = process.new_subject()
  process.send(concrete, 7)
  let concrete_selector =
    process.new_selector()
    |> process.select(concrete)
    |> process.map_selector(fn(value) { value + 2 })
    |> process.map_selector(fn(value) { value * 3 })
  let assert Ok(27) = process.selector_receive(concrete_selector, 0)
  Nil
}
// @geam:expect Nil
