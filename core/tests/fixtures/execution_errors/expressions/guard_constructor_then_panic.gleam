pub type Box {
  Box(Int)
}

pub fn main() {
  let value = 7
  case Box(7) {
    candidate if Box(value) == candidate -> {
      echo value as "matched"
      panic as "after"
    }
    _ -> panic as "unmatched"
  }
}
// @geam:echo
// tests/fixtures/execution_errors/expressions/guard_constructor_then_panic.gleam:9 matched
// 7
// @geam:expect-error
// geam::panic
//
//   x panic: after
//     ,-[tests/fixtures/execution_errors/expressions/guard_constructor_then_panic.gleam:10:7]
//   9 |       echo value as "matched"
//  10 |       panic as "after"
//     :       ^^^^^^^^|^^^^^^^
//     :               `-- panic in main.main
//  11 |     }
//     `----
