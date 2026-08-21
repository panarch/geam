pub fn main() -> List(UtfCodepoint) {
  todo
}

// @geam:expect-error
// geam::todo
//
//   x todo: `todo` expression evaluated. This code has not yet been implemented.
//    ,-[tests/fixtures/execution_errors/expressions/todo_list_utf_codepoint.gleam:2:3]
//  1 | pub fn main() -> List(UtfCodepoint) {
//  2 |   todo
//    :   ^^|^
//    :     `-- todo in main.main
//  3 | }
//    `----
