pub fn main() {
  let size = 16
  <<panic as "value failed":float-size(size)>>
}

// geam:expect-error
// geam::panic
//
//   x panic: value failed
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_dynamic_float_value_panic.gleam:3:5]
//  2 |   let size = 16
//  3 |   <<panic as "value failed":float-size(size)>>
//    :     ^^^^^^^^^^^|^^^^^^^^^^^
//    :                `-- panic in main.main
//  4 | }
//    `----
