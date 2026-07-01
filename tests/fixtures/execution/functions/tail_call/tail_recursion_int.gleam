fn count_down(n: Int, acc: Int) {
  case n {
    0 -> acc
    _ -> count_down(n - 1, acc + 1)
  }
}

pub fn main() {
  count_down(10000, 0)
}

// geam:expect Int(10000)
