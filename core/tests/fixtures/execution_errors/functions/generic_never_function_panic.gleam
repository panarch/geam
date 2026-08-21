fn fail() -> fn(Int) -> value {
  panic as "generic never function failed"
}

pub fn main() {
  fail()
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic never function failed
//    ,-[tests/fixtures/execution_errors/functions/generic_never_function_panic.gleam:2:3]
//  1 | fn fail() -> fn(Int) -> value {
//  2 |   panic as "generic never function failed"
//    :   ^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^
//    :                       `-- panic in main.fail
//  3 | }
//    `----
