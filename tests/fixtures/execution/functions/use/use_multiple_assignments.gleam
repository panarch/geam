fn with_pair(continue: fn(Int, Int) -> Int) {
  continue(20, 22)
}

pub fn main() {
  use left, right <- with_pair
  left + right
}

// geam:expect Int(42)
