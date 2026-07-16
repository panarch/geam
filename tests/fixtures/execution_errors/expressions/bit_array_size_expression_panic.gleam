pub fn main() {
  let bits = <<1>>
  <<bits:bits-size(panic as "size failed")>>
}

// geam:expect-error
// geam::panic
//
//   x panic: size failed
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_size_expression_panic.gleam:3:20]
//  2 |   let bits = <<1>>
//  3 |   <<bits:bits-size(panic as "size failed")>>
//    :                    ^^^^^^^^^^^|^^^^^^^^^^
//    :                               `-- panic in main.main
//  4 | }
//    `----
