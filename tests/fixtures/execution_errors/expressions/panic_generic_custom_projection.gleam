pub type Boxed(value) {
  Boxed(value: value)
}

fn boxed_value() -> Boxed(value) {
  Boxed(value: panic as "generic custom projection failed")
}

pub fn main() -> value {
  boxed_value().value
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic custom projection failed
//    ,-[tests/fixtures/execution_errors/expressions/panic_generic_custom_projection.gleam:6:16]
//  5 | fn boxed_value() -> Boxed(value) {
//  6 |   Boxed(value: panic as "generic custom projection failed")
//    :                ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^
//    :                                     `-- panic in main.boxed_value
//  7 | }
//    `----
