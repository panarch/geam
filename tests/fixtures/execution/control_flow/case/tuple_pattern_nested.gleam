pub fn main() {
  let pair = #(1, #(2, 3))
  case pair {
    #(left, #(middle, right)) -> left == 1 && middle == 2 && right == 3
  }
}

// @geam:expect Bool(true)
