fn fail() -> Int {
  panic as "subject"
}

pub fn main() -> UtfCodepoint {
  case fail() {
    1 -> panic
    _ -> panic
  }
}

// geam:expect-error
// geam::panic
//
//   x panic: subject
//    ,-[tests/fixtures/execution_errors/functions/utf_codepoint_return_int_case_subject.gleam:2:3]
//  1 | fn fail() -> Int {
//  2 |   panic as "subject"
//    :   ^^^^^^^^^|^^^^^^^^
//    :            `-- panic in main.fail
//  3 | }
//    `----
