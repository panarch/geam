fn choice() -> Int {
  0
}

fn generic_stop(path: Int) -> value {
  case path {
    0 -> case True || choice() == 1 {
      True -> panic as "generic static bool case failed"
      False -> panic as "unselected generic static bool case"
    }
    1 -> case False {
      True -> panic as "unselected generic false-case true branch"
      False -> panic as "generic false-case failed"
    }
    2 -> case choice() == 1 {
      True -> panic as "generic dynamic true branch failed"
      False -> panic as "generic dynamic false branch failed"
    }
    _ -> case { panic as "generic bool subject failed" } {
      True -> panic as "unselected generic bool true branch"
      False -> panic as "unselected generic bool false branch"
    }
  }
}

pub fn main() {
  generic_stop(choice())
  Nil
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic static bool case failed
//    ,-[tests/fixtures/execution_errors/functions/generic_never_static_bool_case.gleam:8:15]
//  7 |     0 -> case True || choice() == 1 {
//  8 |       True -> panic as "generic static bool case failed"
//    :               ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^
//    :                                    `-- panic in main.generic_stop
//  9 |       False -> panic as "unselected generic static bool case"
//    `----
