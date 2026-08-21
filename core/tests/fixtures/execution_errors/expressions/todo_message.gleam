pub fn main() {
  todo as "later"
}

// @geam:expect-error
// geam::todo
//
//   x todo: later
//    ,-[tests/fixtures/execution_errors/expressions/todo_message.gleam:2:3]
//  1 | pub fn main() {
//  2 |   todo as "later"
//    :   ^^^^^^^|^^^^^^^
//    :          `-- todo in main.main
//  3 | }
//    `----
