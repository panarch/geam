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
  )
  int_value()
}


// @geam:explain
// module main
// main int#0
//
// function never#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail never#0 args=[]
//
// function int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.never#0:shape#1(fn() -> param#0) = function[Never] reference never#0
//     %function.float#0:shape#3(fn() -> Float) = function[Float] reference float#0
//     %function.string#0:shape#5(fn() -> String) = function[String] reference string#0
//     %function.bit_array#0:shape#7(fn() -> BitArray) = function[BitArray] reference bit_array#0
//     %function.utf_codepoint#0:shape#9(fn() -> UtfCodepoint) = function[UtfCodepoint] reference utf_codepoint#0
//     %function.custom#0:shape#11(fn() -> custom_type#0) = function[Custom] reference custom#0
//     %function.bool#0:shape#13(fn() -> Bool) = function[Bool] reference bool#0
//     %function.nil#0:shape#15(fn() -> Nil) = function[Nil] reference nil#0
//     %function.tuple#0:shape#18(fn() -> #(Int)) = function[Tuple] reference tuple#0
//     %tuple#0:shape#19(#(fn() -> param#0, fn() -> Float, fn() -> String, fn() -> BitArray, fn() -> UtfCodepoint, fn() -> custom_type#0, fn() -> Bool, fn() -> Nil, fn() -> #(Int))) = tuple.value elements=[%function.never#0, %function.float#0, %function.string#0, %function.bit_array#0, %function.utf_codepoint#0, %function.custom#0, %function.bool#0, %function.nil#0, %function.tuple#0]
//     tail int#1 args=[]
//
// function int#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail int#1 args=[]
//
// function float#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail float#0 args=[]
//
// function string#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail string#0 args=[]
//
// function bit_array#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail bit_array#0 args=[]
//
// function utf_codepoint#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail utf_codepoint#0 args=[]
//
// function custom#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail custom#0 args=[]
//
// function bool#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail bool#0 args=[]
//
// function nil#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail nil#0 args=[]
//
// function tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail tuple#0 args=[]
