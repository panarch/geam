import gleam/string_tree

pub fn main() {
  let empty = string_tree.new()
  let flat = string_tree.from_string("ab")
  let segmented = string_tree.from_strings(["a", "b"])
  let concatenated = string_tree.concat([
    string_tree.from_string("a"),
    string_tree.from_string("b"),
  ])
  let built =
    string_tree.append(string_tree.prepend(empty, "one-"), "two")
    |> string_tree.prepend_tree(string_tree.from_string("zero-"))
    |> string_tree.append_tree(string_tree.from_string("-three"))

  assert string_tree.is_empty(empty)
  assert string_tree.is_empty(string_tree.from_string(""))
  assert empty != string_tree.from_string("")
  assert segmented != flat
  assert segmented == concatenated
  assert string_tree.is_equal(segmented, flat)
  assert string_tree.to_string(built) == "zero-one-two-three"
  assert string_tree.byte_size(string_tree.from_string("👍")) == 4
  assert string_tree.to_string(string_tree.join([flat, flat], with: "-")) == "ab-ab"
  assert string_tree.to_string(string_tree.lowercase(string_tree.from_string("Gleam")))
    == "gleam"
  assert string_tree.to_string(string_tree.uppercase(string_tree.from_string("Gleam")))
    == "GLEAM"
  assert string_tree.to_string(string_tree.reverse(string_tree.from_string("A👍🏽é")))
    == "é👍🏽A"
  assert string_tree.split(string_tree.from_string("a,b,c"), on: ",")
    == [
      string_tree.from_string("a"),
      string_tree.from_string("b"),
      string_tree.from_string("c"),
    ]
  assert string_tree.split(string_tree.from_string("abc"), on: "")
    == [string_tree.from_string("abc")]
  assert string_tree.to_string(
    string_tree.replace(string_tree.from_string("a-b-a"), each: "a", with: "x"),
  ) == "x-b-x"
  assert string_tree.to_string(
    string_tree.replace(string_tree.from_string("abc"), each: "", with: "x"),
  ) == "abc"

  built
}

// @geam:expect string_tree.from_string("zero-one-two-three")
