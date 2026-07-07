pub fn main() {
  let pair = #(1, 2)
  case pair {
    value if value.0 > 10 -> 0
    value as alias if alias.1 == 2 -> value.0 + alias.1
    _ -> 999
  }
}

// geam:expect Int(3)
