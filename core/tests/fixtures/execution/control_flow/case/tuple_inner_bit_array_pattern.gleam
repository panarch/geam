pub fn main() {
  case #(<<1>>) {
    #(<<1>>) -> 1
    _ -> 0
  }
}

// @geam:expect Int(1)
