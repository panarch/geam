pub fn main() {
  case 1, 2 {
    _, value | value, _ if value > 1 -> value
    _, _ -> 0
  }
}

// @geam:expect Int(2)
