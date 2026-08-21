pub fn main() {
  let assert True = False
  0
}

// @geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_bool_pattern.gleam:2:3]
//  1 | pub fn main() {
//  2 |   let assert True = False
//    :   ^^^^^|^^^^ ^^|^
//    :        |       `-- pattern
//    :        `-- let assert in main.main
//  3 |   0
//    `----
//   help: failed value: Bool(false)
