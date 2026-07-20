fn tuple_value() -> #(value) {
  #(panic as "generic tuple projection failed")
}

pub fn main() -> value {
  tuple_value().0
}

// geam:expect-error
// geam::panic
//
//   x panic: generic tuple projection failed
//    ,-[tests/fixtures/execution_errors/expressions/panic_generic_tuple_projection.gleam:2:5]
//  1 | fn tuple_value() -> #(value) {
//  2 |   #(panic as "generic tuple projection failed")
//    :     ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^
//    :                          `-- panic in main.tuple_value
//  3 | }
//    `----
