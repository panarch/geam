pub type Boxed {
  Boxed(Int)
}

fn stop() -> value { stop() }
fn int_value() -> Int { int_value() }
fn float_value() -> Float { float_value() }
fn string_value() -> String { string_value() }
fn bit_array_value() -> BitArray { bit_array_value() }
fn utf_codepoint_value() -> UtfCodepoint { utf_codepoint_value() }
fn custom_value() -> Boxed { custom_value() }
fn bool_value() -> Bool { bool_value() }
fn nil_value() -> Nil { nil_value() }
fn tuple_value() -> #(Int) { tuple_value() }

fn parameter_list() -> List(value) { parameter_list() }
fn int_list() -> List(Int) { int_list() }
fn string_list() -> List(String) { string_list() }
fn bit_array_list() -> List(BitArray) { bit_array_list() }
fn utf_codepoint_list() -> List(UtfCodepoint) { utf_codepoint_list() }
fn custom_list() -> List(Boxed) { custom_list() }
fn float_list() -> List(Float) { float_list() }
fn bool_list() -> List(Bool) { bool_list() }
fn nil_list() -> List(Nil) { nil_list() }
fn tuple_list() -> List(#(Int)) { tuple_list() }
fn parameter_list_list() -> List(List(value)) { parameter_list_list() }
fn list_list() -> List(List(Int)) { list_list() }
fn function_list() -> List(fn() -> Int) { function_list() }

fn identity(value: value) -> value { identity(value) }
fn int_function() -> fn() -> Int { int_function() }
fn float_function() -> fn() -> Float { float_function() }
fn string_function() -> fn() -> String { string_function() }
fn bit_array_function() -> fn() -> BitArray { bit_array_function() }
fn utf_codepoint_function() -> fn() -> UtfCodepoint { utf_codepoint_function() }
fn custom_function() -> fn() -> Boxed { custom_function() }
fn bool_function() -> fn() -> Bool { bool_function() }
fn nil_function() -> fn() -> Nil { nil_function() }
fn tuple_function() -> fn() -> #(Int) { tuple_function() }
fn generic_function() -> fn(value) -> value { generic_function() }
fn never_function() -> fn() -> value { never_function() }
fn parameter_list_function() -> fn() -> List(value) { parameter_list_function() }
fn int_list_function() -> fn() -> List(Int) { int_list_function() }
fn string_list_function() -> fn() -> List(String) { string_list_function() }
fn bit_array_list_function() -> fn() -> List(BitArray) { bit_array_list_function() }
fn utf_codepoint_list_function() -> fn() -> List(UtfCodepoint) { utf_codepoint_list_function() }
fn custom_list_function() -> fn() -> List(Boxed) { custom_list_function() }
fn float_list_function() -> fn() -> List(Float) { float_list_function() }
fn bool_list_function() -> fn() -> List(Bool) { bool_list_function() }
fn nil_list_function() -> fn() -> List(Nil) { nil_list_function() }
fn tuple_list_function() -> fn() -> List(#(Int)) { tuple_list_function() }
fn parameter_list_list_function() -> fn() -> List(List(value)) {
  parameter_list_list_function()
}
fn list_list_function() -> fn() -> List(List(Int)) { list_list_function() }
fn function_list_function() -> fn() -> List(fn() -> Int) { function_list_function() }
fn function_function() -> fn() -> fn() -> Int { function_function() }

pub fn main() {
  let _ = #(
    stop,
    float_value,
    string_value,
    bit_array_value,
    utf_codepoint_value,
    custom_value,
    bool_value,
    nil_value,
    tuple_value,
    parameter_list,
    int_list,
    string_list,
    bit_array_list,
    utf_codepoint_list,
    custom_list,
    float_list,
    bool_list,
    nil_list,
    tuple_list,
    parameter_list_list,
    list_list,
    function_list,
    int_function,
    float_function,
    string_function,
    bit_array_function,
    utf_codepoint_function,
    custom_function,
    bool_function,
    nil_function,
    tuple_function,
    generic_function,
    never_function,
    parameter_list_function,
    int_list_function,
    string_list_function,
    bit_array_list_function,
    utf_codepoint_list_function,
    custom_list_function,
    float_list_function,
    bool_list_function,
    nil_list_function,
    tuple_list_function,
    parameter_list_list_function,
    list_list_function,
    function_list_function,
    function_function,
  )
  int_value()
}
