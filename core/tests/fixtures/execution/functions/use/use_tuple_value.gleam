fn with_pair(continue: fn(#(Int, String)) -> String) {
  continue(#(1, "one"))
}

pub fn main() {
  use pair <- with_pair
  pair.1
}

// @geam:expect String("one")
