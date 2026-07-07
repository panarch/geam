pub fn main() {
  let pair = #(1, 2)
  case pair {
    #(left, right) as whole -> left + right + whole.0
  }
}
