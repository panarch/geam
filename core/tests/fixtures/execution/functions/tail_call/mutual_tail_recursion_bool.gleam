fn even(n: Int) {
  case n {
    0 -> True
    _ -> odd(n - 1)
  }
}

fn odd(n: Int) {
  case n {
    0 -> False
    _ -> even(n - 1)
  }
}

pub fn main() {
  even(10001)
}

// @geam:expect Bool(false)
