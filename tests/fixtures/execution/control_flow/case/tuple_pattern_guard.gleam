pub fn main() {
  let pair = #(1, 2)
  case pair {
    #(left, right) if left > 10 -> right + 100
    #(left, right) if left > 0 -> right
    _ -> 999
  }
}

// geam:expect Int(2)
