fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let f = add_one
  1 |> f
}

// @geam:expect Int(2)
