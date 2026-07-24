fn select(functions: List(fn(Int) -> value)) -> fn(Int) -> value {
  let assert [function] = functions
  function
}

pub fn main() {
  select([])(1)
}

// @geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/functions/generic_never_function_callee_failure.gleam:2:3]
//  1 | fn select(functions: List(fn(Int) -> value)) -> fn(Int) -> value {
//  2 |   let assert [function] = functions
//    :   ^^^^^|^^^^ ^^^^^|^^^^
//    :        |          `-- pattern
//    :        `-- let assert in main.select
//  3 |   function
//    `----
//   help: failed value: List(fn(Int) -> Parameter(0))([])
