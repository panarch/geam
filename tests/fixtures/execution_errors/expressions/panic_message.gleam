pub fn main() {
  panic as "boom"
}

// geam:expect-error
// geam::panic
//
//   x panic: boom
//    ,-[tests/fixtures/execution_errors/expressions/panic_message.gleam:2:3]
//  1 | pub fn main() {
//  2 |   panic as "boom"
//    :   ^^^^^^^|^^^^^^^
//    :          `-- panic in main.main
//  3 | }
//    `----
