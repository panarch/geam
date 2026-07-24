pub fn main() {
  let value = 2
  let int_guard = case value {
    other as alias if other + alias == 4 -> alias
    _ -> 0
  }

  let bool_guard = case True {
    value as alias if value && alias -> alias
    _ -> False
  }

  let string_guard = case "one" {
    value as alias if value == alias -> alias
    _ -> ""
  }

  let float_guard = case 1.5 {
    value as alias if value == alias -> value +. alias
    _ -> 0.0
  }

  int_guard == 2 && bool_guard && string_guard == "one" && float_guard == 3.0
}

// @geam:expect Bool(true)
