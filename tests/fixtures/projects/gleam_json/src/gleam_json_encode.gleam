import gleam/dict
import gleam/dynamic/decode
import gleam/json
import gleam/option.{None, Some}
import gleam/string
import gleam/string_tree

fn assert_control(codepoint: Int, expected: String) {
  let assert Ok(codepoint) = string.utf_codepoint(codepoint)
  let value = string.from_utf_codepoints([codepoint])
  assert json.to_string(json.string(value)) == expected
}

pub fn main() {
  assert json.to_string(json.string("quote: \" slash: / backslash: \\"))
    == "\"quote: \\\" slash: / backslash: \\\\\""
  assert json.to_string(json.string("한글 💜")) == "\"한글 💜\""

  assert_control(0, "\"\\u0000\"")
  assert_control(1, "\"\\u0001\"")
  assert_control(2, "\"\\u0002\"")
  assert_control(3, "\"\\u0003\"")
  assert_control(4, "\"\\u0004\"")
  assert_control(5, "\"\\u0005\"")
  assert_control(6, "\"\\u0006\"")
  assert_control(7, "\"\\u0007\"")
  assert_control(8, "\"\\b\"")
  assert_control(9, "\"\\t\"")
  assert_control(10, "\"\\n\"")
  assert_control(11, "\"\\u000B\"")
  assert_control(12, "\"\\f\"")
  assert_control(13, "\"\\r\"")
  assert_control(14, "\"\\u000E\"")
  assert_control(15, "\"\\u000F\"")
  assert_control(16, "\"\\u0010\"")
  assert_control(17, "\"\\u0011\"")
  assert_control(18, "\"\\u0012\"")
  assert_control(19, "\"\\u0013\"")
  assert_control(20, "\"\\u0014\"")
  assert_control(21, "\"\\u0015\"")
  assert_control(22, "\"\\u0016\"")
  assert_control(23, "\"\\u0017\"")
  assert_control(24, "\"\\u0018\"")
  assert_control(25, "\"\\u0019\"")
  assert_control(26, "\"\\u001A\"")
  assert_control(27, "\"\\u001B\"")
  assert_control(28, "\"\\u001C\"")
  assert_control(29, "\"\\u001D\"")
  assert_control(30, "\"\\u001E\"")
  assert_control(31, "\"\\u001F\"")

  assert json.to_string(json.bool(True)) == "true"
  assert json.to_string(json.bool(False)) == "false"
  assert json.to_string(json.int(123_456_789_012_345_678_901_234_567_890))
    == "123456789012345678901234567890"
  assert json.to_string(json.float(1.0)) == "1.0"
  assert json.to_string(json.float(-0.0)) == "-0.0"
  assert json.to_string(json.float(1.0e20)) == "1.0e20"
  assert json.to_string(json.float(1.0e-7)) == "1.0e-7"
  assert json.to_string(json.null()) == "null"
  assert json.to_string(json.nullable(Some(7), json.int)) == "7"
  assert json.to_string(json.nullable(None, json.int)) == "null"

  let object =
    json.object([
      #("first", json.int(1)),
      #("first", json.int(2)),
      #("items", json.preprocessed_array([json.bool(True), json.null()])),
    ])
  assert json.to_string(object)
    == "{\"first\":1,\"first\":2,\"items\":[true,null]}"

  assert json.to_string(json.array([1, 2, 3], json.int)) == "[1,2,3]"
  assert json.to_string(json.preprocessed_array([])) == "[]"
  let tree = json.to_string_tree(json.array(["one", "two"], json.string))
  assert string_tree.is_equal(
    tree,
    string_tree.from_string("[\"one\",\"two\"]"),
  )

  let source = dict.from_list([#("one", 1), #("two", 2)])
  let encoded = json.to_string(json.dict(source, fn(key) { key }, json.int))
  assert json.parse(encoded, decode.dict(decode.string, decode.int))
    == Ok(source)

  Nil
}
// @geam:expect Nil
