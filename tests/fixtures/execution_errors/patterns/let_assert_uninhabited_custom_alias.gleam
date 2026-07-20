pub type Optional(value) {
  Empty
  Full(value)
}

fn unwrap(value: Optional(item)) {
  let assert Full(inner as alias) = value
  0
}

pub fn main() {
  unwrap(Empty)
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_uninhabited_custom_alias.gleam:7:3]
//  6 | fn unwrap(value: Optional(item)) {
//  7 |   let assert Full(inner as alias) = value
//    :   ^^^^^|^^^^ ^^^^^^^^^^|^^^^^^^^^
//    :        |               `-- pattern
//    :        `-- let assert in main.unwrap
//  8 |   0
//    `----
//   help: failed value: geam/main/Optional(Parameter(0))::Empty()
