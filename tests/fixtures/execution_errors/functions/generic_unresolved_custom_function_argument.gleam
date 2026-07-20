fn fail() -> value {
  panic as "generic function argument failed"
}

pub type Token {
  Token
}

fn result(_other: other) -> Token {
  Token
}

pub fn main() {
  let function = result
  function(fail())
}

// geam:expect-error
// geam::panic
//
//   x panic: generic function argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_unresolved_custom_function_argument.gleam:2:3]
//  1 | fn fail() -> value {
//  2 |   panic as "generic function argument failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^
//    :                        `-- panic in main.fail
//  3 | }
//    `----
