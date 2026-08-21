pub fn main() {
  case 1, <<2>> {
    1, value | _, value -> value
  }
}

// @geam:expect BitArray(bytes=[2], bit_len=8)
