pub type Boxed(value) {
  Boxed(value)
}

fn choice() -> Int {
  0
}

fn custom_stop(path: Int) -> Boxed(value) {
  case path {
    0 -> case True || choice() == 1 {
      True -> Boxed(panic as "custom static bool case failed")
      False -> Boxed(panic as "unselected custom static bool case")
    }
    1 -> case False {
      True -> Boxed(panic as "unselected custom false-case true branch")
      False -> Boxed(panic as "custom false-case failed")
    }
    2 -> case choice() == 1 {
      True -> Boxed(panic as "custom dynamic true branch failed")
      False -> Boxed(panic as "custom dynamic false branch failed")
    }
    _ -> case { panic as "custom bool subject failed" } {
      True -> Boxed(panic as "unselected custom bool true branch")
      False -> Boxed(panic as "unselected custom bool false branch")
    }
  }
}

pub fn main() {
  custom_stop(choice())
  Nil
}

// geam:expect-error
// geam::panic
//
//   x panic: custom static bool case failed
//     ,-[tests/fixtures/execution_errors/functions/generic_custom_never_static_bool_case.gleam:12:21]
//  11 |     0 -> case True || choice() == 1 {
//  12 |       True -> Boxed(panic as "custom static bool case failed")
//     :                     ^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^
//     :                                         `-- panic in main.custom_stop
//  13 |       False -> Boxed(panic as "unselected custom static bool case")
//     `----
