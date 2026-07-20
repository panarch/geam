fn invalid_int() -> Int {
  panic as "generic argument failed"
}

fn fail(_value: Int) -> value {
  panic as "callee must not run"
}

pub fn main() {
  let function = fail
  function(invalid_int())
}

// geam:expect-error
// geam::panic
//
//   x panic: generic argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_never_function_argument_failure.gleam:2:3]
//  1 | fn invalid_int() -> Int {
//  2 |   panic as "generic argument failed"
//    :   ^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^
//    :                    `-- panic in main.invalid_int
//  3 | }
//    `----
