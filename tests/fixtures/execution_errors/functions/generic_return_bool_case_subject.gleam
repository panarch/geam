fn value_stop(value: item) -> item {
  case { panic as "generic value subject failed" } {
    True -> value
    False -> value
  }
}

pub fn main() {
  value_stop(1) == 1
}

// geam:expect-error
// geam::panic
//
//   x panic: generic value subject failed
//    ,-[tests/fixtures/execution_errors/functions/generic_return_bool_case_subject.gleam:2:10]
//  1 | fn value_stop(value: item) -> item {
//  2 |   case { panic as "generic value subject failed" } {
//    :          ^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^
//    :                             `-- panic in main.value_stop
//  3 |     True -> value
//    `----
