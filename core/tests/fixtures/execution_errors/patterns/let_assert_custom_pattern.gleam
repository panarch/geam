pub type Choice {
  Empty
  Full(Int)
}

pub fn main() {
  let assert Full(value) = Empty as "expected full"
  value
}

// @geam:expect-error
// geam::let_assert
//
//   x let_assert: expected full
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_custom_pattern.gleam:7:3]
//  6 | pub fn main() {
//  7 |   let assert Full(value) = Empty as "expected full"
//    :   ^^^^^|^^^^ ^^^^^|^^^^^
//    :        |          `-- pattern
//    :        `-- let assert in main.main
//  8 |   value
//    `----
//   help: failed value: geam/main/Choice::Empty()
