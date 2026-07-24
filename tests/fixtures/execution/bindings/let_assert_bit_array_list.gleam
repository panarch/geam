pub fn main() {
  let assert [first, ..rest] = [<<1>>, <<2>>]
  #(first, rest)
}

// @geam:expect Tuple([BitArray(bytes=[1], bit_len=8), List(BitArray)([BitArray(bytes=[2], bit_len=8)])])
