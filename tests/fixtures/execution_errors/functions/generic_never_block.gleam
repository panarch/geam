fn first(value: Int, _other: other) {
  value
}

pub fn main() {
  first(1, {
    let _ = Nil
    panic as "generic block failed"
  })
}

// geam:expect-error
// geam::panic
//
//   x panic: generic block failed
//    ,-[tests/fixtures/execution_errors/functions/generic_never_block.gleam:8:5]
//  7 |     let _ = Nil
//  8 |     panic as "generic block failed"
//    :     ^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^
//    :                    `-- panic in main.main
//  9 |   })
//    `----
