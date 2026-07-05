fn add_one(value: Int) {
  value + 1
}

const f = add_one

pub fn main() {
  f(41)
}

// geam:expect Int(42)
