fn first(value: Int, _other: other) {
  value
}

pub fn main() {
  let selector = 1.0
  first(
    1,
    case selector {
      1.0 -> panic as "generic float case failed"
      _ -> panic as "unselected generic float case"
    },
  )
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic float case failed
//     ,-[tests/fixtures/execution_errors/functions/generic_never_float_case.gleam:10:14]
//   9 |     case selector {
//  10 |       1.0 -> panic as "generic float case failed"
//     :              ^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^
//     :                                `-- panic in main.main
//  11 |       _ -> panic as "unselected generic float case"
//     `----
