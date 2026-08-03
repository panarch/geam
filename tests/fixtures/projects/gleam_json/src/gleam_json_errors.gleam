import gleam/dynamic/decode
import gleam/json

pub fn main() {
  assert json.parse("", decode.dynamic) == Error(json.UnexpectedEndOfInput)
  assert json.parse("[", decode.dynamic) == Error(json.UnexpectedEndOfInput)
  assert json.parse("}", decode.dynamic) == Error(json.UnexpectedByte("0x7D"))
  assert json.parse("\"\\uxxxx\"", decode.dynamic)
    == Error(json.UnexpectedSequence("\\uxxxx"))
  assert json.parse("\"\\uD800\"", decode.dynamic)
    == Error(json.UnexpectedEndOfInput)
  assert json.parse("true false", decode.dynamic)
    == Error(json.UnexpectedByte("0x66"))
  assert json.parse("1e400", decode.dynamic)
    == Error(json.UnexpectedSequence("1.0e400"))
  assert json.parse_bits(<<255>>, decode.dynamic)
    == Error(json.UnexpectedByte("0xFF"))
  assert json.parse_bits(<<1:size(1)>>, decode.dynamic)
    == Error(json.UnexpectedByte(""))

  assert json.parse("1", decode.string)
    == Error(
      json.UnableToDecode([
        decode.DecodeError(expected: "String", found: "Int", path: []),
      ]),
    )
  assert json.parse(
      "{\"user\":{\"age\":\"old\"}}",
      decode.at(["user", "age"], decode.int),
    )
    == Error(
      json.UnableToDecode([
        decode.DecodeError(expected: "Int", found: "String", path: [
          "user",
          "age",
        ]),
      ]),
    )

  Nil
}
// @geam:expect Nil
