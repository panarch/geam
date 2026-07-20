fn fail() -> value {
  panic as "generic tuple operand failed"
}

pub fn main() {
  #(fail()) == #(fail())
}

// geam:expect-error
// geam::panic
//
//   x panic: generic tuple operand failed
//    ,-[tests/fixtures/execution_errors/functions/generic_unresolved_tuple_equality_operand.gleam:2:3]
//  1 | fn fail() -> value {
//  2 |   panic as "generic tuple operand failed"
//    :   ^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^
//    :                      `-- panic in main.fail
//  3 | }
//    `----
