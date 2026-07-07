pub fn main() {
  let value = 1
  case value {
    other as alias -> other + alias
    _ -> 0
  }
}

// geam:expect Int(2)
