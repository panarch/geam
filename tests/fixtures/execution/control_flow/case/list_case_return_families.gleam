pub fn main() {
  case 1 {
    1 -> ["one"]
    _ -> ["many"]
  }
}

pub fn string_case(value: String) {
  case value {
    "one" -> [1]
    _ -> [0]
  }
}

pub fn bool_case(value: Bool) {
  case value {
    True -> [1.0]
    False -> [0.0]
  }
}

pub fn float_case(value: Float) {
  case value {
    1.0 -> [True]
    _ -> [False]
  }
}

// geam:expect List(String)([String("one")])
