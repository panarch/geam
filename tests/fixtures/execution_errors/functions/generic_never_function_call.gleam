fn fail() -> value {
  panic as "generic function call failed"
}

fn invoke(function: fn() -> value) -> value {
  function()
}

pub fn main() {
  invoke(fail)
}

// geam:expect-error
// geam::panic
//
//   x panic: generic function call failed
//    ,-[tests/fixtures/execution_errors/functions/generic_never_function_call.gleam:2:3]
//  1 | fn fail() -> value {
//  2 |   panic as "generic function call failed"
//    :   ^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^
//    :                      `-- panic in main.fail
//  3 | }
//    `----
