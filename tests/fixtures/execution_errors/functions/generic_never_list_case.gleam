fn first(values: List(value)) -> value {
  case values {
    [value, ..] -> value
    _ -> panic as "empty generic list"
  }
}

pub fn main() {
  first([])
}

// @geam:expect-error
// geam::panic
//
//   x panic: empty generic list
//    ,-[tests/fixtures/execution_errors/functions/generic_never_list_case.gleam:4:10]
//  3 |     [value, ..] -> value
//  4 |     _ -> panic as "empty generic list"
//    :          ^^^^^^^^^^^^^^|^^^^^^^^^^^^^^
//    :                        `-- panic in main.first
//  5 |   }
//    `----
