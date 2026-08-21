fn with_value(continue: fn(Int) -> Int) {
  continue(32)
}

pub fn main() {
  let base = 10
  let run = fn() {
    use value <- with_value
    value + base
  }

  run()
}

// @geam:expect Int(42)
