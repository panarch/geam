fn loop(n: Int, acc: Int) {
  {
    case n {
      0 -> acc
      _ -> loop(n - 1, acc + 2)
    }
  }
}

pub fn main() {
  loop(5000, 0)
}

// @geam:expect Int(10000)
