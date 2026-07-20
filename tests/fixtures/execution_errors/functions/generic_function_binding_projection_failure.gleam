fn fail() -> #(fn(value) -> value) {
  panic as "generic function binding failed"
}

pub fn main() {
  let function = fail().0
  function
}

// geam:expect-error
// geam::panic
//
//   x panic: generic function binding failed
//    ,-[tests/fixtures/execution_errors/functions/generic_function_binding_projection_failure.gleam:2:3]
//  1 | fn fail() -> #(fn(value) -> value) {
//  2 |   panic as "generic function binding failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^
//    :                        `-- panic in main.fail
//  3 | }
//    `----
