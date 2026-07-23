import helper

pub fn main() {
  helper.fail()
}
// geam:expect-error
// geam::panic
//
//   x panic: dependency failed
//    ,-[tests/fixtures/execution_errors/modules/dependency_panic/helper.gleam:2:3]
//  1 | pub fn fail() -> Int {
//  2 |   panic as "dependency failed"
//    :   ^^^^^^^^^^^^^^|^^^^^^^^^^^^^
//    :                 `-- panic in helper.fail
//  3 | }
//    `----
