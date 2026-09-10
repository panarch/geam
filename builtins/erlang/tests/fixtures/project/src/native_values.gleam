import gleam/dynamic/decode
import gleam/erlang/application
import gleam/erlang/atom
import gleam/erlang/charlist
import gleam/erlang/node
import gleam/erlang/process
import gleam/erlang/reference

pub fn main() {
  let tag = atom.create("packet")
  let subject =
    process.unsafely_create_subject(process.self(), atom.to_dynamic(tag))
  process.send(subject, 42)
  let selector =
    process.new_selector()
    |> process.select_record(tag, 1, fn(message) {
      decode.run(message, decode.at([1], decode.int))
    })
    |> process.select_other(fn(_) { Error([]) })
  let result = process.selector_receive_forever(selector)
  let ref = reference.new()
  #(
    result,
    atom.get("packet") == Ok(tag),
    atom.to_string(tag),
    charlist.to_string(charlist.from_string("hello")),
    node.visible(),
    atom.to_string(node.name(node.self())),
    node.connect(node.name(node.self())),
    application.priv_directory("unknown_package"),
    ref == ref,
    ref == reference.new(),
  )
}
// @geam:expect #(Ok(42), True, "packet", "hello", [], "nonode@nohost", Error(LocalNodeIsNotAlive), Error(Nil), True, False)
