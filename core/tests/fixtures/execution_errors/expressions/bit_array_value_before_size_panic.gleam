pub fn main() {
  <<panic as "value failed":bits-size(panic as "size failed")>>
}

// @geam:expect-error
// geam::panic
//
//   x panic: value failed
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_value_before_size_panic.gleam:2:5]
//  1 | pub fn main() {
//  2 |   <<panic as "value failed":bits-size(panic as "size failed")>>
//    :     ^^^^^^^^^^^|^^^^^^^^^^^
//    :                `-- panic in main.main
//  3 | }
//    `----
