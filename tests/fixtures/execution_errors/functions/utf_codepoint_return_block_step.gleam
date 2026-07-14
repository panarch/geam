fn fail() -> Nil {
  panic as "step"
}

pub fn main() -> UtfCodepoint {
  let _ = fail()
  panic
}

// geam:expect-error
// geam::panic
//
//   x panic: step
//    ,-[tests/fixtures/execution_errors/functions/utf_codepoint_return_block_step.gleam:2:3]
//  1 | fn fail() -> Nil {
//  2 |   panic as "step"
//    :   ^^^^^^^|^^^^^^^
//    :          `-- panic in main.fail
//  3 | }
//    `----
