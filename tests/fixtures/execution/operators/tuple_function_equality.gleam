fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  #(1, add_one) == #(1, add_one)
}

// @geam:expect Bool(true)
