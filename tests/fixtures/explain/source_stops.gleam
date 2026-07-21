fn empty_function() {}

fn empty_block() {
  {}
}

fn with_value(continue: fn(Int) -> Int) {
  continue(1)
}

fn incomplete_use() -> Int {
  use value <- with_value
}

fn stopped_with_todo() -> Int {
  todo
}

fn stopped_with_assert() -> Nil {
  assert False
}

pub fn main() {
  let _ = #(empty_function, empty_block, incomplete_use, stopped_with_todo, stopped_with_assert)
  0
}


// geam:run
// geam:explain
// module main
// main int#0
//
// function never#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     source_stop kind=empty_function message=none
//
// function never#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     source_stop kind=empty_block message=none
//
// function int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.never#0:shape#1(fn() -> param#0) = function[Never] reference never#0
//     %function.never#1:shape#3(fn() -> param#1) = function[Never] reference never#1
//     %function.int#0:shape#5(fn() -> Int) = function[Int] reference int#1
//     %function.int#1:shape#5(fn() -> Int) = function[Int] reference int#2
//     %function.nil#0:shape#7(fn() -> Nil) = function[Nil] reference nil#0
//     %tuple#0:shape#8(#(fn() -> param#0, fn() -> param#1, fn() -> Int, fn() -> Int, fn() -> Nil)) = tuple.value elements=[%function.never#0, %function.never#1, %function.int#0, %function.int#1, %function.nil#0]
//     %int#0:shape#4(Int) = int.value 0
//     return %int#0
//
// function int#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.int#0:shape#9(fn(Int) -> Int) = function[Int] closure target=int#3 captures=[]
//     tail int#4 args=[%function.int#0]
//
// function int#2
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     source_stop kind=todo message=none
//
// function int#3
//   entry b0 params=[%int#0:shape#4(Int)] captures=[]
//   block b0 params=[%int#0:shape#4(Int)]
//     source_stop kind=incomplete_use message=none
//
// function int#4
//   entry b0 params=[%function.int#0:shape#9(fn(Int) -> Int)] captures=[]
//   block b0 params=[%function.int#0:shape#9(fn(Int) -> Int)]
//     %int#0:shape#4(Int) = int.value 1
//     %int#1:shape#4(Int) = int.function_call %function.int#0 args=[%int#0]
//     return %int#1
//
// function nil#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     source_stop kind=assert message=none
