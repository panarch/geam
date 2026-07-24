fn choose(value: Bool) -> List(BitArray) {
  case value {
    True -> [<<1>>]
    False -> [<<2>>]
  }
}

pub fn main() {
  #(choose(True), choose(False))
}

// @geam:expect Tuple([List(BitArray)([BitArray(bytes=[1], bit_len=8)]), List(BitArray)([BitArray(bytes=[2], bit_len=8)])])
