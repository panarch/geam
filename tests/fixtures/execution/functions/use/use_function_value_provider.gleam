fn with_value(continue: fn(Int) -> Int) {
  continue(41)
}

pub fn main() {
  let provider = with_value
  use value <- provider
  value + 1
}

// geam:expect Int(42)
