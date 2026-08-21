pub fn main() {
  let base = 1.5
  let add_base = fn(value) {
    value +. base
  }

  add_base(2.5)
}

// @geam:expect Float(4.0)
