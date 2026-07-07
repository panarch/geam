fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let assert [] = [#(1.5, "one", True, Nil, add_one)]
  1
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/patterns/let_assert_compound_failed_value.gleam:6:3]
//  5 | pub fn main() {
//  6 |   let assert [] = [#(1.5, "one", True, Nil, add_one)]
//    :   ^^^^^|^^^^ ^|
//    :        |      `-- pattern
//    :        `-- let assert in main.main
//  7 |   1
//    `----
//   help: failed value: List(#(Float, String, Bool, Nil, fn(Int) -> Int))([Tuple([Float(1.5), String("one"), Bool(true), Nil, Function(fn(Int) -> Int)])])
