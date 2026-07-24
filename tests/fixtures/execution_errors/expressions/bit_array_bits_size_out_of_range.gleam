pub fn main() {
  let bits = <<1>>
  let size = 99999999999999999999999999999999999999999999999999
  <<bits:bits-size(size)>>
}

// @geam:expect-error
// geam::bit_array_segment
//
//   x bit_array_segment: BitArray segment construction failed.
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_bits_size_out_of_range.gleam:4:5]
//  3 |   let size = 99999999999999999999999999999999999999999999999999
//  4 |   <<bits:bits-size(size)>>
//    :     ^^^^^^^^^^|^^^^^^^^^
//    :               `-- bit array segment in main.main
//  5 | }
//    `----
//   help: BitArray segment size 99999999999999999999999999999999999999999999999999 exceeds the supported host range
