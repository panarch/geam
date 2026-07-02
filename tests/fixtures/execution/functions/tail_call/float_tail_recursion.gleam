fn loop(count: Int, acc: Float) {
  case count {
    0 -> acc
    _ -> loop(count - 1, acc +. 0.5)
  }
}

pub fn main() {
  loop(10000, 0.0)
}

// geam:expect Float(5000.0)
