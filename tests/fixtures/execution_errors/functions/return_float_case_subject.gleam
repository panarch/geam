fn fail() -> Float {
  panic
}

fn loop() -> Int {
  case fail() {
    1.0 -> loop()
    _ -> 0
  }
}

pub fn main() -> Int {
  loop()
}

// @geam:expect-error
// geam::panic
//
//   x panic: `panic` expression evaluated.
//    ,-[tests/fixtures/execution_errors/functions/return_float_case_subject.gleam:2:3]
//  1 | fn fail() -> Float {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.fail
//  3 | }
//    `----
