pub fn main() {
  let pair = #(1, 2)
  case pair {
    #(1, value) | #(2, value) -> value
    _ -> 0
  }
}

// @geam:expect Int(2)
