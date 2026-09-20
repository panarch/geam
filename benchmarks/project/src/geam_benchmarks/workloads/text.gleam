import gleam/list
import gleam/string

pub fn normalize_fields(input: String) -> String {
  input
  |> string.split(",")
  |> list.map(string.trim)
  |> string.join("|")
}

pub fn bit_checksum(input: BitArray, total: Int) -> Int {
  case input {
    <<a:8, b:8, c:8, d:8, rest:bits>> ->
      bit_checksum(rest, total + a + b * 2 + c * 3 + d * 4)
    <<>> -> total
    _ -> panic as "incomplete benchmark record"
  }
}

pub fn parse_sum(input: BitArray) -> Result(Int, Nil) {
  parse_decimal(input, 0, 0)
}

fn parse_decimal(
  input: BitArray,
  current: Int,
  total: Int,
) -> Result(Int, Nil) {
  case input {
    <<digit:8, rest:bits>> if digit >= 48 && digit <= 57 ->
      parse_decimal(rest, current * 10 + digit - 48, total)
    <<separator:8, rest:bits>> if separator == 44 || separator == 10 ->
      parse_decimal(rest, 0, total + current)
    <<>> -> Ok(total + current)
    _ -> Error(Nil)
  }
}
