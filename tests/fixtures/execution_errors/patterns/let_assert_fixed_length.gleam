pub fn main() {
  let assert [first, second] = [1]
  first + second
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_fixed_length.gleam:2:3]
//  1 | pub fn main() {
//  2 |   let assert [first, second] = [1]
//    :   ^^^^^|^^^^ ^^^^^^^|^^^^^^^
//    :        |            `-- pattern
//    :        `-- let assert in main.main
//  3 |   first + second
//    `----
//   help: failed value: List(Int)([Int(1)])
