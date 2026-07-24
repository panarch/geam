pub type Token {
  Token
}

fn fail() -> value {
  panic as "generic function argument failed"
}

fn result_int(_other: Int) -> Int {
  1
}

fn result_float(_other: Int) -> Float {
  1.5
}

fn result_string(_other: Int) -> String {
  "value"
}

fn result_bit_array(_other: Int) -> BitArray {
  <<1>>
}

fn result_utf_codepoint(_other: Int) -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn result_custom(_other: Int) -> Token {
  Token
}

fn result_bool(_other: Int) -> Bool {
  True
}

fn result_nil(_other: Int) -> Nil {
  Nil
}

fn result_tuple(_other: Int) -> #(Int) {
  #(1)
}

fn result_list(_other: Int) -> List(Int) {
  [1]
}

fn result_function(_other: Int) -> fn() -> Int {
  fn() { 1 }
}

fn call_int() -> Int {
  let function = result_int
  function(fail())
}

fn call_float() -> Float {
  let function = result_float
  function(fail())
}

fn call_string() -> String {
  let function = result_string
  function(fail())
}

fn call_bit_array() -> BitArray {
  let function = result_bit_array
  function(fail())
}

fn call_utf_codepoint() -> UtfCodepoint {
  let function = result_utf_codepoint
  function(fail())
}

fn call_custom() -> Token {
  let function = result_custom
  function(fail())
}

fn call_bool() -> Bool {
  let function = result_bool
  function(fail())
}

fn call_nil() -> Nil {
  let function = result_nil
  function(fail())
}

fn call_tuple() -> #(Int) {
  let function = result_tuple
  function(fail())
}

fn call_list() -> List(Int) {
  let function = result_list
  function(fail())
}

fn call_function() -> fn() -> Int {
  let function = result_function
  function(fail())
}

pub fn main() {
  call_float()
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic function argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_typed_diverging_function_arguments.gleam:6:3]
//  5 | fn fail() -> value {
//  6 |   panic as "generic function argument failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^
//    :                        `-- panic in main.fail
//  7 | }
//    `----
