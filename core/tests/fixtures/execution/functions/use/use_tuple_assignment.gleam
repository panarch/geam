fn with_pair(continue: fn(#(Int, Int)) -> Int) {
  continue(#(1, 2))
}

pub fn main() {
  use #(one, two) <- with_pair
  one + two
}

// @geam:expect Int(3)
