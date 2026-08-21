pub fn main() {
  case #(1, 2) {
    #(1, value) | #(value, 1) -> fn() { value }()
    _ -> 0
  }
}

// @geam:expect Int(2)
