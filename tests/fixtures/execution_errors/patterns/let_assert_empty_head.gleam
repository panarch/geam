pub fn main() {
  let assert [first, ..] = []
  first + 1
}

// @geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_empty_head.gleam:2:3]
//  1 | pub fn main() {
//  2 |   let assert [first, ..] = []
//    :   ^^^^^|^^^^ ^^^^^|^^^^^
//    :        |          `-- pattern
//    :        `-- let assert in main.main
//  3 |   first + 1
//    `----
//   help: failed value: List(Int)([])
