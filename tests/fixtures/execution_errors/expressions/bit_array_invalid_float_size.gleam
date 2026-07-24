pub fn main() {
  let size = 24
  <<1.5:float-size(size)>>
}

// @geam:expect-error
// geam::bit_array_segment
//
//   x bit_array_segment: BitArray segment construction failed.
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_invalid_float_size.gleam:3:5]
//  2 |   let size = 24
//  3 |   <<1.5:float-size(size)>>
//    :     ^^^^^^^^^^|^^^^^^^^^
//    :               `-- bit array segment in main.main
//  4 | }
//    `----
//   help: float segments must be 16, 32, or 64 bits; evaluated size was 24 bits
