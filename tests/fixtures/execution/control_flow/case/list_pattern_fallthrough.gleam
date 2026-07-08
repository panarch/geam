pub fn main() {
  case [1, 2] {
    [value] -> value
    [left, right] -> left + right
    _ -> 0
  }
}

// geam:expect Int(3)
