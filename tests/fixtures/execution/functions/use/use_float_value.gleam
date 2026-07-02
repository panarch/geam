fn with_value(continue: fn(Float) -> Float) {
  continue(1.5)
}

pub fn main() {
  use value <- with_value
  value +. 0.5
}

// geam:expect Float(2.0)
