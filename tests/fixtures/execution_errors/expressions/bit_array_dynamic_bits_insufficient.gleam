pub fn main() {
  let bits = <<1>>
  let size = 9
  <<bits:bits-size(size)>>
}

// @geam:expect-error
// geam::bit_array_segment
//
//   x bit_array_segment: BitArray segment construction failed.
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_dynamic_bits_insufficient.gleam:4:5]
//  3 |   let size = 9
//  4 |   <<bits:bits-size(size)>>
//    :     ^^^^^^^^^^|^^^^^^^^^
//    :               `-- bit array segment in main.main
//  5 | }
//    `----
//   help: sized bits segment requested 9 bits, but the value contains 8 bits
