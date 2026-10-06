import gleam/dynamic.{type Dynamic}
import gleam/dynamic/decode
import gleam/io

@external(erlang, "gleam@function", "identity")
fn coerce(value: a) -> b

pub fn scalar_control() -> Bool {
  let value: Int = coerce(42)
  value == 42
}

pub fn function_control() -> Bool {
  let function: fn(Int) -> Int = coerce(fn(value: Int) { value + 1 })
  function(41) == 42
}

pub type Handler(a, b) {
  Handler(callback: fn(a) -> b)
}

fn increment(value: Int) -> Int {
  value + 1
}

pub fn named_erasure() -> Bool {
  let function: fn(Dynamic) -> Dynamic = coerce(increment)
  let assert Ok(value) = decode.run(function(dynamic.int(41)), decode.int)
  value == 42
}

pub fn captured_erasure() -> Bool {
  let amount = 2
  let function: fn(Dynamic) -> Dynamic =
    coerce(fn(value: Int) { value + amount })
  let assert Ok(value) = decode.run(function(dynamic.int(40)), decode.int)
  value == 42
}

pub fn nested_function_control() -> Bool {
  let handler: Handler(Int, Int) = coerce(Handler(increment))
  handler.callback(41) == 42
}

pub fn nested_function_erasure() -> Bool {
  let handler: Handler(Dynamic, Dynamic) = coerce(Handler(increment))
  let assert Ok(value) =
    decode.run(handler.callback(dynamic.int(41)), decode.int)
  value == 42
}

pub fn function_erasure() -> Bool {
  let function: fn(Dynamic) -> Dynamic = coerce(fn(value: Int) { value + 1 })
  let assert Ok(value) = decode.run(function(dynamic.int(41)), decode.int)
  value == 42
}

pub fn dynamic_function_control() -> Bool {
  let function: fn(Dynamic) -> Dynamic = coerce(fn(value: Dynamic) { value })
  let assert Ok(value) = decode.run(function(dynamic.int(42)), decode.int)
  value == 42
}

pub fn roundtrip() -> Bool {
  let bias = 2
  let original = fn(a: Int) { a + bias }
  let view: fn(Dynamic) -> Dynamic = coerce(original)
  let erased: Dynamic = coerce(view)
  let restored: fn(Int) -> Int = coerce(erased)
  let again: fn(Dynamic) -> Dynamic = coerce(restored)
  let callback: fn(Dynamic) -> Dynamic = coerce(increment)
  let tuple: #(fn(Dynamic) -> Dynamic, List(fn(Dynamic) -> Dynamic)) =
    coerce(#(original, [increment]))
  let assert #(tuple_callback, [list_callback]) = tuple
  let assert Ok(tuple_value) =
    decode.run(tuple_callback(dynamic.int(40)), decode.int)
  let assert Ok(list_value) =
    decode.run(list_callback(dynamic.int(41)), decode.int)
  let assert Ok(again_value) = decode.run(again(dynamic.int(40)), decode.int)
  restored == original
  && again == view
  && callback(dynamic.int(41)) == dynamic.int(42)
  && restored(40) == 42
  && tuple_value == 42
  && list_value == 42
  && again_value == 42
}

pub fn invalid_input() -> Bool {
  let view: fn(Dynamic) -> Dynamic = coerce(increment)
  let _ = view(dynamic.string("wrong"))
  True
}

pub fn invalid_result() -> Int {
  let view: fn(Dynamic) -> Int = coerce(fn(_a: Int) { "wrong" })
  view(dynamic.int(42))
}

pub fn invalid_arity() -> Int {
  let view: fn(Dynamic, Dynamic) -> Dynamic = coerce(increment)
  let value: Int = coerce(view(dynamic.int(20), dynamic.int(22)))
  value
}

pub fn main() {
  assert scalar_control()
  assert function_control()
  assert dynamic_function_control()
  assert nested_function_control()
  assert function_erasure()
  assert named_erasure()
  assert captured_erasure()
  assert nested_function_erasure()
  assert roundtrip()
  io.println("native function views: 42")
}
