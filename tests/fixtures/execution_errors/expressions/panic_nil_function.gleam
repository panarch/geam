pub fn main() -> fn() -> Nil {
  panic
}

// @geam:expect-error
// geam::panic
//
//   x panic: `panic` expression evaluated.
//    ,-[tests/fixtures/execution_errors/expressions/panic_nil_function.gleam:2:3]
//  1 | pub fn main() -> fn() -> Nil {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.main
//  3 | }
//    `----
