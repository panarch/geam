pub fn main() {
  let add = fn(left, right) { left + right }
  let add_one = add(1, _)
  add_one(41)
}

// @geam:expect Int(42)
