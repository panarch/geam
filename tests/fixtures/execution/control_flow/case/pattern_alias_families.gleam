pub fn main() {
  let bool_literal = case True {
    True as alias -> alias
    False -> False
  }

  let bool_variable = case True {
    value as alias -> value && alias
  }

  let string_variable = case "one" {
    value as alias -> value <> alias
  }

  let string_literal = case "one" {
    "one" as alias -> alias
    _ -> ""
  }

  let float_literal = case 1.5 {
    1.5 as alias -> alias +. 0.5
    _ -> 0.0
  }

  let float_variable = case 1.5 {
    value as alias -> value +. alias
  }

  bool_literal
  && bool_variable
  && string_variable == "oneone"
  && string_literal == "one"
  && float_literal == 2.0
  && float_variable == 3.0
}

// geam:expect Bool(true)
