pub fn main() {
  let threshold = 40
  fn(value) {
    case value {
      other if other > threshold -> other + 1
      _ -> 0
    }
  }(41)
}

// @geam:expect Int(42)
