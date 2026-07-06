fn with_value(continue: fn(Int) -> Int) {
  continue(1)
}

pub fn main() -> Int {
  use value <- with_value
}

// geam:expect-error Panic(incomplete_use)
