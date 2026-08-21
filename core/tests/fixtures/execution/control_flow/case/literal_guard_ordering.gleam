pub fn main() {
  case 1 {
    1 if False -> 999
    1 -> 42
    _ -> 0
  }
}

// @geam:expect Int(42)
