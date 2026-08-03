import gleam/dynamic
import gleam/dynamic/decode

pub fn main() {
  let data = dynamic.properties([
    #(dynamic.string("name"), dynamic.string("Ada")),
    #(
      dynamic.string("nested"),
      dynamic.properties([#(dynamic.string("count"), dynamic.int(2))]),
    ),
  ])

  let decoder = {
    use name <- decode.field("name", decode.string)
    use age <- decode.optional_field("age", 36, decode.int)
    decode.success(#(name, age))
  }
  assert decode.run(data, decoder) == Ok(#("Ada", 36))
  let subfield_decoder = {
    use count <- decode.subfield(["nested", "count"], decode.int)
    decode.success(count)
  }
  assert decode.run(data, subfield_decoder) == Ok(2)
  assert decode.run(data, decode.at(["nested", "count"], decode.int)) == Ok(2)
  assert decode.run(data, decode.optionally_at(["nested", "missing"], 7, decode.int)) ==
    Ok(7)
  assert decode.run(data, decode.at(["missing"], decode.int)) == Error([
    decode.DecodeError(expected: "Field", found: "Nothing", path: ["missing"]),
  ])

  let list_data = dynamic.list([
    dynamic.int(0),
    dynamic.int(1),
    dynamic.int(2),
    dynamic.int(3),
    dynamic.int(4),
    dynamic.int(5),
    dynamic.int(6),
    dynamic.int(7),
    dynamic.int(8),
  ])
  let array_data = dynamic.array([
    dynamic.int(0),
    dynamic.int(1),
    dynamic.int(2),
    dynamic.int(3),
    dynamic.int(4),
    dynamic.int(5),
    dynamic.int(6),
    dynamic.int(7),
    dynamic.int(8),
  ])
  assert decode.run(list_data, decode.at([7], decode.int)) == Ok(7)
  assert decode.run(list_data, decode.at([8], decode.int)) == Error([
    decode.DecodeError(expected: "Indexable", found: "List", path: []),
  ])
  assert decode.run(array_data, decode.at([8], decode.int)) == Ok(8)
  assert decode.run(array_data, decode.optionally_at([20], 20, decode.int)) == Ok(20)

  #("Ada", data)
}

// @geam:expect #("Ada", dict.from_list([#("name", "Ada"), #("nested", dict.from_list([#("count", 2)]))]))
