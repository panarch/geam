fn with_value(continue: fn(Int) -> Int) {
  continue(40)
}

pub fn main() {
  {
    let base = 2
    use value <- with_value
    value + base
  }
}

// @geam:expect Int(42)
