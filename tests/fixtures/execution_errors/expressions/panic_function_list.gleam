pub fn main() -> List(fn() -> Int) {
  panic
}

// @geam:expect-error
// geam::panic
//
//   x panic: `panic` expression evaluated.
//    ,-[tests/fixtures/execution_errors/expressions/panic_function_list.gleam:2:3]
//  1 | pub fn main() -> List(fn() -> Int) {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.main
//  3 | }
//    `----
