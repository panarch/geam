pub fn main() {
  let bits = <<0xab, 0xcd>>
  let dynamic_size = 12
  let unaligned = <<0b10101:size(5)>>

  #(
    <<bits:bits-size(8)>>,
    <<bits:bits-size(dynamic_size)>>,
    <<bits:bits-size(3)-unit(4)>>,
    <<bits:bits-size(dynamic_size / 4)-unit(4)>>,
    <<bits:bits-size(0)>>,
    <<bits:bits-size(dynamic_size - 20)>>,
    <<unaligned:bits-size(3)>>,
  )
}

// geam:expect Tuple([BitArray(bytes=[171], bit_len=8), BitArray(bytes=[171, 192], bit_len=12), BitArray(bytes=[171, 192], bit_len=12), BitArray(bytes=[171, 192], bit_len=12), BitArray(bytes=[], bit_len=0), BitArray(bytes=[], bit_len=0), BitArray(bytes=[160], bit_len=3)])
