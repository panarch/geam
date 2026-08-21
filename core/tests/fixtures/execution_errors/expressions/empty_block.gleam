pub fn main() {
  {}
}

// @geam:expect-error
// geam::empty_block
//
//   x empty_block: Block is empty.
//    ,-[tests/fixtures/execution_errors/expressions/empty_block.gleam:2:3]
//  1 | pub fn main() {
//  2 |   {}
//    :   ^|
//    :    `-- empty block in main.main
//  3 | }
//    `----
