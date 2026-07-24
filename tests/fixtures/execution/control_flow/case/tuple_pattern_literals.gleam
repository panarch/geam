pub fn main() {
  let value = #(1, "one", True, 1.5, Nil)
  case value {
    #(2, "two", False, 2.5, Nil) -> 0
    #(1, "one", True, 1.5, Nil) -> 42
    _ -> 999
  }
}

// @geam:expect Int(42)
