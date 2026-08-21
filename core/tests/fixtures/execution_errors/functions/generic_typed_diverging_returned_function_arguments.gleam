pub type Token {
  Token
}

fn fail() -> value {
  panic as "generic returned function argument failed"
}

pub type ValueBox(value) {
  ValueBox(value)
}

fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn identity(value) {
  value
}

fn diverge(_value: Int) -> value {
  panic as "returned diverging function should not run"
}

fn result_int(_other: Int) -> fn() -> Int { fn() { 1 } }
fn result_float(_other: Int) -> fn() -> Float { fn() { 1.5 } }
fn result_string(_other: Int) -> fn() -> String { fn() { "value" } }
fn result_bit_array(_other: Int) -> fn() -> BitArray { fn() { <<1>> } }
fn result_utf_codepoint(_other: Int) -> fn() -> UtfCodepoint { fn() { codepoint() } }
fn result_custom(_other: Int) -> fn() -> Token { fn() { Token } }
fn result_bool(_other: Int) -> fn() -> Bool { fn() { True } }
fn result_nil(_other: Int) -> fn() -> Nil { fn() { Nil } }
fn result_tuple(_other: Int) -> fn() -> #(Int) { fn() { #(1) } }
fn result_list(_other: Int) -> fn() -> List(Int) { fn() { [1] } }
fn result_function(_other: Int) -> fn() -> fn() -> Int { fn() { fn() { 1 } } }
fn result_generic(_other: Int) { identity }
fn result_never(_other: Int) { diverge }
fn result_tuple_never(_other: Int) -> fn() -> #(value) {
  fn() { #(panic as "returned tuple function should not run") }
}
fn result_custom_never(_other: Int) -> fn() -> ValueBox(value) {
  fn() { ValueBox(panic as "returned custom function should not run") }
}

fn call_int(provider: fn() -> fn(Int) -> fn() -> Int) -> fn() -> Int {
  provider()(fail())
}
fn call_float(provider: fn() -> fn(Int) -> fn() -> Float) -> fn() -> Float {
  provider()(fail())
}
fn call_string(provider: fn() -> fn(Int) -> fn() -> String) -> fn() -> String {
  provider()(fail())
}
fn call_bit_array(provider: fn() -> fn(Int) -> fn() -> BitArray) -> fn() -> BitArray {
  provider()(fail())
}
fn call_utf_codepoint(provider: fn() -> fn(Int) -> fn() -> UtfCodepoint) -> fn() -> UtfCodepoint {
  provider()(fail())
}
fn call_custom(provider: fn() -> fn(Int) -> fn() -> Token) -> fn() -> Token {
  provider()(fail())
}
fn call_bool(provider: fn() -> fn(Int) -> fn() -> Bool) -> fn() -> Bool {
  provider()(fail())
}
fn call_nil(provider: fn() -> fn(Int) -> fn() -> Nil) -> fn() -> Nil {
  provider()(fail())
}
fn call_tuple(provider: fn() -> fn(Int) -> fn() -> #(Int)) -> fn() -> #(Int) {
  provider()(fail())
}
fn call_list(provider: fn() -> fn(Int) -> fn() -> List(Int)) -> fn() -> List(Int) {
  provider()(fail())
}
fn call_function(provider: fn() -> fn(Int) -> fn() -> fn() -> Int) -> fn() -> fn() -> Int {
  provider()(fail())
}
fn call_generic(provider) {
  provider()(fail())
}
fn call_never(provider) {
  provider()(fail())
}
fn call_tuple_never(
  provider: fn() -> fn(Int) -> fn() -> #(value),
) -> fn() -> #(value) {
  provider()(fail())
}
fn call_custom_never(
  provider: fn() -> fn(Int) -> fn() -> ValueBox(value),
) -> fn() -> ValueBox(value) {
  provider()(fail())
}

pub fn main() {
  let callers = #(
    call_int,
    call_float,
    call_string,
    call_bit_array,
    call_utf_codepoint,
    call_custom,
    call_bool,
    call_nil,
    call_tuple,
    call_list,
    call_function,
    call_generic,
    call_never,
    call_tuple_never,
    call_custom_never,
  )
  case callers.0 == call_int {
    True -> call_int(fn() { result_int })
    False -> call_int(fn() { result_int })
  }
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic returned function argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_typed_diverging_returned_function_arguments.gleam:6:3]
//  5 | fn fail() -> value {
//  6 |   panic as "generic returned function argument failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^^^^
//    :                             `-- panic in main.fail
//  7 | }
//    `----
