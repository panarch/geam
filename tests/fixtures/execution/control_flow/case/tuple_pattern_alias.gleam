pub fn main() {
  let pair = #(1, 2)
  case pair {
    #(left, right) as whole ->
      left == 1 && right == 2 && whole.0 == 1 && whole.1 == 2
  }
}

// @geam:expect Bool(true)
