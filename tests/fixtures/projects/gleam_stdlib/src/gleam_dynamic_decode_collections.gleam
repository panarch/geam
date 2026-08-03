import gleam/dict
import gleam/dynamic
import gleam/dynamic/decode

pub fn main() {
  let list_data =
    dynamic.list([dynamic.int(1), dynamic.int(2), dynamic.int(3)])
  let array_data =
    dynamic.array([dynamic.int(4), dynamic.int(5), dynamic.int(6)])
  assert decode.run(list_data, decode.list(decode.int)) == Ok([1, 2, 3])
  assert decode.run(array_data, decode.list(decode.int)) == Ok([4, 5, 6])

  let invalid_list =
    dynamic.list([dynamic.int(1), dynamic.string("two"), dynamic.int(3)])
  assert decode.run(invalid_list, decode.list(decode.int)) == Error([
    decode.DecodeError(expected: "Int", found: "String", path: ["1"]),
  ])
  assert decode.run(dynamic.int(1), decode.list(decode.int)) == Error([
    decode.DecodeError(expected: "List", found: "Int", path: []),
  ])

  let properties = dynamic.properties([
    #(dynamic.string("one"), dynamic.int(1)),
    #(dynamic.string("two"), dynamic.int(2)),
  ])
  assert decode.run(properties, decode.dict(decode.string, decode.int)) ==
    Ok(dict.from_list([#("one", 1), #("two", 2)]))
  assert decode.run(dynamic.list([]), decode.dict(decode.string, decode.int)) ==
    Error([decode.DecodeError(expected: "Dict", found: "List", path: [])])

  #(list_data, array_data, properties)
}

// @geam:expect #([1, 2, 3], #(4, 5, 6), dict.from_list([#("one", 1), #("two", 2)]))
