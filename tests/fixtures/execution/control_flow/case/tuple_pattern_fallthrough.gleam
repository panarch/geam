pub fn main() {
  let pair = #(2, 5)
  case pair {
    #(1, value) -> value
    #(2, value) -> value + 10
    _ -> 0
  }
}

// @geam:expect Int(15)
