fn selected(_sample: item) -> List(List(item)) {
  let failed: List(List(item)) = panic as "nested generic list failed"
  failed
}

pub fn main() {
  selected(1) == []
}

// @geam:expect-error
// geam::panic
//
//   x panic: nested generic list failed
//    ,-[tests/fixtures/execution_errors/functions/generic_nested_list_panic.gleam:2:34]
//  1 | fn selected(_sample: item) -> List(List(item)) {
//  2 |   let failed: List(List(item)) = panic as "nested generic list failed"
//    :                                  ^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^
//    :                                                    `-- panic in main.selected
//  3 |   failed
//    `----
