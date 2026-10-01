import gleam/bytes_tree.{type BytesTree}
import gleam/list
import gleam/string_tree

@external(erlang, "bytes_tree_service_native", "read")
fn read(tree: BytesTree) -> BitArray

@external(erlang, "bytes_tree_service_native", "retain")
fn retain(tree: BytesTree) -> Nil

@external(erlang, "bytes_tree_service_native", "read_after_pause")
fn read_after_pause(tree: BytesTree) -> #(BitArray, BitArray)

fn mixed() -> BytesTree {
  let text =
    string_tree.concat([
      string_tree.from_string("한"),
      string_tree.new(),
      string_tree.from_string("\u{0}e\u{301}🙂"),
    ])
  bytes_tree.concat([
    bytes_tree.from_bit_array(<<0, 255, 128>>),
    bytes_tree.concat([bytes_tree.new(), bytes_tree.from_string_tree(text)]),
    bytes_tree.from_bit_array(<<1, 2>>),
  ])
}

fn nest(tree: BytesTree, depth: Int) -> BytesTree {
  case depth {
    0 -> tree
    _ -> nest(bytes_tree.concat([bytes_tree.new(), tree]), depth - 1)
  }
}

pub fn verify() -> BitArray {
  let empty = bytes_tree.new()
  let assert <<>> = read(empty)
  let assert <<>> = read(bytes_tree.from_string(""))
  let assert <<>> = read(bytes_tree.from_bit_array(<<>>))
  let assert <<255, 0, 128>> = read(bytes_tree.from_bit_array(<<255, 0, 128>>))
  let assert <<0, 237, 149, 156>> = read(bytes_tree.from_string("\u{0}한"))
  let assert <<160>> = read(bytes_tree.from_bit_array(<<5:size(3)>>))
  let tree = mixed()
  let alias = tree
  let expected = <<
    0, 255, 128, 237, 149, 156, 0, 101, 204, 129, 240, 159, 153, 130, 1, 2,
  >>
  let assert True = read(tree) == expected
  let assert True = read(tree) == bytes_tree.to_bit_array(tree)
  let assert True = read(alias) == expected
  let grown = tree |> bytes_tree.prepend(<<7>>) |> bytes_tree.append(<<8>>)
  let assert True = read(grown) == <<7, expected:bits, 8>>
  let assert True = read(grown) == bytes_tree.to_bit_array(grown)
  let assert True = read(alias) == expected
  let assert True = read(nest(tree, 1000)) == expected
  let wide = list.repeat(bytes_tree.from_string("x"), 1000) |> bytes_tree.concat
  let assert True = read(wide) == bytes_tree.to_bit_array(wide)
  let wide_text =
    list.repeat(string_tree.from_string("한"), 1000)
    |> string_tree.concat
    |> bytes_tree.from_string_tree
  let assert True = read(wide_text) == bytes_tree.to_bit_array(wide_text)
  expected
}

pub fn capture() -> Nil {
  retain(mixed())
}

pub fn paused() -> #(BitArray, BitArray) {
  let tree = mixed()
  let result = read_after_pause(tree)
  let assert True = read(tree) == result.0
  let assert True = bytes_tree.to_bit_array(tree) == result.1
  result
}

pub fn main() -> Nil {
  let expected = verify()
  let assert True = paused() == #(expected, expected)
  capture()
  Nil
}
