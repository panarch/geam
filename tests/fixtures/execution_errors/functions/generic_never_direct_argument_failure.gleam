fn invalid_int() -> Int {
  let assert 1 = 2
  0
}

fn fail(_value: Int) -> value {
  panic as "callee must not run"
}

pub fn main() {
  #(fail(invalid_int()))
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/functions/generic_never_direct_argument_failure.gleam:2:3]
//  1 | fn invalid_int() -> Int {
//  2 |   let assert 1 = 2
//    :   ^^^^^|^^^^ |
//    :        |     `-- pattern
//    :        `-- let assert in main.invalid_int
//  3 |   0
//    `----
//   help: failed value: Int(2)
