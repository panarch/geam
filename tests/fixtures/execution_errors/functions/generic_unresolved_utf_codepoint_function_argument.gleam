fn fail() -> value {
  panic as "generic function argument failed"
}

fn result(_other: other) -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

pub fn main() {
  let function = result
  function(fail())
}

// geam:expect-error
// geam::panic
//
//   x panic: generic function argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_unresolved_utf_codepoint_function_argument.gleam:2:3]
//  1 | fn fail() -> value {
//  2 |   panic as "generic function argument failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^
//    :                        `-- panic in main.fail
//  3 | }
//    `----
