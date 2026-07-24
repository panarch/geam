fn fail() -> String {
  panic
}

fn loop() -> Int {
  case fail() {
    "one" -> loop()
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
//    ,-[tests/fixtures/execution_errors/functions/return_string_case_subject.gleam:2:3]
//  1 | fn fail() -> String {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.fail
//  3 | }
//    `----
