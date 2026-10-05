import gleam/bytes_tree.{type BytesTree}
import gleam/list
import gleam/string_tree

@external(erlang, "bytes_tree_service_native", "make")
fn make(bytes: BitArray) -> BytesTree

@external(erlang, "bytes_tree_service_native", "make_pair")
fn make_pair(bytes: BitArray) -> #(BytesTree, BytesTree)

@external(erlang, "bytes_tree_service_native", "make_result")
fn make_result(bytes: BitArray, success: Bool) -> Result(BytesTree, BytesTree)

@external(erlang, "bytes_tree_service_native", "make_nested")
fn make_nested(
  bytes: BitArray,
) -> List(#(Result(BytesTree, BytesTree), BytesTree))

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
  verify_outputs()
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

fn verify_outputs() -> Nil {
  let assert <<>> = bytes_tree.to_bit_array(make(<<>>))
  let original = <<0, 255, 128, 42>>
  let tree = make(original)
  let alias = tree
  let assert True = bytes_tree.to_bit_array(tree) == original
  let assert True = read(tree) == original
  let grown = tree |> bytes_tree.prepend(<<7>>) |> bytes_tree.append(<<8>>)
  let assert True = bytes_tree.to_bit_array(grown) == <<7, original:bits, 8>>
  let assert True = bytes_tree.to_bit_array(alias) == original
  let assert True = bytes_tree.to_bit_array(make(original)) == original
  let partial = <<5:size(3)>>
  let assert <<160>> = bytes_tree.to_bit_array(make(partial))
  let assert True =
    bytes_tree.to_bit_array(make(partial))
    == bytes_tree.to_bit_array(bytes_tree.from_bit_array(partial))
  let assert <<5:size(3)>> = partial
  let assert <<_:bytes-size(1), slice:bytes-size(2), _:bytes>> = <<
    42,
    255,
    0,
    99,
  >>
  let assert <<255, 0>> = bytes_tree.to_bit_array(make(slice))
  let pair = make_pair(original)
  let assert True = bytes_tree.to_bit_array(pair.0) == original
  let assert True = bytes_tree.to_bit_array(pair.1) == original
  let assert Ok(ok) = make_result(original, True)
  let assert Error(error) = make_result(original, False)
  let assert True = bytes_tree.to_bit_array(ok) == original
  let assert True = bytes_tree.to_bit_array(error) == original
  let assert [#(Ok(first), first_alias), #(Error(second), second_alias)] =
    make_nested(original)
  let combined = bytes_tree.concat([first, first_alias, second, second_alias])
  let assert True =
    bytes_tree.to_bit_array(combined)
    == <<original:bits, original:bits, original:bits, original:bits>>
  let large =
    list.repeat(original, 4096)
    |> bytes_tree.concat_bit_arrays
    |> bytes_tree.to_bit_array
  let assert True = bytes_tree.to_bit_array(make(large)) == large
  Nil
}

pub fn capture() -> Nil {
  retain(mixed())
}

pub fn capture_generated() -> Nil {
  let tree = make(<<0, 255, 128, 42>>)
  let alias = tree
  let grown = bytes_tree.concat([tree, make(<<5:size(3)>>)])
  let assert <<0, 255, 128, 42, 160>> = bytes_tree.to_bit_array(grown)
  retain(alias)
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
  capture_generated()
  Nil
}
