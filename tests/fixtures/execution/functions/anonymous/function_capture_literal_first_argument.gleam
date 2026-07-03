fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  let add_ten = add(_, 10)
  add_ten(32)
}

// geam:expect Int(42)
