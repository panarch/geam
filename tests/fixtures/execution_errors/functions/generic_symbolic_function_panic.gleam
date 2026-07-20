fn fail() -> value {
  panic as "generic symbolic function failed"
}

fn keep_function(function: fn(input) -> output) {
  function
}

fn choose(value: value, condition: Bool) -> value {
  case condition {
    True -> value
    False -> panic as "unselected generic value"
  }
}

fn codepoint() -> UtfCodepoint {
  case <<"A">> {
    <<value:utf8_codepoint>> -> value
    _ -> panic as "invalid fixture codepoint"
  }
}

pub fn main() {
  let value = codepoint()

  let _ = choose(1, True)
  let _ = choose(1.0, True)
  let _ = choose("one", True)
  let _ = choose(<<1>>, True)
  let _ = choose(value, True)
  let _ = choose(True, True)
  let _ = choose(Nil, True)

  let _ = choose(fn() { 1 }, True)
  let _ = choose(fn() { 1.0 }, True)
  let _ = choose(fn() { "one" }, True)
  let _ = choose(fn() { <<1>> }, True)
  let _ = choose(fn() { value }, True)
  let _ = choose(fn() { True }, True)
  let _ = choose(fn() { Nil }, True)

  keep_function(fail())
}

// geam:expect-error
// geam::panic
//
//   x panic: generic symbolic function failed
//    ,-[tests/fixtures/execution_errors/functions/generic_symbolic_function_panic.gleam:2:3]
//  1 | fn fail() -> value {
//  2 |   panic as "generic symbolic function failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^
//    :                        `-- panic in main.fail
//  3 | }
//    `----
