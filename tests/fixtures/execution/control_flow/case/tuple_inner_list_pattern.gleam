pub fn main() {
  let pair = #([1, 2], 3)
  case pair {
    #([first, ..], value) -> first + value
    _ -> 0
  }
}

// @geam:expect Int(4)
