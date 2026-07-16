pub type Payload {
  Payload(Int)
}

pub fn main() {
  let assert #([1], Payload(2), <<3>>) = #([1], Payload(9), <<3>>)
  0
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_nested_compound_pattern.gleam:6:3]
//  5 | pub fn main() {
//  6 |   let assert #([1], Payload(2), <<3>>) = #([1], Payload(9), <<3>>)
//    :   ^^^^^|^^^^ ^^^^^^^^^^^^|^^^^^^^^^^^^
//    :        |                 `-- pattern
//    :        `-- let assert in main.main
//  7 |   0
//    `----
//   help: failed value: Tuple([List(Int)([Int(1)]), geam/main/Payload::Payload(Int(9)), BitArray(bytes=[3], bit_len=8)])
