import example_native_records as records
import gleam/dict
import gleam/dynamic
import gleam/dynamic/decode
import gleam/string

pub fn main() {
  let record = records.record("visits", 42)
  let native = records.erase(record)
  let tuple = records.erase(#(records.key("record"), "visits", 42))
  let decoder = {
    use label <- decode.field(1, decode.string)
    use count <- decode.field(2, decode.int)
    decode.success(#(label, count))
  }
  assert dynamic.classify(native) == "Array"
  assert decode.run(native, decoder) == Ok(#("visits", 42))
  assert decode.run(tuple, decoder) == Ok(#("visits", 42))
  assert native == tuple
  assert tuple == native
  assert string.inspect(native) == "Record(\"visits\", 42)"
  assert string.inspect(tuple) == string.inspect(native)

  let indexed = dict.from_list([#(native, "found")])
  assert dict.get(indexed, tuple) == Ok("found")
  assert records.erase(records.key("record")) != dynamic.string("record")

  let properties = records.erase(dict.from_list([#("visits", 42)]))
  assert decode.run(properties, decode.dict(decode.string, decode.int))
    == Ok(dict.from_list([#("visits", 42)]))
    as "decode a native dictionary"
  assert decode.run(properties, {
      use visits <- decode.field("visits", decode.int)
      decode.success(visits)
    })
    == Ok(42)

  let suffix = " total"
  assert records.map(native, fn(label, count) { #(label <> suffix, count + 1) })
    == Ok(#("visits total", 43))
  assert records.map(tuple, fn(label, count) { #(label, count) })
    == Ok(#("visits", 42))
  assert records.map(dynamic.int(42), fn(_, _) { Nil }) == Error(Nil)
  assert records.map(
      records.erase(#(records.key("record"), "bad", "count")),
      fn(_, _) { Nil },
    )
    == Error(Nil)
}
