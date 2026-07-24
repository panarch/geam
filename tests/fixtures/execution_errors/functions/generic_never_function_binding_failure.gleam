fn fail(functions: List(fn(Int) -> value)) -> fn(Int) -> value {
  let assert [function] = functions
  function
}

pub fn main() {
  let function = fail([])
  function == function
}

// @geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/functions/generic_never_function_binding_failure.gleam:2:3]
//  1 | fn fail(functions: List(fn(Int) -> value)) -> fn(Int) -> value {
//  2 |   let assert [function] = functions
//    :   ^^^^^|^^^^ ^^^^^|^^^^
//    :        |          `-- pattern
//    :        `-- let assert in main.fail
//  3 |   function
//    `----
//   help: failed value: List(fn(Int) -> Parameter(0))([])
