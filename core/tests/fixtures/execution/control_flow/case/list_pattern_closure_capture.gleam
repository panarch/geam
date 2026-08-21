pub fn main() {
  case [1, 2] {
    [first, ..] -> fn() { first }()
    _ -> 0
  }
}

// @geam:expect Int(1)
