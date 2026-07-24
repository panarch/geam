pub fn main() {
  let bool_value = case True {
    other -> other
  }

  let string_value = case "geam" {
    other -> other
  }

  let float_value = case 1.5 {
    other -> other
  }

  bool_value == True && string_value == "geam" && float_value == 1.5
}

// @geam:expect Bool(true)
