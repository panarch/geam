fn list_stop() -> List(item) {
  case { panic as "generic list subject failed" } {
    True -> []
    False -> []
  }
}

pub fn main() {
  list_stop() == []
}

// geam:expect-error
// geam::panic
//
//   x panic: generic list subject failed
//    ,-[tests/fixtures/execution_errors/functions/generic_list_return_bool_case_subject.gleam:2:10]
//  1 | fn list_stop() -> List(item) {
//  2 |   case { panic as "generic list subject failed" } {
//    :          ^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^
//    :                             `-- panic in main.list_stop
//  3 |     True -> []
//    `----
