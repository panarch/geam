fn count_down(n: Int, acc: #(Int, Int)) {
  case n {
    0 -> acc
    _ -> count_down(n - 1, #(acc.0 + 1, acc.1 + 1))
  }
}

pub fn main() {
  let result = count_down(10000, #(0, 0))
  result.0 + result.1
}

// @geam:expect Int(20000)
