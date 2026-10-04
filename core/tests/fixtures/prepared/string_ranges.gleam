pub fn count(text: String, total: Int) -> Int {
  case text {
    "λ" <> rest -> count(rest, total + 1)
    "" -> total
    _ -> panic as "expected lambda prefix"
  }
}

pub fn select(text: String) -> Int {
  case text {
    "red" -> 7
    "blue" -> 9
    "\n\"\\λ" -> 11
    _ -> -1
  }
}

pub fn aliases(left: String, right: String, flag: Bool, total: Int) -> Int {
  case left {
    "λ" as prefix <> rest as whole if rest == right && flag -> {
      case whole == left && prefix == "λ" {
        True -> total + 2
        False -> -100
      }
    }
    "λ" <> rest if rest == right -> total + 4
    "λ" <> _ -> total + 8
    _ -> total + 16
  }
}

pub fn alternate(left: String, right: String, total: Int) -> Int {
  case left {
    "λ" <> rest | "m" <> rest -> alternate(right, rest, total + 1)
    "" ->
      case right {
        "" -> total
        _ -> alternate(right, left, total)
      }
    _ -> total
  }
}

pub fn same(left: String, right: String, expected: Bool) -> Bool {
  let equal = left == right
  let different = left != right
  case equal == expected {
    True -> different != expected
    False -> False
  }
}

pub fn empty_prefix(text: String) -> Bool {
  case text {
    "" as prefix <> rest as whole ->
      case prefix == "" {
        True -> rest == whole
        False -> False
      }
    _ -> False
  }
}

pub fn literal_only() -> Int {
  case "λtail" {
    "λ" <> rest if rest == "tail" -> 7
    _ -> -1
  }
}

pub fn asserted(text: String, total: Int) -> Int {
  let assert "λ" as prefix <> rest as whole = text as "lambda required"
  case rest != whole {
    True ->
      case prefix == "λ" {
        True -> total + 1
        False -> -1
      }
    False -> -1
  }
}

pub fn caller(text: String, total: Int) -> #(String, Int, List(Int), Bool) {
  let result = count(text, total)
  #(text, result, [3, 5], same(text, text, True))
}

pub fn spin(text: String) -> Int {
  case text {
    "λ" <> _ -> spin(text)
    _ -> 0
  }
}

pub fn running(text: String) -> Int {
  echo "entered-string"
  spin(text)
}

pub fn unsupported(text: String) -> Int {
  let joined = text <> "tail"
  case joined {
    "tail" -> 1
    _ -> 2
  }
}

pub fn main() -> Int {
  count("λλλ", 4)
}

pub fn assert_literal(text: String) -> Int {
  let assert "\n\"\\λ" as whole = text as "literal required"
  case whole {
    "\n\"\\λ" -> 17
    _ -> -1
  }
}

pub fn assert_prefix(text: String) -> Int {
  let assert "λ" <> _ = text as "prefix required"
  19
}

pub fn assert_suffix(text: String, total: Int) -> Int {
  let assert "λ" as unused <> rest = text as "suffix required"
  case total != -1 {
    True ->
      case rest {
        "tail" -> total + 23
        _ -> total + 29
      }
    False -> -1
  }
}

pub fn bits_with_boolean_guard(
  bytes: BitArray,
  left: Bool,
  right: Bool,
) -> Int {
  let assert <<head:8, _:bits>> = bytes
  case left == right {
    True -> head
    False -> 0
  }
}
