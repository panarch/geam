fn add(to base: Int, value amount: Int) {
  base + amount
}

pub fn main() {
  let add_one = add(to: 1, value: _)
  add_one(41)
}

// geam:expect Int(42)
