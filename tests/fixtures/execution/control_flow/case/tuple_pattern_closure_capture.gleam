pub fn main() {
  let pair = #(1, 2)
  case pair {
    #(left, right) -> fn() { left + right }()
  }
}

// @geam:expect Int(3)
