pub fn main() {
  let value = 2
  case value {
    1 | 2 if False -> 10
    2 -> 20
    _ -> 0
  }
}

// @geam:expect Int(20)
