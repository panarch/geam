import gleam/dict
import gleam/dynamic
import gleam/dynamic/decode
import gleam/erlang/atom
import gleam/erlang/process
import gleam/list

fn erase(value) {
  let carrier = process.new_subject()
  process.send(carrier, value)
  process.new_selector()
  |> process.select_other(fn(message) {
    let assert Ok(value) = decode.run(message, decode.at([1], decode.dynamic))
    value
  })
  |> process.selector_receive_forever
}

fn double(value) {
  value * 2
}

pub fn main() {
  let subject = process.new_subject()
  let other = process.new_subject()
  let base = process.new_selector() |> process.select(subject)
  let mapped = process.map_selector(base, double)
  let equivalent = process.map_selector(base, double)
  let erased = erase(mapped)
  let assert Ok(tag) = decode.run(erased, decode.at([0], atom.decoder()))
  let assert Ok(handlers) =
    decode.run(
      erased,
      decode.at([1], decode.dict(decode.dynamic, decode.dynamic)),
    )
  let selected =
    process.new_selector()
    |> process.select_map(subject, fn(value) { value - 1 })
    |> process.select_map(subject, fn(value) { value + 1 })
    |> process.merge_selector(process.new_selector() |> process.select(other))
    |> process.merge_selector(mapped)
    |> process.deselect(other)
  process.send(other, -1)
  process.send(subject, 21)
  let result = process.selector_receive_forever(selected)
  let unselected = process.receive_forever(other)
  let empty = process.deselect(selected, subject)
  #(
    result,
    unselected,
    mapped == equivalent,
    erased == erase(equivalent),
    dict.get(dict.from_list([#(mapped, 42)]), equivalent),
    atom.to_string(tag),
    list.map(dict.values(handlers), dynamic.classify),
    empty == process.new_selector(),
  )
}
// @geam:expect #(42, -1, True, True, Ok(42), "selector", ["Function"], True)
