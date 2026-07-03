fn with_value(value value: Int, continue continue: fn(Int) -> Int) {
  continue(value)
}

pub fn main() {
  use value <- with_value(value: 41)
  value + 1
}

// geam:expect Int(42)
