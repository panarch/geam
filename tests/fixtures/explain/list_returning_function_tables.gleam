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
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.parameter#0 args=[]
//
// function function.list.parameter_list#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.parameter_list#0 args=[]
//
// function function.list.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.function#0:shape#38(fn() -> fn() -> list_type#1) = function[Function] reference function.list.parameter#0
//     %function.function#1:shape#39(fn() -> fn() -> list_type#3) = function[Function] reference function.list.parameter_list#0
//     %function.function#2:shape#40(fn() -> fn() -> list_type#4) = function[Function] reference function.list.string#0
//     %function.function#3:shape#41(fn() -> fn() -> list_type#5) = function[Function] reference function.list.bit_array#0
//     %function.function#4:shape#42(fn() -> fn() -> list_type#6) = function[Function] reference function.list.utf_codepoint#0
//     %function.function#5:shape#43(fn() -> fn() -> list_type#7) = function[Function] reference function.list.custom#0
//     %function.function#6:shape#44(fn() -> fn() -> list_type#8) = function[Function] reference function.list.float#0
//     %function.function#7:shape#45(fn() -> fn() -> list_type#9) = function[Function] reference function.list.bool#0
//     %function.function#8:shape#46(fn() -> fn() -> list_type#10) = function[Function] reference function.list.nil#0
//     %function.function#9:shape#47(fn() -> fn() -> list_type#11) = function[Function] reference function.list.tuple#0
//     %function.function#10:shape#48(fn() -> fn() -> list_type#12) = function[Function] reference function.list.list#0
//     %function.function#11:shape#49(fn() -> fn() -> list_type#13) = function[Function] reference function.list.function#0
//     %tuple#0:shape#50(#(fn() -> fn() -> list_type#1, fn() -> fn() -> list_type#3, fn() -> fn() -> list_type#4, fn() -> fn() -> list_type#5, fn() -> fn() -> list_type#6, fn() -> fn() -> list_type#7, fn() -> fn() -> list_type#8, fn() -> fn() -> list_type#9, fn() -> fn() -> list_type#10, fn() -> fn() -> list_type#11, fn() -> fn() -> list_type#12, fn() -> fn() -> list_type#13)) = tuple.value elements=[%function.function#0, %function.function#1, %function.function#2, %function.function#3, %function.function#4, %function.function#5, %function.function#6, %function.function#7, %function.function#8, %function.function#9, %function.function#10, %function.function#11]
//     tail function.list.int#1 args=[]
//
// function function.list.int#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.int#1 args=[]
//
// function function.list.string#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.string#0 args=[]
//
// function function.list.bit_array#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.bit_array#0 args=[]
//
// function function.list.utf_codepoint#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.utf_codepoint#0 args=[]
//
// function function.list.custom#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.custom#0 args=[]
//
// function function.list.float#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.float#0 args=[]
//
// function function.list.bool#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.bool#0 args=[]
//
// function function.list.nil#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.nil#0 args=[]
//
// function function.list.tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.tuple#0 args=[]
//
// function function.list.list#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.list#0 args=[]
//
// function function.list.function#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.function#0 args=[]
