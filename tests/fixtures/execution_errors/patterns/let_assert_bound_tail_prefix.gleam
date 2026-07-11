pub fn main() {
  let assert [[first], ..rest] = [[1, 2]]
  #(first, rest)
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_bound_tail_prefix.gleam:2:3]
//  1 | pub fn main() {
//  2 |   let assert [[first], ..rest] = [[1, 2]]
//    :   ^^^^^|^^^^ ^^^^^^^^|^^^^^^^^
//    :        |             `-- pattern
//    :        `-- let assert in main.main
//  3 |   #(first, rest)
//    `----
//   help: failed value: List(List(Int))([List(Int)([Int(1), Int(2)])])
