fn int_value() -> Int { int_value() }
fn int_list() -> List(Int) { int_list() }
fn int_function() -> fn() -> Int { int_function() }
fn int_list_function() -> fn() -> List(Int) { int_list_function() }
fn function_function() -> fn() -> fn() -> Int { function_function() }

pub fn main() {
  let _ = #(
    int_list,
    int_function,
    int_list_function,
    function_function,
  )
  int_value()
}


// @geam:explain
// module main
// main int#0
//
// function int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.list.int#0:shape#3(fn() -> list_type#0) = function[List] reference list.int#0
//     %function.function#0:shape#4(fn() -> fn() -> Int) = function[Function] reference function.int#0
//     %function.function#1:shape#5(fn() -> fn() -> list_type#0) = function[Function] reference function.list.int#0
//     %function.function#2:shape#6(fn() -> fn() -> fn() -> Int) = function[Function] reference function.function#0
//     %tuple#0:shape#7(#(fn() -> list_type#0, fn() -> fn() -> Int, fn() -> fn() -> list_type#0, fn() -> fn() -> fn() -> Int)) = tuple.value elements=[%function.list.int#0, %function.function#0, %function.function#1, %function.function#2]
//     tail int#1 args=[]
//
// function int#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail int#1 args=[]
//
// function list.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.int#0 args=[]
//
// function function.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.int#0 args=[]
//
// function function.list.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.list.int#0 args=[]
//
// function function.function#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail function.function#0 args=[]
