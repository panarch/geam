fn stop(_value: Int) -> Int {
  panic as "after"
}

pub fn main() {
  1
  |> echo as "before"
  |> stop
}

// @geam:echo
// tests/fixtures/execution_errors/pipeline/echo_then_panic.gleam:7 before
// 1
// @geam:expect-error
// geam::panic
//
//   x panic: after
//    ,-[tests/fixtures/execution_errors/pipeline/echo_then_panic.gleam:2:3]
//  1 | fn stop(_value: Int) -> Int {
//  2 |   panic as "after"
//    :   ^^^^^^^^|^^^^^^^
//    :           `-- panic in main.stop
//  3 | }
//    `----
