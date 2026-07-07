pub fn main() {
  assert False
  1
}

// geam:expect-error
// geam::assert
//
//   x assert: Assertion failed.
//    ,-[tests/fixtures/execution_errors/statements/assert_statement.gleam:2:3]
//  1 | pub fn main() {
//  2 |   assert False
//    :   ^^^^^^|^^^^^
//    :         `-- assert in main.main
//  3 |   1
//    `----
