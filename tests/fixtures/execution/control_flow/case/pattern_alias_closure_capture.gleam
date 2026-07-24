pub fn main() {
  case 21 {
    value as alias -> fn() { value + alias }()
    _ -> 0
  }
}

// @geam:expect Int(42)
