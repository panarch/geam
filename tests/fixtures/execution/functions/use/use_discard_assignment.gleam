fn with_value(continue: fn(Int) -> Int) {
  continue(41)
}

pub fn main() {
  use _ <- with_value
  42
}

// geam:expect Int(42)
