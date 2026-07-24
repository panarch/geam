fn values(_: Int) -> List(UtfCodepoint) {
  []
}

pub fn main() {
  values(panic as "argument")
}

// @geam:expect-error
// geam::panic
//
//   x panic: argument
//    ,-[tests/fixtures/execution_errors/functions/utf_codepoint_list_call_argument.gleam:6:10]
//  5 | pub fn main() {
//  6 |   values(panic as "argument")
//    :          ^^^^^^^^^|^^^^^^^^^
//    :                   `-- panic in main.main
//  7 | }
//    `----
