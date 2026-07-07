pub fn main() {
  let pair = #(1, 2)
  case pair {
    value as alias -> value.0 + alias.1
  }
}

// geam:expect Int(3)
