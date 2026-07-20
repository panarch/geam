pub type Token {
  Token
}

fn fail() -> value {
  panic as "generic function argument failed"
}

fn invoke(function: fn(argument) -> result) -> result {
  function(fail())
}

fn identity(value: value) {
  value
}

fn diverge(_value: Int) -> value {
  panic as "diverging function must not run"
}

fn result_int(_value) { 1 }
fn result_float(_value) { 1.5 }
fn result_string(_value) { "value" }
fn result_bit_array(_value) { <<1>> }
fn result_utf_codepoint(_value) -> UtfCodepoint {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  codepoint
}
fn result_custom(_value) { Token }
fn result_bool(_value) { True }
fn result_nil(_value) { Nil }
fn result_tuple(_value) { #(1) }
fn result_list(_value) { [1] }
fn result_string_list(_value) { ["value"] }
fn result_bit_array_list(_value) { [<<1>>] }
fn result_utf_codepoint_list(_value) -> List(UtfCodepoint) {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  [codepoint]
}
fn result_custom_list(_value) { [Token] }
fn result_float_list(_value) { [1.5] }
fn result_bool_list(_value) { [True] }
fn result_nil_list(_value) { [Nil] }
fn result_tuple_list(_value) { [#(1)] }
fn result_nested_list(_value) { [[1]] }
fn result_function_list(_value) -> List(fn(Int) -> Int) { [fn(value) { value }] }
fn result_int_function(_value) -> fn(Int) -> Int { fn(value) { value } }
fn result_float_function(_value) -> fn(Int) -> Float { fn(_value) { 1.5 } }
fn result_string_function(_value) -> fn(Int) -> String { fn(_value) { "value" } }
fn result_bit_array_function(_value) -> fn(Int) -> BitArray { fn(_value) { <<1>> } }
fn result_utf_codepoint_function(_value) -> fn(Int) -> UtfCodepoint {
  fn(_value) {
    let assert <<codepoint:utf8_codepoint>> = <<65>>
    codepoint
  }
}
fn result_custom_function(_value) -> fn(Int) -> Token { fn(_value) { Token } }
fn result_bool_function(_value) -> fn(Int) -> Bool { fn(_value) { True } }
fn result_nil_function(_value) -> fn(Int) -> Nil { fn(_value) { Nil } }
fn result_tuple_function(_value) -> fn(Int) -> #(Int) { fn(_value) { #(1) } }
fn result_list_function(_value) -> fn(Int) -> List(Int) { fn(_value) { [1] } }
fn result_function_function(_value) -> fn(Int) -> fn(Int) -> Int {
  fn(_value) { fn(value) { value } }
}
fn result_generic_function(_value) { identity }
fn result_never_function(_value) { diverge }

fn call_int() { invoke(result_int) }
fn call_float() { invoke(result_float) }
fn call_string() { invoke(result_string) }
fn call_bit_array() { invoke(result_bit_array) }
fn call_utf_codepoint() { invoke(result_utf_codepoint) }
fn call_custom() { invoke(result_custom) }
fn call_bool() { invoke(result_bool) }
fn call_nil() { invoke(result_nil) }
fn call_tuple() { invoke(result_tuple) }
fn call_list() { invoke(result_list) }
fn call_string_list() { invoke(result_string_list) }
fn call_bit_array_list() { invoke(result_bit_array_list) }
fn call_utf_codepoint_list() { invoke(result_utf_codepoint_list) }
fn call_custom_list() { invoke(result_custom_list) }
fn call_float_list() { invoke(result_float_list) }
fn call_bool_list() { invoke(result_bool_list) }
fn call_nil_list() { invoke(result_nil_list) }
fn call_tuple_list() { invoke(result_tuple_list) }
fn call_nested_list() { invoke(result_nested_list) }
fn call_function_list() { invoke(result_function_list) }
fn call_int_function() { invoke(result_int_function) }
fn call_float_function() { invoke(result_float_function) }
fn call_string_function() { invoke(result_string_function) }
fn call_bit_array_function() { invoke(result_bit_array_function) }
fn call_utf_codepoint_function() { invoke(result_utf_codepoint_function) }
fn call_custom_function() { invoke(result_custom_function) }
fn call_bool_function() { invoke(result_bool_function) }
fn call_nil_function() { invoke(result_nil_function) }
fn call_tuple_function() { invoke(result_tuple_function) }
fn call_list_function() { invoke(result_list_function) }
fn call_function_function() { invoke(result_function_function) }
fn call_generic_function() { invoke(result_generic_function) }
fn call_never_function() { invoke(result_never_function) }

pub fn main() {
  let _ = #(
    call_int == call_int,
    call_float == call_float,
    call_string == call_string,
    call_bit_array == call_bit_array,
    call_utf_codepoint == call_utf_codepoint,
    call_custom == call_custom,
    call_bool == call_bool,
    call_nil == call_nil,
    call_tuple == call_tuple,
    call_list == call_list,
    call_string_list == call_string_list,
    call_bit_array_list == call_bit_array_list,
    call_utf_codepoint_list == call_utf_codepoint_list,
    call_custom_list == call_custom_list,
    call_float_list == call_float_list,
    call_bool_list == call_bool_list,
    call_nil_list == call_nil_list,
    call_tuple_list == call_tuple_list,
    call_nested_list == call_nested_list,
    call_function_list == call_function_list,
    call_int_function == call_int_function,
    call_float_function == call_float_function,
    call_string_function == call_string_function,
    call_bit_array_function == call_bit_array_function,
    call_utf_codepoint_function == call_utf_codepoint_function,
    call_custom_function == call_custom_function,
    call_bool_function == call_bool_function,
    call_nil_function == call_nil_function,
    call_tuple_function == call_tuple_function,
    call_list_function == call_list_function,
    call_function_function == call_function_function,
    call_generic_function == call_generic_function,
    call_never_function == call_never_function,
  )
  call_int()
}

// geam:expect-error
// geam::panic
//
//   x panic: generic function argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_diverging_function_call_family_lowering.gleam:6:3]
//  5 | fn fail() -> value {
//  6 |   panic as "generic function argument failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^
//    :                        `-- panic in main.fail
//  7 | }
//    `----
