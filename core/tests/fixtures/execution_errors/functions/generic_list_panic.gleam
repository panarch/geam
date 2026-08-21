fn selected(_sample: item) -> List(item) {
  let failed: List(item) = panic as "generic list failed"
  failed
}

pub fn main() {
  selected(1) == []
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic list failed
//    ,-[tests/fixtures/execution_errors/functions/generic_list_panic.gleam:2:28]
//  1 | fn selected(_sample: item) -> List(item) {
//  2 |   let failed: List(item) = panic as "generic list failed"
//    :                            ^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^
//    :                                           `-- panic in main.selected
//  3 |   failed
//    `----
