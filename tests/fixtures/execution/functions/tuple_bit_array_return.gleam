fn values() -> #(BitArray) {
  #(<<1>>)
}

pub fn main() {
  values()
}

// geam:expect Tuple([BitArray(bytes=[1], bit_len=8)])
