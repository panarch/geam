pub fn main() {
  let base = 1
  let add_block_base = {
    let base = 10
    fn(value) { value + base }
  }

  add_block_base(31) + base
}

// @geam:expect Int(42)
