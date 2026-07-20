pub type Boxed {
  Boxed(Int)
}

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

pub fn main() -> List(Int) {
  let _ = #(
    parameter_list,
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
  )
  int_list()
}

// geam:explain
// module main
// main list.int#0
//
// function list.parameter#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.parameter#0 args=0
//
// function list.int#0
//   entry steps=1
//   graph entry=b0
//   b0 tail list.int#1 args=0
//
// function list.int#1
//   entry steps=0
//   graph entry=b0
//   b0 tail list.int#1 args=0
//
// function list.string#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.string#0 args=0
//
// function list.bit_array#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.bit_array#0 args=0
//
// function list.utf_codepoint#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.utf_codepoint#0 args=0
//
// function list.custom#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.custom#0 args=0
//
// function list.float#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.float#0 args=0
//
// function list.bool#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.bool#0 args=0
//
// function list.nil#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.nil#0 args=0
//
// function list.tuple#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.tuple#0 args=0
//
// function list.parameter_list#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.parameter_list#0 args=0
//
// function list.list#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.list#0 args=0
//
// function list.function#0
//   entry steps=0
//   graph entry=b0
//   b0 tail list.function#0 args=0
