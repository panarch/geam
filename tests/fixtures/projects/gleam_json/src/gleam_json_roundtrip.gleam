import gleam/dynamic/decode
import gleam/json
import gleam/result

pub fn main() {
  let value =
    json.object([
      #("name", json.string("Ada")),
      #(
        "values",
        json.preprocessed_array([json.int(1), json.bool(True), json.null()]),
      ),
    ])
  let same =
    json.object([
      #("name", json.string("Ada")),
      #(
        "values",
        json.preprocessed_array([json.int(1), json.bool(True), json.null()]),
      ),
    ])
  assert value == same

  let encoded = json.to_string(value)
  let assert Ok(decoded) = json.parse(encoded, decode.dynamic)
  assert decode.run(decoded, decode.at(["name"], decode.string)) == Ok("Ada")
  assert decode.run(decoded, decode.at(["values"], decode.list(decode.dynamic)))
    |> result.is_ok

  value
}
// @geam:expect "{\"name\":\"Ada\",\"values\":[1,true,null]}"
