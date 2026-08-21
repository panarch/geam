pub fn main() -> List(List(value)) {
  [panic as "nested generic list element failed"]
}

// @geam:expect-error
// geam::panic
//
//   x panic: nested generic list element failed
//    ,-[tests/fixtures/execution_errors/expressions/panic_nested_generic_list_element.gleam:2:4]
//  1 | pub fn main() -> List(List(value)) {
//  2 |   [panic as "nested generic list element failed"]
//    :    ^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^
//    :                          `-- panic in main.main
//  3 | }
//    `----
