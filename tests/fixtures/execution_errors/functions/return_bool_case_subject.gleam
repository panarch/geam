fn fail() -> Bool {
  panic
}

fn loop() -> Int {
  case fail() {
    True -> loop()
    False -> 0
  }
}

pub fn main() -> Int {
  loop()
}

// geam:expect-error
// geam::panic
//
//   x panic: `panic` expression evaluated.
//    ,-[tests/fixtures/execution_errors/functions/return_bool_case_subject.gleam:2:3]
//  1 | fn fail() -> Bool {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.fail
//  3 | }
//    `----
