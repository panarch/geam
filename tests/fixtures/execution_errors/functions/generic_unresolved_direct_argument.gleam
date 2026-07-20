fn fail() -> value {
  panic as "generic direct argument failed"
}

fn result(_other: other) -> Int {
  1
}

pub fn main() {
  let value = result(fail())
  value
}

// geam:expect-error
// geam::panic
//
//   x panic: generic direct argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_unresolved_direct_argument.gleam:2:3]
//  1 | fn fail() -> value {
//  2 |   panic as "generic direct argument failed"
//    :   ^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^
//    :                       `-- panic in main.fail
//  3 | }
//    `----
