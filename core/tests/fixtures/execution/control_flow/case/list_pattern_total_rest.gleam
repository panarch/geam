pub fn main() {
  case [1, 2] {
    [..rest] -> rest == [1, 2]
  }
}

// @geam:expect Bool(true)
