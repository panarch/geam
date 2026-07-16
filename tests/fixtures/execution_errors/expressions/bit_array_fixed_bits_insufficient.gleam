pub fn main() {
  let bits = <<1>>
  <<bits:bits-size(9)>>
}

// geam:expect-error
// geam::bit_array_segment
//
//   x bit_array_segment: BitArray segment construction failed.
//    ,-[tests/fixtures/execution_errors/expressions/bit_array_fixed_bits_insufficient.gleam:3:5]
//  2 |   let bits = <<1>>
//  3 |   <<bits:bits-size(9)>>
//    :     ^^^^^^^^|^^^^^^^^
//    :             `-- bit array segment in main.main
//  4 | }
//    `----
//   help: sized bits segment requested 9 bits, but the value contains 8 bits
