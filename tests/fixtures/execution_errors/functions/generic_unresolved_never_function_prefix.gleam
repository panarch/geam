fn fail_value() -> value {
  panic as "generic argument failed"
}

fn diverge(_value: Int) -> value {
  panic as "diverging function must not run"
}

fn stop(_function: fn(Int) -> value, _other: other) -> result {
  panic as "callee must not run"
}

pub fn main() {
  stop(diverge, fail_value())
}

// geam:expect-error
// geam::panic
//
//   x panic: generic argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_unresolved_never_function_prefix.gleam:2:3]
//  1 | fn fail_value() -> value {
//  2 |   panic as "generic argument failed"
//    :   ^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^
//    :                    `-- panic in main.fail_value
//  3 | }
//    `----
