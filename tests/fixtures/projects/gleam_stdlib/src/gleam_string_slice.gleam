import gleam/string

pub fn main() {
  assert string.slice(from: "A👍🏽é", at_index: 1, length: 1) == "👍🏽"
  assert string.slice(from: "gleam", at_index: -2, length: 2) == "am"
  assert string.slice(from: "gleam", at_index: 20, length: 2) == ""
  assert string.slice(from: "gleam", at_index: 1, length: 0) == ""
  assert string.drop_start(from: "A👍🏽é", up_to: 1) == "👍🏽é"
  assert string.drop_end(from: "A👍🏽é", up_to: 1) == "A👍🏽"
  assert string.split("a,b,c", on: ",") == ["a", "b", "c"]
  assert string.split("A👍🏽é", on: "") == ["A", "👍🏽", "é"]
  assert string.split_once("home/gleam/desktop", on: "/")
    == Ok(#("home", "gleam/desktop"))
  assert string.split_once("home", on: "?") == Error(Nil)
  assert string.split_once("home", on: "") == Error(Nil)

  let assert Ok(next_line) = string.utf_codepoint(133)
  let assert Ok(left_to_right) = string.utf_codepoint(8206)
  let assert Ok(non_breaking_space) = string.utf_codepoint(160)
  let next_line = string.from_utf_codepoints([next_line])
  let left_to_right = string.from_utf_codepoints([left_to_right])
  let non_breaking_space = string.from_utf_codepoints([non_breaking_space])
  let padded = next_line <> left_to_right <> "hats" <> next_line
  assert string.trim(padded) == "hats"
  assert string.trim_start(padded) == "hats" <> next_line
  assert string.trim_end(padded) == next_line <> left_to_right <> "hats"
  assert string.trim(non_breaking_space <> "hats" <> non_breaking_space)
    == non_breaking_space <> "hats" <> non_breaking_space

  "slice"
}

// @geam:expect "slice"
