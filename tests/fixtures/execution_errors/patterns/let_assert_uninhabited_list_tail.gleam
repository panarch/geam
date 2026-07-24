fn tail(values: List(value)) {
  let assert [_, ..tail] = values
  tail
}

pub fn main() {
  tail([])
}

// @geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_uninhabited_list_tail.gleam:2:3]
//  1 | fn tail(values: List(value)) {
//  2 |   let assert [_, ..tail] = values
//    :   ^^^^^|^^^^ ^^^^^|^^^^^
//    :        |          `-- pattern
//    :        `-- let assert in main.tail
//  3 |   tail
//    `----
//   help: failed value: List(Parameter(0))([])
