fn diverge(_value: Int) -> value {
  panic as "unreached diverging function"
}

pub fn main() {
  [diverge] == [diverge]
}

// geam:expect Bool(true)
