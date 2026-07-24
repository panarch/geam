fn add(to base: Int, value amount: Int) {
  base + amount
}

pub fn main() {
  2 |> add(to: 40)
}

// @geam:expect Int(42)
