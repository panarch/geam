pub fn main() {
  1 |> fn(right) { fn(left) { left + right } }(2)
}

// @geam:expect Int(3)
