fn invalid_int() -> Int {
  let assert 1 = 2
  0
}

fn fail(_value: Int) -> value {
  panic as "callee must not run"
}

fn provide(_value: Int) -> fn(Int) -> value {
  fail
}

pub fn main() {
  #(provide(invalid_int()))
}

// geam:expect-error
// geam::let_assert
//
//   x let_assert: Pattern match failed, no pattern matched the value.
//    ,-[tests/fixtures/execution_errors/functions/generic_never_returned_function_argument_failure.gleam:2:3]
//  1 | fn invalid_int() -> Int {
//  2 |   let assert 1 = 2
//    :   ^^^^^|^^^^ |
//    :        |     `-- pattern
//    :        `-- let assert in main.invalid_int
//  3 |   0
//    `----
//   help: failed value: Int(2)
