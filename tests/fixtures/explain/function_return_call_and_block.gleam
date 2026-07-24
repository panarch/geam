fn bit_array_value() -> BitArray { <<>> }

fn bit_array_block() -> fn() -> BitArray {
  {
    let selected = bit_array_value
    selected
  }
}

fn bit_array_tail() -> fn() -> BitArray { bit_array_tail() }

fn utf_codepoint_value(value: UtfCodepoint) -> UtfCodepoint { value }

fn utf_codepoint_block() -> fn(UtfCodepoint) -> UtfCodepoint {
  {
    let selected = utf_codepoint_value
    selected
  }
}

fn utf_codepoint_tail() -> fn(UtfCodepoint) -> UtfCodepoint {
  utf_codepoint_tail()
}

fn float_tail() -> fn() -> Float { float_tail() }

fn bool_value() -> Bool { True }

fn bool_block() -> fn() -> Bool {
  {
    let selected = bool_value
    selected
  }
}

fn nil_value() -> Nil { Nil }

pub fn main() -> fn() -> Nil {
  let _ = #(
    bit_array_block(),
    bit_array_tail,
    utf_codepoint_block(),
    utf_codepoint_tail,
    float_tail,
    bool_block(),
  )
  {
    let selected = nil_value
    selected
  }
}


// @geam:explain
// module main
// main function.nil#0
//
// function bit_array#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %bit_array#0:shape#0(BitArray) = bit_array.value []
//     return %bit_array#0
//
// function utf_codepoint#0
//   entry b0 params=[%utf_codepoint#0:shape#2(UtfCodepoint)] captures=[]
//   block b0 params=[%utf_codepoint#0:shape#2(UtfCodepoint)]
//     return %utf_codepoint#0
//
// function bool#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %bool#0:shape#9(Bool) = bool.value True
//     return %bool#0
//
// function nil#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %nil#0:shape#12(Nil) = nil.value
//     return %nil#0
//
// function function.float#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.float#0 args=[]
//
// function function.bit_array#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.bit_array#0:shape#1(fn() -> BitArray) = function[BitArray] reference bit_array#0
//     return %function.bit_array#0
//
// function function.bit_array#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.bit_array#1 args=[]
//
// function function.utf_codepoint#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.utf_codepoint#0:shape#3(fn(UtfCodepoint) -> UtfCodepoint) = function[UtfCodepoint] reference utf_codepoint#0
//     return %function.utf_codepoint#0
//
// function function.utf_codepoint#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.utf_codepoint#1 args=[]
//
// function function.bool#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.bool#0:shape#10(fn() -> Bool) = function[Bool] reference bool#0
//     return %function.bool#0
//
// function function.nil#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.bit_array#0:shape#1(fn() -> BitArray) = function[BitArray] call function.bit_array#0 args=[]
//     %function.function#0:shape#6(fn() -> fn() -> BitArray) = function[Function] reference function.bit_array#1
//     %function.utf_codepoint#0:shape#3(fn(UtfCodepoint) -> UtfCodepoint) = function[UtfCodepoint] call function.utf_codepoint#0 args=[]
//     %function.function#1:shape#7(fn() -> fn(UtfCodepoint) -> UtfCodepoint) = function[Function] reference function.utf_codepoint#1
//     %function.function#2:shape#8(fn() -> fn() -> Float) = function[Function] reference function.float#0
//     %function.bool#0:shape#10(fn() -> Bool) = function[Bool] call function.bool#0 args=[]
//     %tuple#0:shape#11(#(fn() -> BitArray, fn() -> fn() -> BitArray, fn(UtfCodepoint) -> UtfCodepoint, fn() -> fn(UtfCodepoint) -> UtfCodepoint, fn() -> fn() -> Float, fn() -> Bool)) = tuple.value elements=[%function.bit_array#0, %function.function#0, %function.utf_codepoint#0, %function.function#1, %function.function#2, %function.bool#0]
//     %function.nil#0:shape#13(fn() -> Nil) = function[Nil] reference nil#0
//     return %function.nil#0
