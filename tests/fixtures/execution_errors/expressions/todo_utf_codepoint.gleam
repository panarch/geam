pub fn main() -> UtfCodepoint {
  todo
}

// geam:expect-error
// geam::todo
//
//   x todo: `todo` expression evaluated. This code has not yet been implemented.
//    ,-[tests/fixtures/execution_errors/expressions/todo_utf_codepoint.gleam:2:3]
//  1 | pub fn main() -> UtfCodepoint {
//  2 |   todo
//    :   ^^|^
//    :     `-- todo in main.main
//  3 | }
//    `----
