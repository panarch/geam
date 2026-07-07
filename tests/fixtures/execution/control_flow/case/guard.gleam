pub fn main() {
  let value = 1
  case value {
    other if other > 0 -> other
    _ -> 0
  }
}

// geam:expect Int(1)
