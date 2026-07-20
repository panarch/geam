fn fail(count: Int) -> value {
  case count {
    0 -> panic as "generic recursion failed"
    _ -> fail(count - 1)
  }
}

pub fn main() {
  fail(3)
}

// geam:expect-error
// geam::panic
//
//   x panic: generic recursion failed
//    ,-[tests/fixtures/execution_errors/functions/generic_never_recursion.gleam:3:10]
//  2 |   case count {
//  3 |     0 -> panic as "generic recursion failed"
//    :          ^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^
//    :                           `-- panic in main.fail
//  4 |     _ -> fail(count - 1)
//    `----
