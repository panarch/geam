fn first_float() -> Float { 1.0 }
fn second_float() -> Float { 2.0 }

fn select_float_function(value: Int) -> fn() -> Float {
  {
    case value {
      0 -> first_float
      _ -> second_float
    }
  }
}

fn string_value() -> String { "value" }

fn select_string_function() -> fn() -> String {
  {
    let selected = string_value
    selected
  }
}

fn first_list() -> List(Int) { [] }
fn second_list() -> List(Int) { [] }

fn select_list_function(value: Int) -> fn() -> List(Int) {
  {
    case value {
      0 -> first_list
      _ -> second_list
    }
  }
}

fn first_tuple() -> #(Int) { #(1) }
fn second_tuple() -> #(Int) { #(2) }

fn select_tuple_function(value: Int) -> fn() -> #(Int) {
  {
    case value {
      0 -> first_tuple
      _ -> second_tuple
    }
  }
}

pub fn main() -> fn() -> List(Int) {
  let _ = #(
    select_float_function(0),
    select_string_function(),
    select_tuple_function(0),
  )
  select_list_function(0)
}


// geam:explain
// module main
// main function.list.int#0
//
// function float#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %float#0:shape#1(Float) = float.value 1.0
//     return %float#0
//
// function float#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %float#0:shape#1(Float) = float.value 2.0
//     return %float#0
//
// function string#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %string#0:shape#3(String) = string.value "value"
//     return %string#0
//
// function tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %tuple#0:shape#5(#(Int)) = tuple.value elements=[%int#0]
//     return %tuple#0
//
// function tuple#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 2
//     %tuple#0:shape#5(#(Int)) = tuple.value elements=[%int#0]
//     return %tuple#0
//
// function list.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.int#0:shape#8(list_type#0) = list.int[type#0] value elements=[]
//     return %list.int#0
//
// function list.int#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.int#0:shape#8(list_type#0) = list.int[type#0] value elements=[]
//     return %list.int#0
//
// function function.float#0
//   entry b0 params=[%int#0:shape#0(Int)] captures=[]
//   block b0 params=[%int#0:shape#0(Int)]
//     switch.int %int#0 clauses=[0->b1()] fallback=b2()
//   block b1 params=[]
//     %function.float#0:shape#2(fn() -> Float) = function[Float] reference float#0
//     return %function.float#0
//   block b2 params=[]
//     %function.float#0:shape#2(fn() -> Float) = function[Float] reference float#1
//     return %function.float#0
//
// function function.string#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.string#0:shape#4(fn() -> String) = function[String] reference string#0
//     return %function.string#0
//
// function function.tuple#0
//   entry b0 params=[%int#0:shape#0(Int)] captures=[]
//   block b0 params=[%int#0:shape#0(Int)]
//     switch.int %int#0 clauses=[0->b1()] fallback=b2()
//   block b1 params=[]
//     %function.tuple#0:shape#6(fn() -> #(Int)) = function[Tuple] reference tuple#0
//     return %function.tuple#0
//   block b2 params=[]
//     %function.tuple#0:shape#6(fn() -> #(Int)) = function[Tuple] reference tuple#1
//     return %function.tuple#0
//
// function function.list.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 0
//     %function.float#0:shape#2(fn() -> Float) = function[Float] call function.float#0 args=[%int#0]
//     %function.string#0:shape#4(fn() -> String) = function[String] call function.string#0 args=[]
//     %int#1:shape#0(Int) = int.value 0
//     %function.tuple#0:shape#6(fn() -> #(Int)) = function[Tuple] call function.tuple#0 args=[%int#1]
//     %tuple#0:shape#7(#(fn() -> Float, fn() -> String, fn() -> #(Int))) = tuple.value elements=[%function.float#0, %function.string#0, %function.tuple#0]
//     %int#2:shape#0(Int) = int.value 0
//     tail function.list.int#1 args=[%int#2]
//
// function function.list.int#1
//   entry b0 params=[%int#0:shape#0(Int)] captures=[]
//   block b0 params=[%int#0:shape#0(Int)]
//     switch.int %int#0 clauses=[0->b1()] fallback=b2()
//   block b1 params=[]
//     %function.list.int#0:shape#9(fn() -> list_type#0) = function[List] reference list.int#0
//     return %function.list.int#0
//   block b2 params=[]
//     %function.list.int#0:shape#9(fn() -> list_type#0) = function[List] reference list.int#1
//     return %function.list.int#0
