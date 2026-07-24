pub fn main() {
  let assert [[first], ..] = [[1, 2]]
  first + 1
}

// @geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_nested_prefix.gleam:2:3]
//  1 | pub fn main() {
//  2 |   let assert [[first], ..] = [[1, 2]]
//    :   ^^^^^|^^^^ ^^^^^^|^^^^^^
//    :        |           `-- pattern
//    :        `-- let assert in main.main
//  3 |   first + 1
//    `----
//   help: failed value: List(List(Int))([List(Int)([Int(1), Int(2)])])
