import future_builtins/native
import geam/future
import gleam/dict
import gleam/dynamic
import gleam/dynamic/decode
import gleam/io
import gleam/json
import gleam/string_tree
import gleam/time/timestamp

pub fn work() -> future.Future(Int) {
  let data = dict.from_list([#("numbers", [1, 2, 3])])
  let boxed =
    dynamic.properties([
      #(
        dynamic.string("numbers"),
        dynamic.list([dynamic.int(1), dynamic.int(2), dynamic.int(3)]),
      ),
    ])
  let tree = string_tree.from_strings(["hello", " ", "world"])
  let encoded = json.object([#("message", json.string("hello world"))])
  let #(started, _) =
    timestamp.system_time() |> timestamp.to_unix_seconds_and_nanoseconds
  io.println("created")
  let pending =
    native.apply(fn(value) {
      assert decode.run(
          boxed,
          decode.dict(decode.string, decode.list(decode.int)),
        )
        == Ok(data)
      assert string_tree.to_string(tree) == "hello world"
      assert json.to_string(encoded) == "{\"message\":\"hello world\"}"
      let assert Ok(decoded) =
        json.parse(json.to_string(encoded), decode.dynamic)
      assert decode.run(decoded, decode.at(["message"], decode.string))
        == Ok("hello world")
      let #(now, _) =
        timestamp.system_time() |> timestamp.to_unix_seconds_and_nanoseconds
      assert now == started + value - 19
      io.println("callback")
      echo value
      value * 2
    })
  let alias = pending
  let independent = future.ready(82)
  let keys =
    dict.from_list([#(pending, "operation"), #(independent, "independent")])
  let compound_keys = dict.from_list([#(#(pending, "key"), 42)])
  use value <- future.map(pending)
  assert pending == alias
  assert pending != independent
  assert dict.size(keys) == 2
  assert dict.get(keys, alias) == Ok("operation")
  assert dict.get(keys, independent) == Ok("independent")
  assert dict.get(compound_keys, #(alias, "key")) == Ok(42)
  value
}

pub fn later() -> Int {
  io.println("later")
  let #(now, _) =
    timestamp.system_time() |> timestamp.to_unix_seconds_and_nanoseconds
  now
}
