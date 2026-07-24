fn with_value(continue: fn(Int) -> Int) {
  continue(41)
}

pub fn main() {
  use value <- with_value
  value + 1
}

// @geam:expect Int(42)
