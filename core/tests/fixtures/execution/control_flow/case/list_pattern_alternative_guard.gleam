pub fn main() {
  case [1, 2] {
    [1, value] | [0, value] if value > 1 -> value
    _ -> 0
  }
}

// @geam:expect Int(2)
