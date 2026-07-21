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

// geam:explain
// module main
// main int#0
//
// function int#0
//   graph entry=b0
//   b0 instructions=5 tail int#1 args=0
//
// function int#1
//   graph entry=b0
//   b0 instructions=0 tail int#1 args=0
//
// function list.int#0
//   graph entry=b0
//   b0 instructions=0 tail list.int#0 args=0
//
// function function.int#0
//   graph entry=b0
//   b0 instructions=0 tail function.int#0 args=0
//
// function function.list.int#0
//   graph entry=b0
//   b0 instructions=0 tail function.list.int#0 args=0
//
// function function.function#0
//   graph entry=b0
//   b0 instructions=0 tail function.function#0 args=0
