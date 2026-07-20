fn compare(selector: Bool) -> Bool {
  #(
    case selector {
      True -> panic as "unselected generic case"
      False -> panic as "generic false case failed"
    },
  ) == #(panic as "right operand must not run")
}

pub fn main() {
  compare(False)
}

// geam:expect-error
// geam::panic
//
//   x panic: generic false case failed
//    ,-[tests/fixtures/execution_errors/functions/generic_unresolved_false_case_operand.gleam:5:16]
//  4 |       True -> panic as "unselected generic case"
//  5 |       False -> panic as "generic false case failed"
//    :                ^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^
//    :                                  `-- panic in main.compare
//  6 |     },
//    `----
