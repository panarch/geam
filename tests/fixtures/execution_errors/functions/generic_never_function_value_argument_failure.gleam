fn consume(_function: fn(Int) -> value) {
  Nil
}

fn fail() -> fn(Int) -> value {
  panic as "never function value argument failed"
}

pub fn main() {
  consume(fail())
}

// @geam:expect-error
// geam::panic
//
//   x panic: never function value argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_never_function_value_argument_failure.gleam:6:3]
//  5 | fn fail() -> fn(Int) -> value {
//  6 |   panic as "never function value argument failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^^
//    :                          `-- panic in main.fail
//  7 | }
//    `----
