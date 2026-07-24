fn fail() -> value {
  panic as "generic function argument failed"
}

fn first(value: Int, _other: other) {
  value
}

pub fn main() {
  let function = first
  function(1, fail())
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic function argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_unresolved_function_argument.gleam:2:3]
//  1 | fn fail() -> value {
//  2 |   panic as "generic function argument failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^
//    :                        `-- panic in main.fail
//  3 | }
//    `----
