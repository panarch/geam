pub type Optional(value) {
  Empty
  Full(value)
}

fn unwrap(value: Optional(item)) -> item {
  case value {
    Full(item) -> item
    Empty -> panic as "generic custom case failed"
  }
}

pub fn main() {
  unwrap(Empty)
}

// geam:expect-error
// geam::panic
//
//   x panic: generic custom case failed
//     ,-[tests/fixtures/execution_errors/functions/generic_never_custom_case.gleam:9:14]
//   8 |     Full(item) -> item
//   9 |     Empty -> panic as "generic custom case failed"
//     :              ^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^
//     :                                `-- panic in main.unwrap
//  10 |   }
//     `----
