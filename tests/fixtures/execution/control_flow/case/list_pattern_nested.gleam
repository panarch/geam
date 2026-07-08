pub fn main() {
  case [[1, 2], [3]] {
    [[first, ..], [second]] -> first == 1 && second == 3
    _ -> False
  }
}

// geam:expect Bool(true)
