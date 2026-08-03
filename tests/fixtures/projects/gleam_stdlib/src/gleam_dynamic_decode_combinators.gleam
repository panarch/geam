import gleam/dynamic
import gleam/dynamic/decode
import gleam/int
import gleam/list

fn positive_int(data: dynamic.Dynamic) -> Result(Int, Int) {
  case decode.run(data, decode.int) {
    Ok(value) if value > 0 -> Ok(value)
    Ok(_) | Error(_) -> Error(0)
  }
}

pub fn main() {
  assert decode.decode_error("Number", dynamic.string("one")) == [
    decode.DecodeError(expected: "Number", found: "String", path: []),
  ]

  let doubled = decode.int |> decode.map(fn(value) { value * 2 })
  assert decode.run(dynamic.int(4), doubled) == Ok(8)

  let mapped_errors =
    decode.int
    |> decode.map_errors(fn(errors) {
      list.map(errors, fn(error) {
        decode.DecodeError(
          expected: "Mapped",
          found: error.found,
          path: error.path,
        )
      })
    })
  assert decode.run(dynamic.string("four"), mapped_errors) == Error([
    decode.DecodeError(expected: "Mapped", found: "String", path: []),
  ])

  let collapsed = decode.int |> decode.collapse_errors("WholeNumber")
  assert decode.run(dynamic.string("four"), collapsed) == Error([
    decode.DecodeError(expected: "WholeNumber", found: "String", path: []),
  ])

  let increment_positive =
    decode.int
    |> decode.then(fn(value) {
      case value > 0 {
        True -> decode.success(value + 1)
        False -> decode.failure(0, expected: "PositiveInt")
      }
    })
  assert decode.run(dynamic.int(4), increment_positive) == Ok(5)
  assert decode.run(dynamic.int(-1), increment_positive) == Error([
    decode.DecodeError(expected: "PositiveInt", found: "Int", path: []),
  ])

  let string_or_int =
    decode.one_of(decode.string, or: [
      decode.int |> decode.map(int.to_string),
    ])
  assert decode.run(dynamic.string("four"), string_or_int) == Ok("four")
  assert decode.run(dynamic.int(4), string_or_int) == Ok("4")
  assert decode.run(dynamic.bool(True), string_or_int) == Error([
    decode.DecodeError(expected: "String", found: "Bool", path: []),
  ])

  let failed = decode.failure(0, expected: "Never")
  assert decode.run(dynamic.float(1.0), failed) == Error([
    decode.DecodeError(expected: "Never", found: "Float", path: []),
  ])

  let positive = decode.new_primitive_decoder("PositiveInt", positive_int)
  assert decode.run(dynamic.int(7), positive) == Ok(7)
  assert decode.run(dynamic.int(-7), positive) == Error([
    decode.DecodeError(expected: "PositiveInt", found: "Int", path: []),
  ])

  Nil
}

// @geam:expect Nil
