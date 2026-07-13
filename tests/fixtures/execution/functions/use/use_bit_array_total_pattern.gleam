fn with_bits(continue: fn(BitArray) -> Int) {
  continue(<<1, 2>>)
}

pub fn main() {
  use <<all:bits>> <- with_bits
  case all {
    <<1, 2>> -> 1
    _ -> 0
  }
}

// geam:expect Int(1)
