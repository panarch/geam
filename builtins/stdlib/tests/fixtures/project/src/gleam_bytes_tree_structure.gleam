import gleam/bit_array
import gleam/bytes_tree
import gleam/list
import gleam/string

pub fn main() {
  let base = bytes_tree.from_string("middle")
  let left = bytes_tree.prepend_string(base, "left-")
  let right = bytes_tree.append_string(base, "-right")

  assert bytes_tree.to_bit_array(base) == <<"middle":utf8>>
  assert bytes_tree.to_bit_array(left) == <<"left-middle":utf8>>
  assert bytes_tree.to_bit_array(right) == <<"middle-right":utf8>>

  let with_bytes =
    bytes_tree.prepend(base, <<"[":utf8>>)
    |> bytes_tree.append(<<"]":utf8>>)
  assert bytes_tree.to_bit_array(with_bytes) == <<"[middle]":utf8>>

  let with_trees =
    bytes_tree.prepend_tree(base, bytes_tree.from_string("before-"))
    |> bytes_tree.append_tree(bytes_tree.from_string("-after"))
  assert bytes_tree.to_bit_array(with_trees) == <<"before-middle-after":utf8>>

  let concatenated = bytes_tree.concat([
    bytes_tree.from_string("one"),
    bytes_tree.from_string("-"),
    bytes_tree.from_string("two"),
  ])
  assert bytes_tree.to_bit_array(concatenated) == <<"one-two":utf8>>

  let concatenated_bits = bytes_tree.concat_bit_arrays([
    <<5:size(3)>>,
    <<3:size(2)>>,
  ])
  assert bytes_tree.to_bit_array(concatenated_bits)
    == <<5:size(3), 0:size(5), 3:size(2), 0:size(6)>>

  let flat = bytes_tree.from_string("ab")
  let segmented = bytes_tree.concat([
    bytes_tree.from_string("a"),
    bytes_tree.from_string("b"),
  ])
  assert flat != segmented
  assert bytes_tree.to_bit_array(flat) == bytes_tree.to_bit_array(segmented)

  let deep =
    list.repeat(bytes_tree.from_string("x"), times: 512)
    |> list.fold(from: bytes_tree.new(), with: bytes_tree.append_tree)
  let deep_bits = bytes_tree.to_bit_array(deep)
  assert bytes_tree.byte_size(deep) == 512
  assert bit_array.to_string(deep_bits) == Ok(string.repeat("x", times: 512))

  Nil
}
// @geam:expect Nil
