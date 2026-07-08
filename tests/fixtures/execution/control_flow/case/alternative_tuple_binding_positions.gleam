pub fn main() {
  let pair = #(11, 37)
  case pair {
    #(value, 0) | #(11, value) -> value
    _ -> 0
  }
}

// geam:expect Int(37)
