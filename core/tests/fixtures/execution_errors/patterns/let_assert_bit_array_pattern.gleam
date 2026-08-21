pub fn main() {
  let assert <<1, rest:bits>> = <<2, 3>> as "wrong bits"
  rest
}

// @geam:expect-error
// geam::let_assert
//
//   x let_assert: wrong bits
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_bit_array_pattern.gleam:2:3]
//  1 | pub fn main() {
//  2 |   let assert <<1, rest:bits>> = <<2, 3>> as "wrong bits"
//    :   ^^^^^|^^^^ ^^^^^^^^|^^^^^^^
//    :        |             `-- pattern
//    :        `-- let assert in main.main
//  3 |   rest
//    `----
//   help: failed value: BitArray(bytes=[2, 3], bit_len=16)
