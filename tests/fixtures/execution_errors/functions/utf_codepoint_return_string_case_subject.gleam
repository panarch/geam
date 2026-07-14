fn fail() -> String {
  panic as "subject"
}

pub fn main() -> UtfCodepoint {
  case fail() {
    "match" -> panic
    _ -> panic
  }
}

// geam:expect-error
// geam::panic
//
//   x panic: subject
//    ,-[tests/fixtures/execution_errors/functions/utf_codepoint_return_string_case_subject.gleam:2:3]
//  1 | fn fail() -> String {
//  2 |   panic as "subject"
//    :   ^^^^^^^^^|^^^^^^^^
//    :            `-- panic in main.fail
//  3 | }
//    `----
