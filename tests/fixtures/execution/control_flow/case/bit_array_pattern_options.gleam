pub fn main() {
  case <<1>> {
    <<value:bytes>> -> value
    _ -> <<>>
  }
}

// @geam:expect BitArray(bytes=[1], bit_len=8)
