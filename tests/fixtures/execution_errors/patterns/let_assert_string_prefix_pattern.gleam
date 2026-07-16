pub fn main() {
  let assert "pre" <> rest = "other"
  rest
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_string_prefix_pattern.gleam:2:3]
//  1 | pub fn main() {
//  2 |   let assert "pre" <> rest = "other"
//    :   ^^^^^|^^^^ ^^^^^^|^^^^^^
//    :        |           `-- pattern
//    :        `-- let assert in main.main
//  3 |   rest
//    `----
//   help: failed value: String("other")
