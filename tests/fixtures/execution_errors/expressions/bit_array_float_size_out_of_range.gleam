pub fn main() {
  let size = 99999999999999999999999999999999999999999999999999
  <<1.5:float-size(size)>>
}

// @geam:expect-error
// geam::bit_array_segment
//
//   x bit_array_segment: BitArray segment construction failed.
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_float_size_out_of_range.gleam:3:5]
//  2 |   let size = 99999999999999999999999999999999999999999999999999
//  3 |   <<1.5:float-size(size)>>
//    :     ^^^^^^^^^^|^^^^^^^^^
//    :               `-- bit array segment in main.main
//  4 | }
//    `----
//   help: BitArray segment size 99999999999999999999999999999999999999999999999999 exceeds the supported host range
