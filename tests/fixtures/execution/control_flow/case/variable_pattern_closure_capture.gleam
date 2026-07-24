pub fn main() {
  case 41 {
    other -> fn() { other + 1 }()
  }
}

// @geam:expect Int(42)
