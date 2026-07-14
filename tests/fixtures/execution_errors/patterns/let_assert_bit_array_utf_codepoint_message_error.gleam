fn fail_message() -> String {
  panic as "message"
}

pub fn main() {
  let assert <<_:utf8_codepoint>> = <<255>> as fail_message()
  0
}

// geam:expect-error
// geam::panic
//
//   x panic: message
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_bit_array_utf_codepoint_message_error.gleam:2:3]
//  1 | fn fail_message() -> String {
//  2 |   panic as "message"
//    :   ^^^^^^^^^|^^^^^^^^
//    :            `-- panic in main.fail_message
//  3 | }
//    `----
