fn values() -> List(Int) {
  []
}

pub fn main() {
  case values() {
    [] -> 1
    _ -> 0
  }
}

// @geam:expect Int(1)
