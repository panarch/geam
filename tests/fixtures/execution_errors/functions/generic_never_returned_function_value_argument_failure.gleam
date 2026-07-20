fn invalid_int(values: List(Int)) -> Int {
  let assert [value] = values
  value
}

fn fail(_value: Int) -> value {
  panic as "callee must not run"
}

fn provide(_value: Int) -> fn(Int) -> value {
  fail
}

pub fn main() {
  let provider = provide
  provider(invalid_int([]))
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/functions/generic_never_returned_function_value_argument_failure.gleam:2:3]
//  1 | fn invalid_int(values: List(Int)) -> Int {
//  2 |   let assert [value] = values
//    :   ^^^^^|^^^^ ^^^|^^^
//    :        |        `-- pattern
//    :        `-- let assert in main.invalid_int
//  3 |   value
//    `----
//   help: failed value: List(Int)([])
