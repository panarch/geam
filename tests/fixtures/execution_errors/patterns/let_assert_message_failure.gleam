fn fail_message() -> String {
  panic as "message failed"
}

pub fn main() {
  let assert 1 = 2 as fail_message()
  0
}

// geam:expect-error
// geam::panic
//
//   x panic: message failed
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_message_failure.gleam:2:3]
//  1 | fn fail_message() -> String {
//  2 |   panic as "message failed"
//    :   ^^^^^^^^^^^^|^^^^^^^^^^^^
//    :               `-- panic in main.fail_message
//  3 | }
//    `----
