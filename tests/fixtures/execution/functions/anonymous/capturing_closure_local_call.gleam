pub fn main() {
  let base = 10
  let add_base = fn(value) {
    let adjusted = base - -value
    adjusted
  }

  add_base(32)
}

// @geam:expect Int(42)
