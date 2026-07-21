pub type Boxed {
  Boxed(Int)
}

fn parameter_list_function() -> fn() -> List(value) { parameter_list_function() }
fn parameter_list_list_function() -> fn() -> List(List(value)) {
  parameter_list_list_function()
}
fn int_list_function() -> fn() -> List(Int) { int_list_function() }
fn string_list_function() -> fn() -> List(String) { string_list_function() }
fn bit_array_list_function() -> fn() -> List(BitArray) { bit_array_list_function() }
fn utf_codepoint_list_function() -> fn() -> List(UtfCodepoint) {
  utf_codepoint_list_function()
}
fn custom_list_function() -> fn() -> List(Boxed) { custom_list_function() }
fn float_list_function() -> fn() -> List(Float) { float_list_function() }
fn bool_list_function() -> fn() -> List(Bool) { bool_list_function() }
fn nil_list_function() -> fn() -> List(Nil) { nil_list_function() }
fn tuple_list_function() -> fn() -> List(#(Int)) { tuple_list_function() }
fn list_list_function() -> fn() -> List(List(Int)) { list_list_function() }
fn function_list_function() -> fn() -> List(fn() -> Int) { function_list_function() }

pub fn main() -> fn() -> List(Int) {
  let _ = #(
    parameter_list_function,
    parameter_list_list_function,
    string_list_function,
    bit_array_list_function,
    utf_codepoint_list_function,
    custom_list_function,
    float_list_function,
    bool_list_function,
    nil_list_function,
    tuple_list_function,
    list_list_function,
    function_list_function,
  )
  int_list_function()
}

// geam:explain
// module main
// main function.list.int#0
//
// function function.list.parameter#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.parameter#0 args=0
//
// function function.list.parameter_list#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.parameter_list#0 args=0
//
// function function.list.int#0
//   graph entry=b0
//   b0 instructions=13 tail function.list.int#1 args=0
//
// function function.list.int#1
//   graph entry=b0
//   b0 instructions=0 tail function.list.int#1 args=0
//
// function function.list.string#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.string#0 args=0
//
// function function.list.bit_array#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.bit_array#0 args=0
//
// function function.list.utf_codepoint#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.utf_codepoint#0 args=0
//
// function function.list.custom#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.custom#0 args=0
//
// function function.list.float#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.float#0 args=0
//
// function function.list.bool#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.bool#0 args=0
//
// function function.list.nil#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.nil#0 args=0
//
// function function.list.tuple#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.tuple#0 args=0
//
// function function.list.list#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.list#0 args=0
//
// function function.list.function#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.function#0 args=0
