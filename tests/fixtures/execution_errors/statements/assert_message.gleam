pub fn main() {
  assert False as "nope"
  1
}

// geam:expect-error
// geam::assert
//
//   x assert: nope
//    ,-[tests/fixtures/execution_errors/statements/assert_message.gleam:2:3]
//  1 | pub fn main() {
//  2 |   assert False as "nope"
//    :   ^^^^^^^^^^^|^^^^^^^^^^
//    :              `-- assert in main.main
//  3 |   1
//    `----
