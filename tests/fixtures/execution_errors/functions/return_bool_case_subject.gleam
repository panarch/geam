fn loop() -> Int {
  case { panic } {
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
//    ,-[tests/fixtures/execution_errors/functions/return_bool_case_subject.gleam:2:10]
//  1 | fn loop() -> Int {
//  2 |   case { panic } {
//    :          ^^|^^
//    :            `-- panic in main.loop
//  3 |     True -> loop()
//    `----
