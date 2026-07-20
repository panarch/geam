fn first(value: Int, _other: other) {
  value
}

pub fn main() {
  let selector = True
  first(
    1,
    case selector {
      True -> panic as "generic bool case failed"
      False -> panic as "unselected generic bool case"
    },
  )
}

// geam:expect-error
// geam::panic
//
//   x panic: generic bool case failed
//     ,-[tests/fixtures/execution_errors/functions/generic_never_bool_case.gleam:10:15]
//   9 |     case selector {
//  10 |       True -> panic as "generic bool case failed"
//     :               ^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^
//     :                                `-- panic in main.main
//  11 |       False -> panic as "unselected generic bool case"
//     `----
