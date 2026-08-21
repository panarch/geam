pub fn main() -> fn(value) -> value {
  panic
}

// @geam:expect-error
// geam::panic
//
//   x panic: `panic` expression evaluated.
//    ,-[tests/fixtures/execution_errors/expressions/panic_generic_function.gleam:2:3]
//  1 | pub fn main() -> fn(value) -> value {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.main
//  3 | }
//    `----
