pub fn main() {
  let other = 10
  let alias = 20
  let result = case 1 {
    other as alias -> other + alias
    _ -> 0
  }

  result + other + alias
}

// geam:expect Int(32)
