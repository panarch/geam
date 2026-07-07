pub fn main() {
  let bool_ok = case True {
    value if value -> True
    _ -> False
  }

  let bool_literal_ok = case False {
    False if True -> True
    _ -> False
  }

  let bool_true_literal_ok = case True {
    True if True -> True
    _ -> False
  }

  let string_ok = case "go" {
    value if value == "go" -> True
    _ -> False
  }

  let string_literal_ok = case "go" {
    "go" if True -> True
    _ -> False
  }

  let float_ok = case 1.5 {
    value if value >. 1.0 -> True
    _ -> False
  }

  let float_literal_ok = case 1.5 {
    1.5 if True -> True
    _ -> False
  }

  bool_ok && bool_literal_ok && bool_true_literal_ok && string_ok && string_literal_ok && float_ok && float_literal_ok
}

// geam:expect Bool(true)
