pub fn main() {
  let assert <<_:utf8_codepoint>> = <<255>>
  0
}

// @geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_bit_array_utf_codepoint_default_message.gleam:2:3]
//  1 | pub fn main() {
//  2 |   let assert <<_:utf8_codepoint>> = <<255>>
//    :   ^^^^^|^^^^ ^^^^^^^^^^|^^^^^^^^^
//    :        |               `-- pattern
//    :        `-- let assert in main.main
//  3 |   0
//    `----
//   help: failed value: BitArray(bytes=[255], bit_len=8)
