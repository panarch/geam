fn add(left: Int, right: Int) {
  left + right
}

fn multiply(left: Int, right: Int) {
  left * right
}

fn subtract(left: Int, right: Int) {
  left - right
}

pub fn main() {
  let chained = 1 |> add(2) |> multiply(3)
  let hole = 1 |> subtract(10, _)

  chained + hole
}

// geam:expect Int(18)
