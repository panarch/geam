fn values() -> List(BitArray) {
  [<<1>>]
}

pub fn main() {
  let result = values()
  result
}

// geam:expect List(BitArray)([BitArray(bytes=[1], bit_len=8)])
