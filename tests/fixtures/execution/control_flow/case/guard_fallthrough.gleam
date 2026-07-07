pub fn main() {
  let value = -1
  case value {
    other if other > 0 -> 999
    _ -> 0
  }
}

// geam:expect Int(0)
