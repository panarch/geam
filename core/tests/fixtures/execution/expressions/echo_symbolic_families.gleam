pub type Phantom(value) {
  Phantom
}

pub type Boxed(value) {
  Boxed(value)
}

fn identity(value) {
  value
}

fn int_function(_value) {
  1
}

fn float_function(_value) {
  1.0
}

fn string_function(_value) {
  "one"
}

fn bit_array_function(_value) {
  <<1>>
}

fn utf_codepoint_function(_value) -> UtfCodepoint {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  codepoint
}

fn custom_function(_value) {
  Phantom
}

fn bool_function(_value) {
  True
}

fn nil_function(_value) {
  Nil
}

fn tuple_function(_value) {
  #(1)
}

fn list_function(_value) {
  []
}

fn function_function(_value) {
  identity
}

fn diverge_custom() -> Boxed(value) {
  panic as "must not run"
}

fn diverge_tuple() -> #(value) {
  panic as "must not run"
}

fn echo_int(function: fn(input) -> Int) {
  echo function
}

fn echo_float(function: fn(input) -> Float) {
  echo function
}

fn echo_string(function: fn(input) -> String) {
  echo function
}

fn echo_bit_array(function: fn(input) -> BitArray) {
  echo function
}

fn echo_utf_codepoint(function: fn(input) -> UtfCodepoint) {
  echo function
}

fn echo_custom(function: fn(input) -> Phantom(output)) {
  echo function
}

fn echo_bool(function: fn(input) -> Bool) {
  echo function
}

fn echo_nil(function: fn(input) -> Nil) {
  echo function
}

fn echo_tuple(function: fn(input) -> #(Int)) {
  echo function
}

fn echo_list(function: fn(input) -> List(output)) {
  echo function
}

fn echo_function(function: fn(input) -> fn(argument) -> output) {
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
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:113
// []
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:114
// [[]]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:115
// ["one"]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:116
// [<<1>>]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:117
// ['A']
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:118
// [Phantom]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:119
// [1.0]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:120
// [True]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:121
// [Nil]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:122
// [#(1)]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:123
// [[1]]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:124
// [//fn(a) { ... }]
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:67
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:71
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:75
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:79
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:83
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:87
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:91
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:95
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:99
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:103
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:107
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:137
// //fn() { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_symbolic_families.gleam:138
// //fn() { ... }
// @geam:expect Int(1)
