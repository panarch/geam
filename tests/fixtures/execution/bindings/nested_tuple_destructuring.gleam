pub fn main() {
  let #(one, #(two, three)) = #(1, #(2, 3))
  one + two + three
}

// @geam:expect Int(6)
