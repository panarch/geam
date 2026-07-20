fn consume(_function: fn(value) -> value) {
  Nil
}

fn fail() -> fn(value) -> value {
  panic as "symbolic function argument failed"
}

pub fn main() {
  consume(fail())
}

// geam:expect-error
// geam::panic
//
//   x panic: symbolic function argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_symbolic_function_argument_failure.gleam:6:3]
//  5 | fn fail() -> fn(value) -> value {
//  6 |   panic as "symbolic function argument failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^
//    :                         `-- panic in main.fail
//  7 | }
//    `----
