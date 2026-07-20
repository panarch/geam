pub type Boxed(value) {
  Boxed(value)
}

pub fn main() -> Boxed(value) {
  Boxed(panic as "generic custom field failed")
}

// geam:expect-error
// geam::panic
//
//   x panic: generic custom field failed
//    ,-[tests/fixtures/execution_errors/expressions/panic_generic_custom_field.gleam:6:9]
//  5 | pub fn main() -> Boxed(value) {
//  6 |   Boxed(panic as "generic custom field failed")
//    :         ^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^
//    :                            `-- panic in main.main
//  7 | }
//    `----
