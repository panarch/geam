fn fail() -> Bool {
  panic
}

fn loop() -> Int {
  {
    let _ = fail()
    loop()
  }
}

pub fn main() -> Int {
  loop()
}

// @geam:expect-error
// geam::panic
//
//   x panic: `panic` expression evaluated.
//    ,-[tests/fixtures/execution_errors/functions/return_block_step.gleam:2:3]
//  1 | fn fail() -> Bool {
//  2 |   panic
//    :   ^^|^^
//    :     `-- panic in main.fail
//  3 | }
//    `----
