pub fn main() {
  let pair = #(40, 2)
  let add = fn() {
    pair.0 + pair.1
  }

  add()
}

// @geam:expect Int(42)
