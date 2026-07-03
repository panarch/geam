pub fn main() {
  let pair = #(1, 2)
  case pair {
    #(left, right) -> left + right
  }
}
