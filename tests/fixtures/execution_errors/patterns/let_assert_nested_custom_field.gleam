type Maybe(value) {
  Some(value)
  None
}

type Envelope {
  Envelope(value: Maybe(Int))
}

pub fn main() {
  let assert Envelope(value: Some(_)) = Envelope(None)
  Nil
}

// @geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//     ,-[tests/fixtures/execution_errors/patterns/let_assert_nested_custom_field.gleam:11:3]
//  10 | pub fn main() {
//  11 |   let assert Envelope(value: Some(_)) = Envelope(None)
//     :   ^^^^^|^^^^ ^^^^^^^^^^^^|^^^^^^^^^^^
//     :        |                 `-- pattern
//     :        `-- let assert in main.main
//  12 |   Nil
//     `----
//   help: failed value: geam/main/Envelope::Envelope(geam/main/Maybe(Int)::None())
