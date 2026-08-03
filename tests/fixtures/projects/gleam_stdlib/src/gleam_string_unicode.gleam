import gleam/list
import gleam/option
import gleam/string

pub fn main() {
  assert string.pop_grapheme("A👍🏽") == Ok(#("A", "👍🏽"))
  assert string.pop_grapheme("") == Error(Nil)
  assert string.to_graphemes("A👍🏽é") == ["A", "👍🏽", "é"]
  assert string.to_utf_codepoints("A💜")
    |> list.map(string.utf_codepoint_to_int)
    == [65, 128_156]

  let assert Ok(a) = string.utf_codepoint(97)
  let assert Ok(b) = string.utf_codepoint(98)
  assert string.from_utf_codepoints([a, b]) == "ab"
  assert string.utf_codepoint(-1) == Error(Nil)
  assert string.utf_codepoint(55_296) == Error(Nil)
  assert string.utf_codepoint(1_114_112) == Error(Nil)
  assert string.to_option("") == option.None
  assert string.to_option("hats") == option.Some("hats")
  assert string.first("icecream") == Ok("i")
  assert string.first("") == Error(Nil)
  assert string.last("icecream") == Ok("m")
  assert string.last("") == Error(Nil)
  assert string.capitalise("mAMOUNA") == "Mamouna"
  assert string.capitalise("") == ""

  "unicode"
}

// @geam:expect "unicode"
