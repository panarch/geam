fn fail() -> #(UtfCodepoint) {
  panic as "subject"
}

pub fn main() -> UtfCodepoint {
  fail().0
}

// @geam:expect-error
// geam::panic
//
//   x panic: subject
//    ,-[tests/fixtures/execution_errors/functions/utf_codepoint_return_tuple_subject.gleam:2:3]
//  1 | fn fail() -> #(UtfCodepoint) {
//  2 |   panic as "subject"
//    :   ^^^^^^^^^|^^^^^^^^
//    :            `-- panic in main.fail
//  3 | }
//    `----
