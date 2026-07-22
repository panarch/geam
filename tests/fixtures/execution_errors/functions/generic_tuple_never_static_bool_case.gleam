fn choice() -> Int {
  0
}

fn tuple_stop(path: Int) -> #(value) {
  case path {
    0 -> case True || choice() == 1 {
      True -> #(panic as "tuple static bool case failed")
      False -> #(panic as "unselected tuple static bool case")
    }
    1 -> case False {
      True -> #(panic as "unselected tuple false-case true branch")
      False -> #(panic as "tuple false-case failed")
    }
    2 -> case choice() == 1 {
      True -> #(panic as "tuple dynamic true branch failed")
      False -> #(panic as "tuple dynamic false branch failed")
    }
    _ -> case { panic as "tuple bool subject failed" } {
      True -> #(panic as "unselected tuple bool true branch")
      False -> #(panic as "unselected tuple bool false branch")
    }
  }
}

pub fn main() {
  tuple_stop(choice())
  Nil
}

// geam:expect-error
// geam::panic
//
//   x panic: tuple static bool case failed
//    ,-[tests/fixtures/execution_errors/functions/generic_tuple_never_static_bool_case.gleam:8:17]
//  7 |     0 -> case True || choice() == 1 {
//  8 |       True -> #(panic as "tuple static bool case failed")
//    :                 ^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^
//    :                                     `-- panic in main.tuple_stop
//  9 |       False -> #(panic as "unselected tuple static bool case")
//    `----
