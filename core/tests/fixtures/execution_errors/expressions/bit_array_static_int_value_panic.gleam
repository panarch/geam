pub fn main() {
  <<panic as "value failed":int>>
}

// @geam:expect-error
// geam::panic
//
//   x panic: value failed
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_static_int_value_panic.gleam:2:5]
//  1 | pub fn main() {
//  2 |   <<panic as "value failed":int>>
//    :     ^^^^^^^^^^^|^^^^^^^^^^^
//    :                `-- panic in main.main
//  3 | }
//    `----
