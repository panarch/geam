fn fail() -> UtfCodepoint {
  panic as "step"
}

pub fn main() {
  let value = fail()
  let _ = value
  Nil
}

// @geam:expect-error
// geam::panic
//
//   x panic: step
//    ,-[tests/fixtures/execution_errors/functions/utf_codepoint_let_step.gleam:2:3]
//  1 | fn fail() -> UtfCodepoint {
//  2 |   panic as "step"
//    :   ^^^^^^^|^^^^^^^
//    :          `-- panic in main.fail
//  3 | }
//    `----
