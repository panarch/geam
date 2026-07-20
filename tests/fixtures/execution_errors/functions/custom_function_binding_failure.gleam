pub type Boxed {
  Boxed
}

fn fail() -> fn() -> Boxed {
  panic as "custom function binding failed"
}

pub fn main() {
  let function = fail()
  function
}

// geam:expect-error
// geam::panic
//
//   x panic: custom function binding failed
//    ,-[tests/fixtures/execution_errors/functions/custom_function_binding_failure.gleam:6:3]
//  5 | fn fail() -> fn() -> Boxed {
//  6 |   panic as "custom function binding failed"
//    :   ^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^
//    :                       `-- panic in main.fail
//  7 | }
//    `----
