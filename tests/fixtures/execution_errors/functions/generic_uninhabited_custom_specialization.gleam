pub type Boxed(value) {
  Boxed(value)
}

fn impossible(stop: Bool) -> Boxed(value) {
  case stop {
    True -> Boxed(panic as "generic custom specialization failed")
    False -> impossible(stop)
  }
}

pub fn main() {
  impossible(True)
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic custom specialization failed
//    ,-[tests/fixtures/execution_errors/functions/generic_uninhabited_custom_specialization.gleam:7:19]
//  6 |   case stop {
//  7 |     True -> Boxed(panic as "generic custom specialization failed")
//    :                   ^^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^^
//    :                                          `-- panic in main.impossible
//  8 |     False -> impossible(stop)
//    `----
