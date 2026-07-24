fn fail() -> fn(Int) -> value {
  let assert 1 = 2
  fn(_value) { panic }
}

pub fn main() {
  [fail()]
}

// @geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/functions/generic_never_function_list_element_failure.gleam:2:3]
//  1 | fn fail() -> fn(Int) -> value {
//  2 |   let assert 1 = 2
//    :   ^^^^^|^^^^ |
//    :        |     `-- pattern
//    :        `-- let assert in main.fail
//  3 |   fn(_value) { panic }
//    `----
//   help: failed value: Int(2)
