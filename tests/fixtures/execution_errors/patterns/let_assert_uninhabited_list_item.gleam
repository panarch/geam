pub fn main() {
  let assert [_value, ..] = []
  0
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_uninhabited_list_item.gleam:2:3]
//  1 | pub fn main() {
//  2 |   let assert [_value, ..] = []
//    :   ^^^^^|^^^^ ^^^^^^|^^^^^
//    :        |           `-- pattern
//    :        `-- let assert in main.main
//  3 |   0
//    `----
//   help: failed value: List(Parameter(0))([])
