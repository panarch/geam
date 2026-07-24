pub fn main() {
  assert { panic as "condition" } as "checked"
  1
}

// @geam:expect-error
// geam::panic
//
//   x panic: condition
//    ,-[tests/fixtures/execution_errors/statements/assert_condition_error_after_message.gleam:2:12]
//  1 | pub fn main() {
//  2 |   assert { panic as "condition" } as "checked"
//    :            ^^^^^^^^^^|^^^^^^^^^
//    :                      `-- panic in main.main
//  3 |   1
//    `----
