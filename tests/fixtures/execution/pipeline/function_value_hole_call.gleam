fn subtract(left: Int, right: Int) {
  left - right
}

pub fn main() {
  let f = subtract
  1 |> f(10, _)
}

// geam:expect Int(9)
