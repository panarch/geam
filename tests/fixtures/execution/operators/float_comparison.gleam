pub fn main() {
  case 1.0 <. 2.0 && 2.0 <=. 2.0 && 3.0 >. 2.0 && 3.0 >=. 3.0 {
    True -> 42
    False -> 0
  }
}

// @geam:expect Int(42)
