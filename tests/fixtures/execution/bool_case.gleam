fn enabled() {
  True
}

pub fn main() {
  case enabled() {
    True -> 42
    False -> 0
  }
}

// geam:expect Int(42)
