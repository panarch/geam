pub type Empty

pub type Phantom(value) {
  Phantom
}

pub type Boxed(value) {
  Boxed(value)
}

fn identity(value) {
  value
}

fn int_function(_value: Empty) {
  1
}

fn float_function(_value: Empty) {
  1.0
}

fn string_function(_value: Empty) {
  "one"
}

fn bit_array_function(_value: Empty) {
  <<1>>
}

fn utf_codepoint_function(_value: Empty) -> UtfCodepoint {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  codepoint
}

fn custom_function(_value: Empty) {
  Phantom
}

fn bool_function(_value: Empty) {
  True
}

fn nil_function(_value: Empty) {
  Nil
}

fn tuple_function(_value: Empty) {
  #(1)
}

fn list_function(_value: Empty) {
  []
}

fn function_function(_value: Empty) {
  identity
}

fn diverge_custom() -> Boxed(Empty) {
  panic as "must not run"
}

fn diverge_tuple() -> #(Empty) {
  panic as "must not run"
}

fn echo_int(function: fn(Empty) -> Int) {
  echo function
}

fn echo_float(function: fn(Empty) -> Float) {
  echo function
}

fn echo_string(function: fn(Empty) -> String) {
  echo function
}

fn echo_bit_array(function: fn(Empty) -> BitArray) {
  echo function
}

fn echo_utf_codepoint(function: fn(Empty) -> UtfCodepoint) {
  echo function
}

fn echo_custom(function: fn(Empty) -> Phantom(output)) {
  echo function
}

fn echo_bool(function: fn(Empty) -> Bool) {
  echo function
}

fn echo_nil(function: fn(Empty) -> Nil) {
  echo function
}

fn echo_tuple(function: fn(Empty) -> #(Int)) {
  echo function
}

fn echo_list(function: fn(Empty) -> List(output)) {
  echo function
}

fn echo_function(function: fn(Empty) -> fn(argument) -> output) {
  echo function
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>

  echo []
  echo [[]]
  echo ["one"]
  echo [<<1>>]
  echo [codepoint]
  echo [Phantom]
  echo [1.0]
  echo [True]
  echo [Nil]
  echo [#(1)]
  echo [[1]]
  echo [int_function]

  echo_int(int_function)
  echo_float(float_function)
  echo_string(string_function)
  echo_bit_array(bit_array_function)
  echo_utf_codepoint(utf_codepoint_function)
  echo_custom(custom_function)
  echo_bool(bool_function)
  echo_nil(nil_function)
  echo_tuple(tuple_function)
  echo_list(list_function)
  echo_function(function_function)
  echo diverge_custom
  echo diverge_tuple

  1
}

// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:115
// []
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:116
// [[]]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:117
// ["one"]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:118
// [<<1>>]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:119
// ['A']
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:120
// [Phantom]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:121
// [1.0]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:122
// [True]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:123
// [Nil]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:124
// [#(1)]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:125
// [[1]]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:126
// [//fn(a) { ... }]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:69
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:73
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:77
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:81
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:85
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:89
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:93
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:97
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:101
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:105
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:109
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:139
// //fn() { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:140
// //fn() { ... }
// @geam:expect Int(1)
