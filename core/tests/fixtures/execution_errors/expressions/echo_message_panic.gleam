pub fn main() {
  echo 1 as {
    panic as "message failed"
  }
}

// @geam:expect-error
// geam::panic
//
//   x panic: message failed
//    ,-[tests/fixtures/execution_errors/expressions/echo_message_panic.gleam:3:5]
//  2 |   echo 1 as {
//  3 |     panic as "message failed"
//    :     ^^^^^^^^^^^^|^^^^^^^^^^^^
//    :                 `-- panic in main.main
//  4 |   }
//    `----
