fn fallback_first(value: String) {
  case value {
    _ -> 7
    "one" -> 99
  }
}

fn duplicate_literal(value: String) {
  case value {
    "one" -> 100
    "one" -> 200
    _ -> 0
  }
}

pub fn main() {
  fallback_first("one") + duplicate_literal("one")
}

// geam:expect Int(107)
