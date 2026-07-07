fn with_values(continue: fn(List(Int)) -> List(Int)) {
  continue([1])
}

pub fn main() {
  use [..rest] <- with_values
  rest
}

// geam:expect List(Int)([Int(1)])
