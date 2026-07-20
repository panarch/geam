fn fail() -> #(fn(Int) -> value) {
  panic as "never function binding failed"
}

pub fn main() {
  let function = fail().0
  function
}

// geam:expect-error
// geam::panic
//
//   x panic: never function binding failed
//    ,-[tests/fixtures/execution_errors/functions/generic_never_function_binding_projection_failure.gleam:2:3]
//  1 | fn fail() -> #(fn(Int) -> value) {
//  2 |   panic as "never function binding failed"
//    :   ^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^
//    :                       `-- panic in main.fail
//  3 | }
//    `----
