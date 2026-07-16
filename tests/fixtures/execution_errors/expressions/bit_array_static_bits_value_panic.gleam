pub fn main() {
  <<panic as "value failed":bits>>
}

// geam:expect-error
// geam::panic
//
//   x panic: value failed
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_static_bits_value_panic.gleam:2:5]
//  1 | pub fn main() {
//  2 |   <<panic as "value failed":bits>>
//    :     ^^^^^^^^^^^|^^^^^^^^^^^
//    :                `-- panic in main.main
//  3 | }
//    `----
