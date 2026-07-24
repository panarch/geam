pub fn main() {
  let value = todo
  value + 1
}

// @geam:expect-error
// geam::todo
//
//   x todo: `todo` expression evaluated. This code has not yet been implemented.
//    ,-[tests/fixtures/execution_errors/expressions/todo_assignment.gleam:2:15]
//  1 | pub fn main() {
//  2 |   let value = todo
//    :               ^^|^
//    :                 `-- todo in main.main
//  3 |   value + 1
//    `----
