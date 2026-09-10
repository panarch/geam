import gleam/dynamic
import gleam/dynamic/decode
import gleam/erlang/atom
import gleam/erlang/charlist
import gleam/erlang/node
import gleam/erlang/process
import gleam/erlang/reference
import gleam/string

pub type Marker {
  RuntimeMarker
}

fn erase(value) {
  let subject = process.new_subject()
  process.send(subject, value)
  let assert Ok(value) =
    process.new_selector()
    |> process.select_other(fn(message) {
      decode.run(message, decode.at([1], decode.dynamic))
    })
    |> process.selector_receive_forever
  value
}

pub fn main() {
  let assert Ok(marker) = atom.get("runtime_marker")
  let assert Ok(true_atom) = atom.get("true")
  let assert Ok(false_atom) = atom.get("false")
  let assert Ok(nil_atom) = atom.get("nil")
  let name = "created_at_runtime_" <> string.inspect(reference.new())
  let assert Error(Nil) = atom.get(name)
  let created = atom.create(name)
  let assert Ok(same) = atom.get(name)
  let long_name = string.repeat("\u{e9}", 255)
  let long = atom.create(long_name)
  let number = dynamic.int(42)
  let unchecked = atom.cast_from_dynamic(number)
  let assert Error(_) = decode.run(atom.to_dynamic(unchecked), atom.decoder())
  let text = "\u{0}A\u{e9}\u{1f642}"
  let chars = charlist.from_string(text)
  let assert Ok(codepoints) = decode.run(erase(chars), decode.list(decode.int))
  let assert Ok(local_node) = decode.run(erase(node.self()), atom.decoder())
  #(
    RuntimeMarker,
    atom.to_string(marker) == "runtime_marker",
    created == same,
    atom.to_string(long) == long_name,
    atom.to_dynamic(true_atom) == dynamic.bool(True),
    atom.to_dynamic(false_atom) == dynamic.bool(False),
    atom.to_dynamic(nil_atom) == dynamic.nil(),
    atom.to_dynamic(unchecked) == number,
    charlist.to_string(chars) == text,
    codepoints == [0, 65, 233, 128_578],
    atom.to_string(local_node) == "nonode@nohost",
  )
}
// @geam:expect #(RuntimeMarker, True, True, True, True, True, True, True, True, True, True)
