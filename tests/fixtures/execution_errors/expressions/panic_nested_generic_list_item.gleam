fn choose(condition: Bool) -> List(value) {
  case condition {
    True -> [panic as "nested generic list item failed"]
    False -> []
  }
}

pub fn main() -> List(List(value)) {
  [choose(True)]
}

// geam:expect-error
// geam::panic
//
//   x panic: nested generic list item failed
//    ,-[tests/fixtures/execution_errors/expressions/panic_nested_generic_list_item.gleam:3:14]
//  2 |   case condition {
//  3 |     True -> [panic as "nested generic list item failed"]
//    :              ^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^
//    :                                   `-- panic in main.choose
//  4 |     False -> []
//    `----
