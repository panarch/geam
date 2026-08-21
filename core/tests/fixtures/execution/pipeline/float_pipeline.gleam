fn add(value: Float, amount: Float) {
  value +. amount
}

pub fn main() {
  1.5
  |> add(0.5)
}

// @geam:expect Float(2.0)
