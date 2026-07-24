pub fn main() {
  <<panic as "value failed":utf8>>
}

// @geam:expect-error
// geam::panic
//
//   x panic: value failed
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_static_string_value_panic.gleam:2:5]
//  1 | pub fn main() {
//  2 |   <<panic as "value failed":utf8>>
//    :     ^^^^^^^^^^^|^^^^^^^^^^^
//    :                `-- panic in main.main
//  3 | }
//    `----
