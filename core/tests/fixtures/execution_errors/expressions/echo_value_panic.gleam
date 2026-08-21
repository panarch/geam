fn fail() -> Int {
  panic as "value failed"
}

pub fn main() {
  echo fail() as "never"
}

// @geam:expect-error
// geam::panic
//
//   x panic: value failed
//    ,-[tests/fixtures/execution_errors/expressions/echo_value_panic.gleam:2:3]
//  1 | fn fail() -> Int {
//  2 |   panic as "value failed"
//    :   ^^^^^^^^^^^|^^^^^^^^^^^
//    :              `-- panic in main.fail
//  3 | }
//    `----
