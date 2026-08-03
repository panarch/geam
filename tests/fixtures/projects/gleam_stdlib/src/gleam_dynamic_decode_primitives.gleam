import gleam/dynamic
import gleam/dynamic/decode
import gleam/option.{None, Some}

pub fn main() {
  assert decode.run(dynamic.string("Ada"), decode.string) == Ok("Ada")
  assert decode.run(dynamic.bool(True), decode.bool) == Ok(True)
  assert decode.run(dynamic.int(42), decode.int) == Ok(42)
  assert decode.run(dynamic.float(1.5), decode.float) == Ok(1.5)
  assert decode.run(dynamic.bit_array(<<1, 2>>), decode.bit_array) ==
    Ok(<<1, 2>>)
  assert decode.run(dynamic.int(42), decode.float) == Error([
    decode.DecodeError(expected: "Float", found: "Int", path: []),
  ])
  assert decode.run(dynamic.float(42.0), decode.int) == Error([
    decode.DecodeError(expected: "Int", found: "Float", path: []),
  ])
  assert decode.run(dynamic.bit_array(<<"Ada":utf8>>), decode.string) == Error([
    decode.DecodeError(expected: "String", found: "BitArray", path: []),
  ])
  assert decode.run(dynamic.nil(), decode.optional(decode.int)) == Ok(None)
  assert decode.run(dynamic.int(7), decode.optional(decode.int)) == Ok(Some(7))
  assert decode.run(dynamic.int(7), decode.dynamic) == Ok(dynamic.int(7))

  #(True, "Ada", 42, 1.5, <<1, 2>>)
}

// @geam:expect #(True, "Ada", 42, 1.5, <<1, 2>>)
