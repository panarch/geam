import gleam/bytes_tree
import gleam/string_tree

pub fn main() {
  let empty = bytes_tree.new()
  assert bytes_tree.to_bit_array(empty) == <<>>
  assert bytes_tree.byte_size(empty) == 0

  let text = bytes_tree.from_string("hello")
  assert bytes_tree.to_bit_array(text) == <<"hello":utf8>>
  assert bytes_tree.byte_size(text) == 5

  let text_tree = string_tree.from_strings(["hel", "lo"])
  let from_text_tree = bytes_tree.from_string_tree(text_tree)
  assert bytes_tree.to_bit_array(from_text_tree) == <<"hello":utf8>>
  assert bytes_tree.byte_size(from_text_tree) == 5

  let bytes = bytes_tree.from_bit_array(<<1, 2, 3>>)
  assert bytes_tree.to_bit_array(bytes) == <<1, 2, 3>>
  assert bytes_tree.byte_size(bytes) == 3

  let padded = bytes_tree.from_bit_array(<<5:size(3)>>)
  assert bytes_tree.to_bit_array(padded) == <<5:size(3), 0:size(5)>>
  assert bytes_tree.byte_size(padded) == 1

  Nil
}
// @geam:expect Nil
