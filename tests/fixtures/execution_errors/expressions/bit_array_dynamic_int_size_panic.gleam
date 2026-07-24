pub fn main() {
  <<1:size(panic as "size failed")>>
}

// @geam:expect-error
// geam::panic
//
//   x panic: size failed
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_dynamic_int_size_panic.gleam:2:12]
//  1 | pub fn main() {
//  2 |   <<1:size(panic as "size failed")>>
//    :            ^^^^^^^^^^^|^^^^^^^^^^
//    :                       `-- panic in main.main
//  3 | }
//    `----
