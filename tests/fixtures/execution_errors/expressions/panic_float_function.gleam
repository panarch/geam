pub fn main() -> fn() -> Float {
  panic
}

// geam:expect-error
// geam::panic
//
//   x panic: `panic` expression evaluated.
//    ,-[tests/fixtures/execution_errors/expressions/panic_float_function.gleam:2:3]
//  1 | pub fn main() -> fn() -> Float {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.main
//  3 | }
//    `----
