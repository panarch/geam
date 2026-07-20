fn first(values: List(value)) -> value {
  let assert [value, ..] = values as "expected a generic item"
  value
}

pub fn main() {
  first([])
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: expected a generic item
//    ,-[tests/fixtures/execution_errors/functions/generic_never_let_assert.gleam:2:3]
//  1 | fn first(values: List(value)) -> value {
//  2 |   let assert [value, ..] = values as "expected a generic item"
//    :   ^^^^^|^^^^ ^^^^^|^^^^^
//    :        |          `-- pattern
//    :        `-- let assert in main.first
//  3 |   value
//    `----
//   help: failed value: List(Parameter(0))([])
