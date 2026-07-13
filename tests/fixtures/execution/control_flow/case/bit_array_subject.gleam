pub fn main() {
  case <<1>> {
    value as alias if value == alias -> value
    _ -> <<0>>
  }
}

// geam:expect BitArray(bytes=[1], bit_len=8)
