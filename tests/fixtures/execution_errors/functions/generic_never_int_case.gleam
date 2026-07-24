fn first(value: Int, _other: other) {
  value
}

pub fn main() {
  let selector = 1
  first(
    1,
    case selector {
      1 -> panic as "generic int case failed"
      _ -> panic as "unselected generic int case"
    },
  )
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic int case failed
//     ,-[tests/fixtures/execution_errors/functions/generic_never_int_case.gleam:10:12]
//   9 |     case selector {
//  10 |       1 -> panic as "generic int case failed"
//     :            ^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^
//     :                             `-- panic in main.main
//  11 |       _ -> panic as "unselected generic int case"
//     `----
