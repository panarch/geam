pub fn main() {
  let assert [first, ..] = [] as "not empty"
  first + 1
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: not empty
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_message.gleam:2:3]
//  1 | pub fn main() {
//  2 |   let assert [first, ..] = [] as "not empty"
//    :   ^^^^^|^^^^ ^^^^^|^^^^^
//    :        |          `-- pattern
//    :        `-- let assert in main.main
//  3 |   first + 1
//    `----
//   help: failed value: List(Int)([])
