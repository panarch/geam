pub fn main() -> fn() -> UtfCodepoint {
  panic
}

// geam:expect-error
// geam::panic
//
//   x panic: `panic` expression evaluated.
//    ,-[tests/fixtures/execution_errors/expressions/panic_utf_codepoint_function.gleam:2:3]
//  1 | pub fn main() -> fn() -> UtfCodepoint {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.main
//  3 | }
//    `----
