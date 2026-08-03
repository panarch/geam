import gleam/dict
import gleam/dynamic/decode
import gleam/json
import gleam/option.{None}

pub type Person {
  Person(name: String, scores: List(Int))
}

fn person_decoder() {
  use name <- decode.field("name", decode.string)
  use scores <- decode.field("scores", decode.list(decode.int))
  decode.success(Person(name: name, scores: scores))
}

pub fn main() {
  assert json.parse("null", decode.optional(decode.int)) == Ok(None)
  assert json.parse("true", decode.bool) == Ok(True)
  assert json.parse("\"text\"", decode.string) == Ok("text")
  assert json.parse("123456789012345678901234567890", decode.int)
    == Ok(123_456_789_012_345_678_901_234_567_890)
  assert json.parse("1.25", decode.float) == Ok(1.25)
  assert json.parse_bits(<<"[1,2,3]":utf8>>, decode.list(decode.int))
    == Ok([1, 2, 3])

  let assert Ok(list_dynamic) = json.parse("[1,2,3]", decode.dynamic)
  assert decode.run(list_dynamic, decode.list(decode.int)) == Ok([1, 2, 3])

  let assert Ok(dict_dynamic) =
    json.parse("{\"one\":1,\"two\":2}", decode.dynamic)
  assert decode.run(dict_dynamic, decode.dict(decode.string, decode.int))
    == Ok(dict.from_list([#("one", 1), #("two", 2)]))

  assert json.parse("{\"name\":\"Ada\",\"scores\":[9,10]}", person_decoder())
    == Ok(Person(name: "Ada", scores: [9, 10]))

  let assert Ok(duplicates) =
    json.parse(
      "{\"value\":1,\"value\":2}",
      decode.dict(decode.string, decode.int),
    )
  assert dict.get(duplicates, "value") == Ok(1)

  Nil
}
// @geam:expect Nil
