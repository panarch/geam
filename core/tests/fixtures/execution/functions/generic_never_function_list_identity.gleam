pub type Empty

fn diverge(_value: Int) -> Empty {
  panic as "unreached diverging function"
}

pub fn main() {
  [diverge] == [diverge]
}

// @geam:expect Bool(true)
