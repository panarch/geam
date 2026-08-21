fn fail(result: Result(Int, error)) -> value {
  let selected = case result {
    Ok(_) -> panic as "generic certain custom case failed"
    Error(_) -> panic as "unreachable generic result branch"
  }
  selected
}

pub fn main() {
  fail(Ok(1))
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic certain custom case failed
//    ,-[tests/fixtures/execution_errors/functions/generic_certain_custom_never_case.gleam:3:14]
//  2 |   let selected = case result {
//  3 |     Ok(_) -> panic as "generic certain custom case failed"
//    :              ^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^
//    :                                    `-- panic in main.fail
//  4 |     Error(_) -> panic as "unreachable generic result branch"
//    `----
