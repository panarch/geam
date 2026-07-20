fn unfinished() -> value {
  todo as "generic work remains"
}

pub fn main() {
  unfinished()
}

// geam:expect-error
// geam::todo
//
//   x todo: generic work remains
//    ,-[tests/fixtures/execution_errors/functions/generic_never_todo.gleam:2:3]
//  1 | fn unfinished() -> value {
//  2 |   todo as "generic work remains"
//    :   ^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^
//    :                  `-- todo in main.unfinished
//  3 | }
//    `----
