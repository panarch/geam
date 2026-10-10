pub type Empty

fn stop() -> a {
  panic as "body"
}

fn stopped_list() -> List(a) {
  [stop()]
}

pub fn main() -> Empty {
  let values: List(Empty) = stopped_list()
  let assert [value, ..] = values
  value
}
// @geam:expect-error
// geam::panic
//
//   x panic: body
//    ,-[tests/fixtures/execution_errors/functions/generic_empty_list_item_failure.gleam:4:3]
//  3 | fn stop() -> a {
//  4 |   panic as "body"
//    :   ^^^^^^^|^^^^^^^
//    :          `-- panic in main.stop
//  5 | }
//    `----
