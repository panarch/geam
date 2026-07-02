fn classify(value: String) {
  case value {
    "one" -> 10
    "two" -> 20
    _ -> 30
  }
}

pub fn main() {
  classify("one") + classify("many")
}

// geam:expect Int(40)
