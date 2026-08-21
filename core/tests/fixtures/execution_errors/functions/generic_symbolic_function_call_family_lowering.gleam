pub type Token {
  Token
}

fn fail() -> value {
  panic as "symbolic function argument failed"
}

fn identity(value: value) {
  value
}

fn make_int(_trigger) -> fn(value) -> Int { fn(_value) { 1 } }
fn make_float(_trigger) -> fn(value) -> Float { fn(_value) { 1.5 } }
fn make_string(_trigger) -> fn(value) -> String { fn(_value) { "value" } }
fn make_bit_array(_trigger) -> fn(value) -> BitArray { fn(_value) { <<1>> } }
fn make_utf_codepoint(_trigger) -> fn(value) -> UtfCodepoint {
  fn(_value) {
    let assert <<codepoint:utf8_codepoint>> = <<65>>
    codepoint
  }
}
fn make_custom(_trigger) -> fn(value) -> Token { fn(_value) { Token } }
fn make_bool(_trigger) -> fn(value) -> Bool { fn(_value) { True } }
fn make_nil(_trigger) -> fn(value) -> Nil { fn(_value) { Nil } }
fn make_tuple(_trigger) -> fn(value) -> #(Int) { fn(_value) { #(1) } }
fn make_list(_trigger) -> fn(value) -> List(Int) { fn(_value) { [1] } }
fn make_function(_trigger) -> fn(value) -> fn(Int) -> Int {
  fn(_value) { fn(value) { value } }
}
fn make_generic(_trigger) -> fn(value) -> value { fn(value) { value } }

fn call_int() { let make = make_int make(fail()) }
fn call_float() { let make = make_float make(fail()) }
fn call_string() { let make = make_string make(fail()) }
fn call_bit_array() { let make = make_bit_array make(fail()) }
fn call_utf_codepoint() { let make = make_utf_codepoint make(fail()) }
fn call_custom() { let make = make_custom make(fail()) }
fn call_bool() { let make = make_bool make(fail()) }
fn call_nil() { let make = make_nil make(fail()) }
fn call_tuple() { let make = make_tuple make(fail()) }
fn call_list() { let make = make_list make(fail()) }
fn call_function() { let make = make_function make(fail()) }
fn call_generic() { let make = make_generic make(fail()) }

pub fn main() {
  case 0 {
    0 -> call_int() == call_int()
    1 -> call_float() == call_float()
    2 -> call_string() == call_string()
    3 -> call_bit_array() == call_bit_array()
    4 -> call_utf_codepoint() == call_utf_codepoint()
    5 -> call_custom() == call_custom()
    6 -> call_bool() == call_bool()
    7 -> call_nil() == call_nil()
    8 -> call_tuple() == call_tuple()
    9 -> call_list() == call_list()
    10 -> call_function() == call_function()
    _ -> call_generic() == call_generic()
  }
}

// @geam:expect-error
// geam::panic
//
//   x panic: symbolic function argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_symbolic_function_call_family_lowering.gleam:6:3]
//  5 | fn fail() -> value {
//  6 |   panic as "symbolic function argument failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^
//    :                         `-- panic in main.fail
//  7 | }
//    `----
