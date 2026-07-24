fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  let add_one = add(1, _)
  add_one(41)
}

// @geam:expect Int(42)
