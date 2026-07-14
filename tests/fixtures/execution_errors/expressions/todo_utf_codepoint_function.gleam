pub fn main() -> fn() -> UtfCodepoint {
  todo
}

// geam:expect-error
// geam::todo
//
//   x todo: `todo` expression evaluated. This code has not yet been implemented.
//    ,-[tests/fixtures/execution_errors/expressions/todo_utf_codepoint_function.gleam:2:3]
//  1 | pub fn main() -> fn() -> UtfCodepoint {
//  2 |   todo
//    :   ^^|^
//    :     `-- todo in main.main
//  3 | }
//    `----
