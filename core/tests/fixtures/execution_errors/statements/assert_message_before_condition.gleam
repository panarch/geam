fn fail_message() -> String {
  panic as "message"
}

pub fn main() {
  assert True as fail_message()
  1
}

// @geam:expect-error
// geam::panic
//
//   x panic: message
//    ,-[tests/fixtures/execution_errors/statements/assert_message_before_condition.gleam:2:3]
//  1 | fn fail_message() -> String {
//  2 |   panic as "message"
//    :   ^^^^^^^^^|^^^^^^^^
//    :            `-- panic in main.fail_message
//  3 | }
//    `----
