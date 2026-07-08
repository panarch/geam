pub fn main() {
  let values = [1, 2]
  case values {
    [first, ..] -> first
    _ -> 0
  }
}

// geam:expect Int(1)
