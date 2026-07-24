pub fn main() {
  let other = 100
  let selected = case 1 {
    other -> other
  }

  other + selected
}

// @geam:expect Int(101)
