pub fn main() -> #(Int, String) {
  panic
}

// @geam:expect-error
// geam::panic
//
//   x panic: `panic` expression evaluated.
//    ,-[tests/fixtures/execution_errors/expressions/panic_tuple.gleam:2:3]
//  1 | pub fn main() -> #(Int, String) {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.main
//  3 | }
//    `----
