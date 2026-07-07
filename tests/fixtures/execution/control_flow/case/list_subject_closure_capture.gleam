pub fn main() {
  let values = [1, 2]
  case values {
    captured -> fn() { captured == [1, 2] }()
  }
}

// geam:expect Bool(true)
