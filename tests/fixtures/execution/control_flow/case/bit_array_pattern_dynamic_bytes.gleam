pub fn main() {
  case <<3, 10, 20, 30>> {
    <<length, payload:bytes-size(length)>> -> payload
    _ -> <<>>
  }
}

// geam:expect BitArray(bytes=[10, 20, 30], bit_len=24)
