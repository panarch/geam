fn with_value(continue: fn(Int) -> Int) {
  continue(1)
}

pub fn main() -> Int {
  use value <- with_value
}

// geam:expect-error
// geam::incomplete_use
//
//   x incomplete_use: Use callback is incomplete.
//    ,-[tests/fixtures/execution_errors/functions/use/incomplete_use.gleam:6:3]
//  5 | pub fn main() -> Int {
//  6 |   use value <- with_value
//    :   ^^^^^^^^^^^|^^^^^^^^^^^
//    :              `-- incomplete use in main.<anonymous:0>
//  7 | }
//    `----
