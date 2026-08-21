fn provider() {
  fn(_value) {
    panic as "generic closure failed"
  }
}

pub fn main() {
  provider()(Nil)
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic closure failed
//    ,-[tests/fixtures/execution_errors/functions/generic_never_closure_call.gleam:3:5]
//  2 |   fn(_value) {
//  3 |     panic as "generic closure failed"
//    :     ^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^
//    :                     `-- panic in main.<anonymous:0>
//  4 |   }
//    `----
