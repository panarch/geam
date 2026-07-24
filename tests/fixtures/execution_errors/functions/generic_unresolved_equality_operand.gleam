fn fail() -> value {
  panic as "generic equality operand failed"
}

pub fn main() {
  fail() == fail()
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic equality operand failed
//    ,-[tests/fixtures/execution_errors/functions/generic_unresolved_equality_operand.gleam:2:3]
//  1 | fn fail() -> value {
//  2 |   panic as "generic equality operand failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^
//    :                        `-- panic in main.fail
//  3 | }
//    `----
