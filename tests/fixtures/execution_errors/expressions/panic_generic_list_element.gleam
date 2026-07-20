pub fn main() -> List(value) {
  [panic as "generic list element failed"]
}

// geam:expect-error
// geam::panic
//
//   x panic: generic list element failed
//    ,-[tests/fixtures/execution_errors/expressions/panic_generic_list_element.gleam:2:4]
//  1 | pub fn main() -> List(value) {
//  2 |   [panic as "generic list element failed"]
//    :    ^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^
//    :                       `-- panic in main.main
//  3 | }
//    `----
