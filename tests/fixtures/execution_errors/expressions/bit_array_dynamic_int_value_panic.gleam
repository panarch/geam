pub fn main() {
  let size = 8
  <<panic as "value failed":int-size(size)>>
}

// geam:expect-error
// geam::panic
//
//   x panic: value failed
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_dynamic_int_value_panic.gleam:3:5]
//  2 |   let size = 8
//  3 |   <<panic as "value failed":int-size(size)>>
//    :     ^^^^^^^^^^^|^^^^^^^^^^^
//    :                `-- panic in main.main
//  4 | }
//    `----
