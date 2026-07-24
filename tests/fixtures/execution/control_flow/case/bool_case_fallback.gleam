fn fallback(value: Bool) {
  case value {
    True -> 10
    _ -> 20
  }
}

fn fallback_first(value: Bool) {
  case value {
    _ -> 7
    True -> 1
  }
}

pub fn main() {
  fallback(False) + fallback_first(True)
}

// @geam:expect Int(27)
