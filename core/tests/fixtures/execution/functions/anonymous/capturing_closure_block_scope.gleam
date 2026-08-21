pub fn main() {
  let add_base = {
    let base = 10
    fn(value) { value + base }
  }

  add_base(32)
}

// @geam:expect Int(42)
