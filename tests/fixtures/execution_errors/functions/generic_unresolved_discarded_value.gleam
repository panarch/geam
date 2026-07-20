fn fail() -> value {
  panic as "discarded generic value failed"
}

pub fn main() {
  let _ = fail()
  1
}

// geam:expect-error
// geam::panic
//
//   x panic: discarded generic value failed
//    ,-[tests/fixtures/execution_errors/functions/generic_unresolved_discarded_value.gleam:2:3]
//  1 | fn fail() -> value {
//  2 |   panic as "discarded generic value failed"
//    :   ^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^
//    :                       `-- panic in main.fail
//  3 | }
//    `----
