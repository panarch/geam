pub fn main() {
  let value = panic as "boom"
  value + 1
}

// geam:expect-error
// geam::panic
//
//   x panic: boom
//    ,-[tests/fixtures/execution_errors/expressions/panic_assignment.gleam:2:15]
//  1 | pub fn main() {
//  2 |   let value = panic as "boom"
//    :               ^^^^^^^|^^^^^^^
//    :                      `-- panic in main.main
//  3 |   value + 1
//    `----
