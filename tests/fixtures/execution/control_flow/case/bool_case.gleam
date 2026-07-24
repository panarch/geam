fn enabled() {
  True
}

pub fn main() {
  case True {
    True -> "enabled"
    False -> "disabled"
  }

  case True {
    True -> True
    False -> False
  }

  case True {
    True -> Nil
    False -> Nil
  }

  case enabled() {
    True -> 42
    False -> 0
  }
}

// @geam:expect Int(42)
