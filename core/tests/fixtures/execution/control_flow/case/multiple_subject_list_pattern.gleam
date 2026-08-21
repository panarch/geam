pub fn main() {
  case 1, [2] {
    _, [value] -> value
    _, _ -> 0
  }
}

// @geam:expect Int(2)
