pub fn main() {
  let <<all:bits>> = <<1, 2>>
  let #(<<nested:bits>>, value) = #(<<3>>, 4)
  let <<_ as inner:bits>> = <<5>>
  #(all, nested, value, inner)
}

// geam:expect Tuple([BitArray(bytes=[1, 2], bit_len=16), BitArray(bytes=[3], bit_len=8), Int(4), BitArray(bytes=[5], bit_len=8)])
