fn with_value(value: Int, continue: fn(Int) -> Int) {
  continue(value)
}

pub fn main() {
  use left <- with_value(20)
  use right <- with_value(22)
  left + right
}

// geam:expect Int(42)
