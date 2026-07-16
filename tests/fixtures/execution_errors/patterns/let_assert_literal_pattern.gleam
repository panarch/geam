pub fn main() {
  let assert 1 = 2
  0
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_literal_pattern.gleam:2:3]
//  1 | pub fn main() {
//  2 |   let assert 1 = 2
//    :   ^^^^^|^^^^ |
//    :        |     `-- pattern
//    :        `-- let assert in main.main
//  3 |   0
//    `----
//   help: failed value: Int(2)
