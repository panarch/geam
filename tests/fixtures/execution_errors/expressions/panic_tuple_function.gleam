pub fn main() -> fn() -> #(Int, String) {
  panic
}

// @geam:expect-error
// geam::panic
//
//   x panic: `panic` expression evaluated.
//    ,-[tests/fixtures/execution_errors/expressions/panic_tuple_function.gleam:2:3]
//  1 | pub fn main() -> fn() -> #(Int, String) {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.main
//  3 | }
//    `----
