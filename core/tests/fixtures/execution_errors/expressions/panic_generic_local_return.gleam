pub fn main() -> value {
  let value = panic as "generic local failed"
  value
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic local failed
//    ,-[tests/fixtures/execution_errors/expressions/panic_generic_local_return.gleam:2:15]
//  1 | pub fn main() -> value {
//  2 |   let value = panic as "generic local failed"
//    :               ^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^
//    :                              `-- panic in main.main
//  3 |   value
//    `----
