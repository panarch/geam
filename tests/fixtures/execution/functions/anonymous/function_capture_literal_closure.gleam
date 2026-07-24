fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  let base = 1
  let add_base = add(base, _)
  add_base(41)
}

// @geam:expect Int(42)
