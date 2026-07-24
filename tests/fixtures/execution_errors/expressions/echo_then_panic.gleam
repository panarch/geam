pub fn main() {
  echo 1 as "before"
  panic as "after"
}

// @geam:echo
// tests/fixtures/execution_errors/expressions/echo_then_panic.gleam:2 before
// 1
// @geam:expect-error
// geam::panic
//
//   x panic: after
//    ,-[tests/fixtures/execution_errors/expressions/echo_then_panic.gleam:3:3]
//  2 |   echo 1 as "before"
//  3 |   panic as "after"
//    :   ^^^^^^^^|^^^^^^^
//    :           `-- panic in main.main
//  4 | }
//    `----
