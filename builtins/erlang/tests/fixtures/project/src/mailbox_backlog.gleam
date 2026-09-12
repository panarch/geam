import gleam/erlang/process
import gleam/list

pub fn main() {
  let ignored = process.new_subject()
  let selected = process.new_subject()
  let missing: process.Subject(Int) = process.new_subject()
  let values = list.repeat(0, 257) |> list.index_map(fn(_, index) { index + 1 })
  list.each(values, fn(value) { process.send(ignored, value) })
  process.send(selected, 42)
  let assert Ok(42) = process.receive(selected, 0)
  let assert Error(Nil) = process.receive(missing, 0)
  let selector = process.new_selector() |> process.select(selected)
  process.send(selected, 7)
  let assert Ok(7) = process.selector_receive(selector, 0)
  let assert Error(Nil) = process.selector_receive(selector, 0)
  let received = list.map(values, fn(_) { process.receive_forever(ignored) })

  // A later match resumes after all old unmatched messages, without consuming them.
  list.each(values, fn(value) { process.send(ignored, value) })
  process.send_after(selected, 10, 99)
  let later = process.receive_forever(selected)
  let preserved = process.receive_forever(ignored)
  process.flush_messages()
  #(received == values, later, preserved, process.receive(ignored, 0))
}
// @geam:expect #(True, 99, 1, Error(Nil))
