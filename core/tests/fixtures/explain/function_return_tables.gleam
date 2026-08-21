pub type Boxed {
  Boxed(Int)
}

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
fn function_function() -> fn() -> fn() -> Int { function_function() }

pub fn main() -> fn() -> Int {
  let _ = #(
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
    function_function,
  )
  int_function()
}


// @geam:explain
// module main
// main function.int#0
//
// function function.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.function#0:shape#23(fn() -> fn() -> Float) = function[Function] reference function.float#0
//     %function.function#1:shape#24(fn() -> fn() -> String) = function[Function] reference function.string#0
//     %function.function#2:shape#25(fn() -> fn() -> BitArray) = function[Function] reference function.bit_array#0
//     %function.function#3:shape#26(fn() -> fn() -> UtfCodepoint) = function[Function] reference function.utf_codepoint#0
//     %function.function#4:shape#27(fn() -> fn() -> custom_type#0) = function[Function] reference function.custom#0
//     %function.function#5:shape#28(fn() -> fn() -> Bool) = function[Function] reference function.bool#0
//     %function.function#6:shape#29(fn() -> fn() -> Nil) = function[Function] reference function.nil#0
//     %function.function#7:shape#30(fn() -> fn() -> #(Int)) = function[Function] reference function.tuple#0
//     %function.function#8:shape#31(fn() -> fn(param#0) -> param#0) = function[Function] reference function.generic#0
//     %function.function#9:shape#32(fn() -> fn() -> param#1) = function[Function] reference function.never#0
//     %function.function#10:shape#33(fn() -> fn() -> fn() -> Int) = function[Function] reference function.function#0
//     %tuple#0:shape#34(#(fn() -> fn() -> Float, fn() -> fn() -> String, fn() -> fn() -> BitArray, fn() -> fn() -> UtfCodepoint, fn() -> fn() -> custom_type#0, fn() -> fn() -> Bool, fn() -> fn() -> Nil, fn() -> fn() -> #(Int), fn() -> fn(param#0) -> param#0, fn() -> fn() -> param#1, fn() -> fn() -> fn() -> Int)) = tuple.value elements=[%function.function#0, %function.function#1, %function.function#2, %function.function#3, %function.function#4, %function.function#5, %function.function#6, %function.function#7, %function.function#8, %function.function#9, %function.function#10]
//     tail function.int#1 args=[]
//
// function function.int#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.int#1 args=[]
//
// function function.float#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.float#0 args=[]
//
// function function.string#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.string#0 args=[]
//
// function function.bit_array#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.bit_array#0 args=[]
//
// function function.utf_codepoint#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.utf_codepoint#0 args=[]
//
// function function.custom#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.custom#0 args=[]
//
// function function.bool#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.bool#0 args=[]
//
// function function.nil#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.nil#0 args=[]
//
// function function.tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.tuple#0 args=[]
//
// function function.generic#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.generic#0 args=[]
//
// function function.never#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.never#0 args=[]
//
// function function.function#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.function#0 args=[]
