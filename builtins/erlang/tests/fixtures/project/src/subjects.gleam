import gleam/erlang/process

pub fn main() {
  let ordinary = process.new_subject()
  let name = process.new_name("named")
  let assert Ok(Nil) = process.register(process.self(), name)
  let named = process.named_subject(name)
  let ignored = process.new_subject()
  process.send(ignored, "untouched")
  process.send(named, 20)
  process.send(ordinary, 21)
  let selector =
    process.new_selector()
    |> process.select_map(ordinary, fn(value) { value * 2 })
    |> process.select_map(named, fn(value) { value + 2 })
  let first = process.selector_receive_forever(selector)
  let second = process.selector_receive_forever(selector)
  let untouched = process.receive_forever(ignored)
  let empty = process.receive(ordinary, 0)
  let owner = process.subject_owner(named) == Ok(process.self())
  let same_name = process.subject_name(named) == Ok(name)
  let assert Ok(Nil) = process.unregister(name)
  #(first, second, untouched, empty, owner, same_name)
}
// @geam:expect #(22, 42, "untouched", Error(Nil), True, True)
