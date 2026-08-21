pub fn main() {
  let values = [1, 2]
  case values {
    subject as alias -> subject == alias
  }
}

// @geam:expect Bool(true)
