fn fail() -> Bool {
  panic as "subject"
}

pub fn main() -> UtfCodepoint {
  case fail() {
    True -> panic
    False -> panic
  }
}

// geam:expect-error
// geam::panic
//
//   x panic: subject
//    ,-[tests/fixtures/execution_errors/functions/utf_codepoint_return_bool_case_subject.gleam:2:3]
//  1 | fn fail() -> Bool {
//  2 |   panic as "subject"
//    :   ^^^^^^^^^|^^^^^^^^
//    :            `-- panic in main.fail
//  3 | }
//    `----
