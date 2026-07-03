fn with_value(continue: fn(Int) -> Int) {
  continue(1)
}

pub fn main() {
  use value as alias <- with_value
  alias
}
