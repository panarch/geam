fn with_values(continue: fn(List(Int)) -> Bool) {
  continue([1, 2, 3])
}

pub fn main() {
  use values <- with_values
  values == [1, 2, 3]
}

// @geam:expect Bool(true)
