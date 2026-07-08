pub fn main() {
  let values = [1, 2]
  case values {
    [first, second] as whole -> first == 1 && second == 2 && whole == [1, 2]
    _ -> False
  }
}

// geam:expect Bool(true)
