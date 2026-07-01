fn with_value(continue: fn(Int) -> Int) {
  continue(32)
}

pub fn main() {
  let base = 10
  use value <- with_value
  value + base
}

// geam:expect Int(42)
