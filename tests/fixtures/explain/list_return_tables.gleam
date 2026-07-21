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
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.parameter#0 args=[]
//
// function list.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.list.parameter#0:shape#2(fn() -> list_type#1) = function[List] reference list.parameter#0
//     %function.list.string#0:shape#5(fn() -> list_type#2) = function[List] reference list.string#0
//     %function.list.bit_array#0:shape#8(fn() -> list_type#3) = function[List] reference list.bit_array#0
//     %function.list.utf_codepoint#0:shape#11(fn() -> list_type#4) = function[List] reference list.utf_codepoint#0
//     %function.list.custom#0:shape#14(fn() -> list_type#5) = function[List] reference list.custom#0
//     %function.list.float#0:shape#17(fn() -> list_type#6) = function[List] reference list.float#0
//     %function.list.bool#0:shape#20(fn() -> list_type#7) = function[List] reference list.bool#0
//     %function.list.nil#0:shape#23(fn() -> list_type#8) = function[List] reference list.nil#0
//     %function.list.tuple#0:shape#27(fn() -> list_type#9) = function[List] reference list.tuple#0
//     %function.list.parameter_list#0:shape#31(fn() -> list_type#11) = function[List] reference list.parameter_list#0
//     %function.list.list#0:shape#34(fn() -> list_type#12) = function[List] reference list.list#0
//     %function.list.function#0:shape#37(fn() -> list_type#13) = function[List] reference list.function#0
//     %tuple#0:shape#38(#(fn() -> list_type#1, fn() -> list_type#2, fn() -> list_type#3, fn() -> list_type#4, fn() -> list_type#5, fn() -> list_type#6, fn() -> list_type#7, fn() -> list_type#8, fn() -> list_type#9, fn() -> list_type#11, fn() -> list_type#12, fn() -> list_type#13)) = tuple.value elements=[%function.list.parameter#0, %function.list.string#0, %function.list.bit_array#0, %function.list.utf_codepoint#0, %function.list.custom#0, %function.list.float#0, %function.list.bool#0, %function.list.nil#0, %function.list.tuple#0, %function.list.parameter_list#0, %function.list.list#0, %function.list.function#0]
//     tail list.int#1 args=[]
//
// function list.int#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.int#1 args=[]
//
// function list.string#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.string#0 args=[]
//
// function list.bit_array#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.bit_array#0 args=[]
//
// function list.utf_codepoint#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.utf_codepoint#0 args=[]
//
// function list.custom#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.custom#0 args=[]
//
// function list.float#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.float#0 args=[]
//
// function list.bool#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.bool#0 args=[]
//
// function list.nil#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.nil#0 args=[]
//
// function list.tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.tuple#0 args=[]
//
// function list.parameter_list#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.parameter_list#0 args=[]
//
// function list.list#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.list#0 args=[]
//
// function list.function#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.function#0 args=[]
