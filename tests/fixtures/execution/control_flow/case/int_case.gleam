fn classify(value: Int) {
  case value {
    0 -> 10
    1 -> 20
    _ -> 30
  }
}

fn fallback_first(value: Int) {
  case value {
    _ -> 7
    1 -> 99
  }
}

fn duplicate_literal(value: Int) {
  case value {
    1 -> 100
    1 -> 200
    _ -> 0
  }
}

pub fn main() {
  case 1 {
    1 -> "one"
    _ -> "other"
  }

  case 1 {
    1 -> True
    _ -> False
  }

  case 1 {
    1 -> Nil
    _ -> Nil
  }

  classify(1) + classify(9) + fallback_first(1) + duplicate_literal(1)
}

// @geam:expect Int(157)
