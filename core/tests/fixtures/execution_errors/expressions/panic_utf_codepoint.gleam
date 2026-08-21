pub fn main() -> UtfCodepoint {
  panic
}

// @geam:expect-error
// geam::panic
//
//   x panic: `panic` expression evaluated.
//    ,-[tests/fixtures/execution_errors/expressions/panic_utf_codepoint.gleam:2:3]
//  1 | pub fn main() -> UtfCodepoint {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.main
//  3 | }
//    `----
