const value = <<<<1>>:bits-size(16)>>
pub fn main() {
  value
}

// geam:expect-error
// geam::bit_array_segment
//
//   x bit_array_segment: BitArray segment construction failed.
//    ,-[tests/fixtures/execution_errors/expressions/constant_bit_array_segment_failure.gleam:1:17]
//  1 | const value = <<<<1>>:bits-size(16)>>
//    :                 ^^^^^^^^^|^^^^^^^^^
//    :                          `-- bit array segment in main.value
//  2 | pub fn main() {
//    `----
//   help: sized bits segment requested 16 bits, but the value contains 8 bits
