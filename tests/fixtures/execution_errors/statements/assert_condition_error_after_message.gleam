fn fail_condition() -> Bool {
  panic as "condition"
}

pub fn main() {
  assert fail_condition() as "checked"
  1
}

// geam:expect-error
// geam::panic
//
//   x panic: condition
//    ,-[tests/fixtures/execution_errors/statements/assert_condition_error_after_message.gleam:2:3]
//  1 | fn fail_condition() -> Bool {
//  2 |   panic as "condition"
//    :   ^^^^^^^^^^|^^^^^^^^^
//    :             `-- panic in main.fail_condition
//  3 | }
//    `----
