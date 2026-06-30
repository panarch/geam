pub fn main() {
  let base = 10
  let make_adder = fn() { fn(value) { value + base } }
  let add_base = make_adder()

  add_base(32)
}

// geam:expect Int(42)
