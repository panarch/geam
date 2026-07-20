fn first(value: Int, _other: other) {
  value
}

pub fn main() {
  let selector = "selected"
  first(
    1,
    case selector {
      "selected" -> panic as "generic string case failed"
      _ -> panic as "unselected generic string case"
    },
  )
}

// geam:expect-error
// geam::panic
//
//   x panic: generic string case failed
//     ,-[tests/fixtures/execution_errors/functions/generic_never_string_case.gleam:10:21]
//   9 |     case selector {
//  10 |       "selected" -> panic as "generic string case failed"
//     :                     ^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^
//     :                                       `-- panic in main.main
//  11 |       _ -> panic as "unselected generic string case"
//     `----
