pub fn main() {
  case [1, 2] {
    [left, right] -> left == 1 && right == 2
    _ -> False
  }
}

// @geam:expect Bool(true)
